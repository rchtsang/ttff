//! platform.rs
//! 
//! loading platform descriptions from yaml files
use std::fs;
use std::path::Path;
use std::sync::Arc;

use thiserror::Error;
use yaml_rust2::{Yaml, YamlLoader, ScanError};
use flagset::FlagSet;

use fugue_bytes::Endian;
use fugue_core::prelude::*;
use fugue_core::language::*;

use crate::backend::armv7m::SysCtrlConfig;
use crate::backend::{self, armv7m, Backend};
use crate::types::Permission;
use crate::utils::*;


#[derive(Error, Debug)]
pub enum Error {
    #[error("no yaml documents parsed")]
    NoDocs,
    #[error("multiple yaml documents parsed: {0:?} documents")]
    MultipleDocs(usize),
    #[error("invalid field value: {0}")]
    InvalidField(&'static str),
    #[error(transparent)]
    Yaml(#[from] ScanError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    LangBuilder(Arc<LanguageBuilderError>),
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error(transparent)]
    Backend(#[from] backend::Error),
}


/// a memory region
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Region {
    pub name: String,
    pub address: Address,
    pub size: usize,
    pub perms: FlagSet<Permission>,
    pub description: String,
}

/// hardware platform metadata
#[derive(Clone, Debug)]
pub struct Platform {
    pub(crate) name: String,
    pub(crate) cpu_name: String,
    pub(crate) cpu_revision: String,
    pub(crate) cpu_endian: Endian,
    pub(crate) mpu_present: bool,
    pub(crate) fpu_present: bool,
    pub(crate) nvic_prio_bits: u8,
    pub(crate) vendor_systick_config: bool,
    pub(crate) mem: Vec<Region>,
    pub(crate) mmio: Vec<Region>,
}

impl Platform {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let contents = fs::read_to_string(path)?;
        let mut docs = YamlLoader::load_from_str(&contents)?;
        if docs.len() == 0 {
            return Err(Error::NoDocs);
        } else if docs.len() > 1 {
            return Err(Error::MultipleDocs(docs.len()));
        }
        let yaml = docs.swap_remove(0);
        Platform::from_yaml(yaml)
    }

    #[instrument(skip_all)]
    pub fn from_yaml(yaml: Yaml) -> Result<Self, Error> {
        let name = yaml["name"].as_str()
            .ok_or(Error::InvalidField("name"))?.into();
        let cpu_name = yaml["cpu"]["name"].as_str()
            .ok_or(Error::InvalidField("cpu_name"))?.into();
        let cpu_revision = yaml["cpu"]["revision"].as_str()
            .ok_or(Error::InvalidField("cpu_revision"))?.into();
        let cpu_endian = match yaml["cpu"]["endian"].as_str() {
            Some("little") => { Endian::Little },
            Some("big") => { Endian::Big },
            _ => { return Err(Error::InvalidField("cpu_endian")) }
        };
        let mpu_present = match &yaml["cpu"]["mpuPresent"] {
            Yaml::Integer(val) => { !(*val == 0) }
            Yaml::Boolean(val) => { *val }
            variant => {
                error!("invalid field {:?}", variant);
                return Err(Error::InvalidField("mpu_present"));
            }
        };
        let fpu_present = match &yaml["cpu"]["fpuPresent"] {
            Yaml::Integer(val) => { !(*val == 0) }
            Yaml::Boolean(val) => { *val }
            variant => {
                error!("invalid field {:?}", variant);
                return Err(Error::InvalidField("fpu_present"));
            }
        };
        let nvic_prio_bits = yaml["cpu"]["nvicPrioBits"].as_i64()
            .ok_or(Error::InvalidField("nvic_prio_bits"))? as u8;
        let vendor_systick_config = match &yaml["cpu"]["vendorSystickConfig"] {
            Yaml::Integer(val) => { !(*val == 0) }
            Yaml::Boolean(val) => { *val }
            variant => {
                error!("invalid field {:?}", variant);
                return Err(Error::InvalidField("vendor_sytick_config"));
            }
        };

        let mem_regions = yaml["mem"].as_hash()
            .ok_or(Error::InvalidField("mem"))?;
        let mut mem = vec![];
        for (name, data) in mem_regions {
            let name = name.as_str()
                .ok_or(Error::InvalidField("mem region name"))?;
            let region = Region::new_with(name, data)?;
            mem.push(region);
        }
        mem.sort();

        let mmio_regions = yaml["mmio"].as_hash()
            .ok_or(Error::InvalidField("mmio"))?;
        let mut mmio = vec![];
        for (name, data) in mmio_regions {
            let name = name.as_str()
                .ok_or(Error::InvalidField("mmio region name"))?;
            let region = Region::new_with(name, data)?;
            mmio.push(region);
        }
        mmio.sort();

        Ok(Self {
            name,
            cpu_name,
            cpu_revision,
            cpu_endian,
            mpu_present,
            fpu_present,
            nvic_prio_bits,
            vendor_systick_config,
            mem,
            mmio,
        })
        
    }

    pub fn lang(&self, builder: &LanguageBuilder) -> Result<Language, Error> {
        match self.cpu_name.as_str() {
            "CM3" | "CM4" if self.cpu_endian.is_little() => {
                builder.build("ARM:LE:32:Cortex", "default")
                    .map_err(Error::from)
            }
            _ => { Err(Error::Unsupported(format!("cpu: {}", self.cpu_name))) }
        }
    }

    pub fn backend(&self, builder: &LanguageBuilder) -> Result<impl Backend, Error> {
        match self.cpu_name.as_str() {
            "CM3" | "CM4" if self.cpu_endian.is_little() => {
                let scs_config = self._generate_scs_config();
                let mut backend = armv7m::Backend::new_with(builder, scs_config)?;
                for Region {
                    name: _,
                    address,
                    size,
                    perms: _,
                    description: _,
                } in self.mem.iter() {
                    backend.map_mem(address, *size)?;
                }
                Ok(backend)
            }
            _ => { panic!("unsupported cpu: {}", self.cpu_name) }
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cpu_name(&self) -> &str {
        &self.cpu_name
    }

    pub fn cpu_revision(&self) -> &str {
        &self.cpu_revision
    }

    pub fn cpu_endian(&self) -> Endian {
        self.cpu_endian
    }

    pub fn mpu_present(&self) -> bool {
        self.mpu_present
    }

    pub fn fpu_present(&self) -> bool {
        self.fpu_present
    }

    pub fn nvic_prio_bits(&self) -> u8 {
        self.nvic_prio_bits
    }

    pub fn vendor_systick_config(&self) -> bool {
        self.vendor_systick_config
    }

    pub fn mem(&self) -> &[Region] {
        &self.mem[..]
    }

    pub fn mmio(&self) -> &[Region] {
        &self.mmio[..]
    }
}

impl Platform {
    fn _generate_scs_config(&self) -> Option<SysCtrlConfig> {
        // TODO
        None
    }
}



impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.address, self.size).partial_cmp(&(other.address, other.size))
    }
}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Region {
    #[instrument(skip_all)]
    pub fn new_with(name: &str, yaml: &Yaml) -> Result<Self, Error> {
        // debug!("{name}: {yaml:?}");
        let name = name.into();
        let description = yaml["description"].as_str()
            .unwrap_or("").into();
        let address = yaml["address"].as_i64()
            .map(|val| Address::from(val as u64))
            .ok_or(Error::InvalidField("region address"))?;
        let size = yaml["size"].as_i64()
            .ok_or(Error::InvalidField("region size"))? as usize;
        let perms = match &yaml["perms"] {
            Yaml::Integer(val) => { *val as u8 }
            Yaml::String(val) => {
                str_to_uint(val).map_err(|_| {
                    Error::InvalidField("region perms")
                })? as u8
            }
            _ => { return Err(Error::InvalidField("region perms")) }
        };
        let perms = FlagSet::new(perms as u8)
            .map_err(|_| Error::InvalidField("region perms"))?;
        Ok(Self { name, address, size, perms, description })
    }
}

impl From<LanguageBuilderError> for Error {
    fn from(err: LanguageBuilderError) -> Self {
        Self::LangBuilder(Arc::new(err))
    }
}

impl From<Arc<LanguageBuilderError>> for Error {
    fn from(err: Arc<LanguageBuilderError>) -> Self {
        Self::LangBuilder(err)
    }
}