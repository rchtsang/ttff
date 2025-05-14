//! program.rs
//! 
//! loading firmware binaries and storing metadata

use ahash::AHashMap;
use thiserror::Error;
use bumpalo::{
    self,
    Bump,
    collections::{
        String as BumpString,
        Vec as BumpVec,
    },
};
use elf::{
    self,
    ElfBytes,
    endian::AnyEndian,
    symbol::Symbol as ElfSymbol,
    string_table::StringTable,
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
    sh_type: u32,
    sh_flags: u64,
}

impl<'bump> std::fmt::Debug for Section<'bump> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<20} {:<14} {:#010x} {:#08x} {}",
            self.name.as_str(),
            elfdefs::sh_type_str(self.sh_type),
            self.address.offset(),
            self.size,
            elfdefs::sh_flag_str(self.sh_flags).as_str(),
        )
    }
}

impl<'bump> Section<'bump> {
    pub fn name(&self) -> &str { self.name.as_str() }
    pub fn address(&self) -> Address { self.address.clone() }
    pub fn size(&self) -> usize { self.size }
    pub fn sh_type(&self) -> u32 { self.sh_type }
    pub fn sh_type_str(&self) -> &str { elfdefs::sh_type_str(self.sh_type) }
    pub fn sh_flags(&self) -> u64 { self.sh_flags }
}

#[derive(Debug)]
pub struct Segment<'bump> {
    /// elf segment type
    p_type: u32,
    /// segment offset into elf file
    p_offset: u64,
    /// virtual address
    p_vaddr: Address,
    /// physical address
    p_paddr: Address,
    /// size in elf file
    p_filesz: usize,
    /// size in memory
    p_memsz: usize,
    /// segment flags
    p_flags: u32,
    /// segment alignment
    p_align: u64,
    /// bytes data
    data: BumpVec<'bump, u8>,
}

impl<'bump> Segment<'bump> {
    pub fn p_type_str(&self)      -> &str { elfdefs::p_type_str(self.p_type) }
    pub fn p_type(&self)    -> u32 { self.p_type }
    pub fn p_offset(&self)  -> u64 { self.p_offset }
    pub fn p_vaddr(&self)   -> Address { self.p_vaddr.clone() }
    pub fn p_paddr(&self)   -> Address { self.p_paddr.clone() }
    pub fn p_filesz(&self)  -> usize { self.p_filesz }
    pub fn p_memsz(&self)   -> usize { self.p_memsz }
    pub fn p_flags(&self)   -> u32 { self.p_flags }
    pub fn p_align(&self)   -> u64 { self.p_align }
    pub fn data(&self)      -> &[u8] { self.data.as_slice() }
}

/// an elf symbol
#[derive(Debug)]
pub struct Symbol<'bump> {
    pub name: &'bump str,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

impl<'bump> Symbol<'bump> {
    pub fn new_in<'data>(
        bump: &'bump Bump,
        sym: ElfSymbol,
        strtab: StringTable<'data>,
    ) -> Self {
        let name = strtab.get(sym.st_name as usize)
            .expect("could not find name in symtab");
        let name = BumpString::from_str_in(name, bump)
            .into_bump_str();
        let st_shndx = sym.st_shndx;
        let st_value = sym.st_value;
        let st_size = sym.st_size;

        Self { name, st_shndx, st_value, st_size }
    }
}

/// a wrapper for the program binary
#[derive(Default, Debug)]
pub struct Program<'bump> {
    sections: Vec<Section<'bump>>,
    segments: Vec<Segment<'bump>>,
    symtab: AHashMap<&'bump str, Symbol<'bump>>,
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
                Section {
                    name: bumpalo::format!(in bump, "{}", name),
                    address: shdr.sh_addr.into(),
                    size: shdr.sh_size as usize,
                    sh_type: shdr.sh_type,
                    sh_flags: shdr.sh_flags,
                }
            }).collect();
        
        let segtab = elf.segments()
            .ok_or(Error::Elf("expected segment table"))?;
        let segments = segtab.iter()
            .map(|phdr| {
                Segment {
                    p_type: phdr.p_type,
                    p_offset: phdr.p_offset,
                    p_vaddr: Address::from(phdr.p_vaddr),
                    p_paddr: Address::from(phdr.p_paddr),
                    p_filesz: phdr.p_filesz as usize,
                    p_memsz: phdr.p_memsz as usize,
                    p_flags: phdr.p_flags,
                    p_align: phdr.p_align,
                    data: BumpVec::from_iter_in(
                        elf.segment_data(&phdr)
                            .unwrap_or(&[])
                            .iter()
                            .cloned(),
                        bump),
                }
            }).collect();


        let (symtab, strtab) = elf.symbol_table()?
            .ok_or(Error::Elf("expected a symbol table"))?;
        let symtab: AHashMap<_, _> = symtab.iter()
            .map(|sym| {
                let symbol = Symbol::new_in(bump, sym, strtab);
                (symbol.name, symbol)
            }).collect();

        Ok(Self { sections, segments, symtab })
    }

    pub fn new_from_bytes<'data>(
        bump: &'bump Bump,
        base: impl Into<Address>,
        bytes: &[u8],
    ) -> Result<Self, Error> {
        let base = base.into();
        let sections = vec![
            Section {
                name: BumpString::from_str_in("text", bump),
                address: base.clone(),
                size: bytes.len(),
                sh_type: elfdefs::SHT_PROGBITS,
                sh_flags: elfdefs::SHF_ALLOC | elfdefs::SHF_EXECINSTR,
                // data: BumpVec::from_iter_in(bytes.iter().cloned(), bump),
            }
        ];
        let segments = vec![
            Segment {
                p_type: elfdefs::PT_LOAD,
                p_offset: 0,
                p_vaddr: base.clone(),
                p_paddr: base.clone(),
                p_filesz: bytes.len(),
                p_memsz: bytes.len(),
                p_flags: elfdefs::PF_R | elfdefs::PF_X,
                p_align: 0x1000,
                data: BumpVec::from_iter_in(
                    bytes.iter().cloned(), bump),
            }
        ];
        let symtab = AHashMap::default();
        Ok(Self { sections, segments, symtab })
    }

    pub fn sections(&self) -> &[Section<'bump>] {
        &self.sections
    }

    pub fn symtab(&self) -> &AHashMap<&'bump str, Symbol<'bump>> {  
        &self.symtab
    }

    /// get an iterator over the loadable segments of the binary,
    /// i.e. any segment with PT_LOAD
    /// (not sure if we need to do anything with EXIDX)
    pub fn loadable_segments(&self) -> impl Iterator<Item=&Segment> + use<'_> {
        self.segments.iter()
            .filter(|&seg| seg.p_type & elfdefs::PT_LOAD != 0)

        // note: exidx segment appears to be used for unwinding tables, but
        // i am not certain if it needs to be explicitly loaded...
        // https://sushihangover.github.io/llvm-and-the-arm-elf-arm-dot-exidx-star-section/
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

    pub fn sh_type_str(sh_type: u32) -> &'static str {
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

    pub fn sh_flag_str(sh_flags: u64) -> String {
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

    pub fn sh_flag_write(sh_flags: u64) -> bool {
        sh_flags & SHF_WRITE != 0
    }

    pub fn sh_flag_alloc(sh_flags: u64) -> bool {
        sh_flags & SHF_ALLOC != 0
    }

    pub fn sh_flag_exec(sh_flags: u64) -> bool {
        sh_flags & SHF_EXECINSTR != 0
    }

    // elf segment type definitions
    pub const PT_NULL       : u32 = 0;
    pub const PT_LOAD       : u32 = 1;
    pub const PT_DYNAMIC    : u32 = 2;
    pub const PT_INTERP     : u32 = 3;
    pub const PT_NOTE       : u32 = 4;
    pub const PT_SHLIB      : u32 = 5;
    pub const PT_PHDR       : u32 = 6;
    pub const PT_TLS        : u32 = 7;
    pub const PT_LOOS       : u32 = 0x60000000;
    pub const PT_HIOS       : u32 = 0x6fffffff;
    pub const PT_LOPROC     : u32 = 0x70000000;
    pub const PT_HIPROC     : u32 = 0x7fffffff;

    // elf segment flag definitions
    pub const PF_X          : u32 = 0x1;
    pub const PF_W          : u32 = 0x2;
    pub const PF_R          : u32 = 0x4;
    pub const PF_MASKOS     : u32 = 0x0ff00000;
    pub const PF_MASKPROC   : u32 = 0xf0000000;



    pub fn p_type_str(p_type: u32) -> &'static str {
        match p_type {
            PT_NULL     => { "NULL" }
            PT_LOAD     => { "LOAD" }
            PT_DYNAMIC  => { "DYNAMIC" }
            PT_INTERP   => { "INTERP" }
            PT_NOTE     => { "NOTE" }
            PT_SHLIB    => { "SHLIB" }
            PT_PHDR     => { "PHDR" }
            PT_TLS      => { "TLS" }
            PT_LOOS     => { "LOOS" }
            PT_HIOS     => { "HIOS" }
            PT_LOPROC   => { "LOPROC" }
            PT_HIPROC   => { "HIPROC" }
            _ => { "UNKNOWN" }
        }
    }

    pub fn p_flags_str(p_flags: u32) -> String {
        let mut result = String::new();
        // if pf_flags & PF_MASKPROC != 0  {
        //     result.push_str("U");
        // }
        // if pf_flags & PF_MASKOS != 0    {
        //     result.push_str("U");
        // }
        if p_flags & PF_R != 0 {
            result.push_str("R");
        }
        if p_flags & PF_W != 0 {
            result.push_str("W");
        }
        if p_flags & PF_X != 0 {
            result.push_str("X");
        }
        result
    }
}