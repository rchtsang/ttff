//! program.rs
//! 
//! loading firmware binaries and storing metadata

use elf::{
    self,
    ElfBytes,
    endian::AnyEndian,
};
use thiserror::Error;
use bumpalo::{
    self, collections::{
        String as BumpString,
        Vec as BumpVec,
    }, Bump
};

use fugue_core::prelude::*;


#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseElf(#[from] elf::ParseError),
    #[error("elf error {0}")]
    Elf(&'static str),
}

/// a firmware section (based on elf section)
pub struct Section<'bump> {
    name: BumpString<'bump>,
    address: Address,
    size: usize,
    data: BumpVec<'bump, u8>,
    sh_type: u32,
    sh_flags: u64,
}

impl<'bump> std::fmt::Debug for Section<'bump> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<20} {:<14} {:#010x} {:#08x} {}",
            self.name.as_str(),
            elfdefs::type_str(self.sh_type),
            self.address.offset(),
            self.size,
            elfdefs::flag_str(self.sh_flags).as_str(),
        )
    }
}

impl<'bump> Section<'bump> {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn address(&self) -> Address {
        self.address.clone()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn sh_type(&self) -> u32 {
        self.sh_type
    }

    pub fn sh_type_str(&self) -> &'static str {
        elfdefs::type_str(self.sh_type)
    }

    pub fn sh_flags(&self) -> u64 {
        self.sh_flags
    }

    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
}

/// a wrapper for the program binary
#[derive(Default, Debug)]
pub struct Program<'bump> {
    sections: Vec<Section<'bump>>,
}

impl<'bump> Program<'bump> {
    pub fn new_from_elf<'data>(
        bump: &'bump Bump,
        elf: ElfBytes<'data, AnyEndian>,
    ) -> Result<Self, Error> {
        let (shdrs_opt, strtab_opt) = elf.section_headers_with_strtab()?;
        let (shdrs, strtab) = (
            shdrs_opt.ok_or(Error::Elf("expected section headers"))?,
            strtab_opt.ok_or(Error::Elf("expected strings table"))?,
        );
        let sections = shdrs.iter()
            .map(|shdr| {
                let name = strtab.get(shdr.sh_name as usize)
                    .unwrap_or("");
                let (data, _) = elf.section_data(&shdr)
                    .unwrap_or((&[], None));
                let data = BumpVec::from_iter_in(
                    data.into_iter().cloned(), 
                    bump);
                Section {
                    name: bumpalo::format!(in bump, "{}", name),
                    address: shdr.sh_addr.into(),
                    size: shdr.sh_size as usize,
                    sh_type: shdr.sh_type,
                    sh_flags: shdr.sh_flags,
                    data,
                }
            }).collect();
        Ok(Self { sections })
    }

    pub fn new_from_bytes<'data>(
        bump: &'bump Bump,
        base: impl Into<Address>,
        bytes: &[u8],
    ) -> Result<Self, Error> {
        let sections = vec![
            Section {
                name: BumpString::from_str_in("text", bump),
                address: base.into(),
                size: bytes.len(),
                sh_type: elfdefs::SHT_PROGBITS,
                sh_flags: elfdefs::SHF_ALLOC | elfdefs::SHF_EXECINSTR,
                data: BumpVec::from_iter_in(bytes.iter().cloned(), bump),
            }
        ];
        Ok(Self { sections })
    }

    pub fn sections(&self) -> &[Section<'bump>] {
        &self.sections
    }

    /// get an iterator over the loadable sections of the binary,
    /// i.e. any section with the SHF_ALLOC flag set
    pub fn loadable_sections(&self) -> impl Iterator<Item=&Section> + use<'_> {
        self.sections.iter()
            .filter(|&sec| elfdefs::flag_alloc(sec.sh_flags))
    }
}



/// elf definitions
pub mod elfdefs {
    // elf section type definitions
    pub const SHT_NULL          : u32 = 0;
    pub const SHT_PROGBITS      : u32 = 1;
    pub const SHT_SYMTAB        : u32 = 2;
    pub const SHT_STRTAB        : u32 = 3;
    pub const SHT_RELA          : u32 = 4;
    pub const SHT_HASH          : u32 = 5;
    pub const SHT_DYNAMIC       : u32 = 6;
    pub const SHT_NOTE          : u32 = 7;
    pub const SHT_NOBITS        : u32 = 8;
    pub const SHT_REL           : u32 = 9;
    pub const SHT_SHLIB         : u32 = 10;
    pub const SHT_DYNSYM        : u32 = 11;
    pub const SHT_INIT_ARRAY    : u32 = 14;
    pub const SHT_FINI_ARRAY    : u32 = 15;
    pub const SHT_PREINIT_ARRAY : u32 = 16;
    pub const SHT_GROUP         : u32 = 17;
    pub const SHT_SYMTAB_SHNDX  : u32 = 18;
    pub const SHT_LOOS          : u32 = 0x60000000;
    pub const SHT_HIOS          : u32 = 0x6fffffff;
    pub const SHT_LOPROC        : u32 = 0x70000000;
    pub const SHT_HIPROC        : u32 = 0x7fffffff;
    pub const SHT_LOUSER        : u32 = 0x80000000;
    pub const SHT_HIUSER        : u32 = 0xffffffff;

    // elf section flag definitions
    pub const SHF_WRITE         : u64 = 0x1;
    pub const SHF_ALLOC         : u64 = 0x2;
    pub const SHF_EXECINSTR     : u64 = 0x4;
    pub const SHF_MERGE         : u64 = 0x10;
    pub const SHF_STRINGS       : u64 = 0x20;
    pub const SHF_INFO_LINK     : u64 = 0x40;
    pub const SHF_LINK_ORDER    : u64 = 0x80;
    pub const SHF_OS_NONCONFORM : u64 = 0x100;
    pub const SHF_GROUP         : u64 = 0x200;
    pub const SHF_TLS           : u64 = 0x400;
    pub const SHF_MASKOS        : u64 = 0x0ff00000;
    pub const SHF_MASKPROC      : u64 = 0xf0000000;

    pub fn type_str(sh_type: u32) -> &'static str {
        match sh_type {
            SHT_NULL            => { "NULL" }
            SHT_PROGBITS        => { "PROGBITS" }
            SHT_SYMTAB          => { "SYMTAB" }
            SHT_STRTAB          => { "STRTAB" }
            SHT_RELA            => { "RELA" }
            SHT_HASH            => { "HASH" }
            SHT_DYNAMIC         => { "DYNAMIC" }
            SHT_NOTE            => { "NOTE" }
            SHT_NOBITS          => { "NOBITS" }
            SHT_REL             => { "REL" }
            SHT_SHLIB           => { "SHLIB" }
            SHT_DYNSYM          => { "DYNSYM" }
            SHT_INIT_ARRAY      => { "INIT_ARRAY" }
            SHT_FINI_ARRAY      => { "FINI_ARRAY" }
            SHT_PREINIT_ARRAY   => { "PREINIT_ARRAY" }
            SHT_GROUP           => { "GROUP" }
            SHT_SYMTAB_SHNDX    => { "SYMTAB_SHNDX" }
            SHT_LOOS            => { "LOOS" }
            SHT_HIOS            => { "HIOS" }
            SHT_LOPROC          => { "LOPROC" }
            SHT_HIPROC          => { "HIPROC" }
            SHT_LOUSER          => { "LOUSER" }
            SHT_HIUSER          => { "HIUSER" }
            _ => { "UNKNOWN" }
        }
    }

    pub fn flag_str(sh_flags: u64) -> String {
        let mut result = String::new();
        if sh_flags & SHF_WRITE != 0 {
            result.push_str("W");
        }
        if sh_flags & SHF_ALLOC != 0 {
            result.push_str("A");
        }
        if sh_flags & SHF_EXECINSTR != 0 {
            result.push_str("X");
        }
        if sh_flags & SHF_MERGE != 0 {
            result.push_str("M");
        }
        if sh_flags & SHF_STRINGS != 0 {
            result.push_str("S");
        }
        if sh_flags & SHF_INFO_LINK != 0 {
            result.push_str("I");
        }
        if sh_flags & SHF_LINK_ORDER != 0 {
            result.push_str("L");
        }
        if sh_flags & SHF_OS_NONCONFORM != 0 {
            result.push_str("O");
        }
        if sh_flags & SHF_GROUP != 0 {
            result.push_str("G");
        }
        if sh_flags & SHF_TLS != 0 {
            result.push_str("T");
        }
        if sh_flags & SHF_MASKOS != 0 {
            result.push_str("o");
        }
        if sh_flags & SHF_MASKPROC != 0 {
            result.push_str("p");
        }
        result
    }

    pub fn flag_write(sh_flags: u64) -> bool {
        sh_flags & SHF_WRITE != 0
    }

    pub fn flag_alloc(sh_flags: u64) -> bool {
        sh_flags & SHF_ALLOC != 0
    }

    pub fn flag_exec(sh_flags: u64) -> bool {
        sh_flags & SHF_EXECINSTR != 0
    }
}