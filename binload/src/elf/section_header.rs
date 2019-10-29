// TODO - remove allow dead code
#![allow(dead_code)]

use std::convert::TryInto;
use std::fmt;
use std::mem;

use super::elf_header::{EI_Data, ELFIdent};

#[allow(dead_code)]
#[derive(Debug)]
pub enum SectionType {
    Null,          // 0x00 Section header table entry unused
    ProgBits,      // 0x01 Program data
    SymTab,        // 0x02 Symbol table
    StrTab,        // 0x03 String table
    Rela,          // 0x04 Relocation entries with addends
    Hash,          // 0x05 Symbol hash table
    Dynamic,       // 0x06 Dynamic linking information
    Note,          // 0x07 Notes
    NoBits,        // 0x08 Program space with no data (bss)
    RelocationEnt, // 0x09 Relocation entries, no addends
    ShLib,         // 0x0a Reserved
    DynSym,        // 0x0b Dynamic linker symbol table
    InitArray,     // 0x0e Array of constructors
    FiniArray,     // 0x0f Array of destructors
    PreinitArray,  // 0x10 Array of pre-constructors
    Group,         // 0x11 Section group
    SymTabShNdx,   // 0x12 Extended section indices  SYMTAB_SHNDX
    Num,           // 0x13 Number of defined types
    LOOS,          // 0x60000000 Start OS-specific
    Unknown,
}
impl SectionType {
    fn from_u32(value: u32) -> SectionType {
        match value {
            0x00 => SectionType::Null,
            0x01 => SectionType::ProgBits,
            0x02 => SectionType::SymTab,
            0x03 => SectionType::StrTab,
            0x04 => SectionType::Rela,
            0x05 => SectionType::Hash,
            0x06 => SectionType::Dynamic,
            0x07 => SectionType::Note,
            0x08 => SectionType::NoBits,
            0x09 => SectionType::RelocationEnt,
            0x0a => SectionType::ShLib,
            0x0b => SectionType::DynSym,
            0x0e => SectionType::InitArray,
            0x0f => SectionType::FiniArray,
            0x10 => SectionType::PreinitArray,
            0x11 => SectionType::Group,
            0x12 => SectionType::SymTabShNdx,
            0x13 => SectionType::Num,
            0x60000000 => SectionType::LOOS,
            _ => SectionType::Unknown,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SectionFlags {
    Write,           // 0x01 Writable
    Alloc,           // 0x02 Occupies memory during execution
    ExecInstr,       // 0x04 Executable
    Merge,           // 0x10 Might be merged
    Strings,         // 0x20 Contains nul-terminated strings
    InfoLink,        // 0x40 'sh_info' contains SHT index
    LinkOrder,       // 0x80 Preserve order after combining
    OSNonconforming, // 0x100 Non-standard OS specific handling required
    Group,           // 0x200 Section is member of a group
    TLS,             // 0x400 Section hold thread-local data
    MaskOS,          // 0x0ff00000 OS specific
    MaskProc,        // 0xf0000000 Processor specific
    Ordered,         // 0x40000000 Special ordering requirement (solaris)
    Exclude,         // 0x80000000 Section is excluded unless referenced or allocated (solaris)
    Unknown,         // unknown
}
impl SectionFlags {
    //    fn from_u32(value: u32) -> SECTION_FLAGS {
    //        SECTION_FLAGS::from_u64(u64::from(value))
    //    }
    fn from_u64(value: u64) -> SectionFlags {
        match value {
            0x01 => SectionFlags::Write,
            0x02 => SectionFlags::Alloc,
            0x04 => SectionFlags::ExecInstr,
            0x10 => SectionFlags::Merge,
            0x20 => SectionFlags::Strings,
            0x40 => SectionFlags::InfoLink,
            0x80 => SectionFlags::LinkOrder,
            0x100 => SectionFlags::OSNonconforming,
            0x200 => SectionFlags::Group,
            0x400 => SectionFlags::TLS,
            0x0ff00000 => SectionFlags::MaskOS,
            0xf0000000 => SectionFlags::MaskProc,
            0x40000000 => SectionFlags::Ordered,
            0x80000000 => SectionFlags::Exclude,
            _ => SectionFlags::Unknown,
        }
    }
}

pub struct SectionHeader64 {
    pub name: u32,                 // Offset to string in .shstrtab section
    pub section_type: SectionType, // Type of this header
    pub flags: SectionFlags,       // Attributes of the section u32 or u64
    pub address: u64, // Virtual address of section in memory, for sections that are loaded u32 or u64
    pub offset: u64,  // Offset of the section in file u32 or u64
    pub size: u64,    // Size in bytes of the section u32 or u64
    pub link: u32,    // Section index of an associated section
    pub info: u32,    // Extra info about the section
    pub addralign: u64, // Required alignment of the section (power of 2) u32 or 64
    pub entsize: u64, // Size in bytes of each entry for sections that contain fixed size entries, else 0 u32 or 64
}

impl SectionHeader64 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> SectionHeader64 {
        // First get the bytes for our header
        const SIZE: usize = mem::size_of::<SectionHeader64>();
        let mut raw: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw).unwrap();

        // Now get our conversion functions to read numbers based on endianness
        let u32_from_bytes = match ident.ei_data {
            EI_Data::LittleEndian => u32::from_le_bytes,
            EI_Data::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EI_Data::LittleEndian => u64::from_le_bytes,
            EI_Data::BigEndian => u64::from_be_bytes,
        };

        // Finally we can create our header
        let result = SectionHeader64 {
            name: u32_from_bytes(raw[0..4].try_into().unwrap()),
            section_type: SectionType::from_u32(u32_from_bytes(raw[4..8].try_into().unwrap())),
            flags: SectionFlags::from_u64(u64_from_bytes(raw[8..16].try_into().unwrap())),
            address: u64_from_bytes(raw[16..24].try_into().unwrap()),
            offset: u64_from_bytes(raw[24..32].try_into().unwrap()),
            size: u64_from_bytes(raw[32..40].try_into().unwrap()),
            link: u32_from_bytes(raw[40..44].try_into().unwrap()),
            info: u32_from_bytes(raw[44..48].try_into().unwrap()),
            addralign: u64_from_bytes(raw[48..56].try_into().unwrap()),
            entsize: u64_from_bytes(raw[56..64].try_into().unwrap()),
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement section header formatter
        // writeln!(f, "{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)");
        let strings = [
            format!("{:15}{:?}", "Name Index:", self.name),
            format!("{:15}{:?}", "Type:", self.section_type),
            format!("{:15}{:?}", "Flags:", self.flags),
            format!("{:15}{:?}", "Address:", self.address),
            format!("{:15}{:?} {}", "Offset:", self.offset, "(bytes)"),
            format!("{:15}{:?} {}", "Size:", self.size, "(bytes)"),
            format!("{:15}{:?}", "Link:", self.link),
            format!("{:15}{:?}", "Info:", self.info),
            format!("{:15}{:?}", "Address Align:", self.addralign),
            format!("{:15}{:?} {}", "Entity Size", self.entsize, "(bytes)"),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for SectionHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for SectionHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
