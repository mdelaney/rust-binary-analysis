#![allow(overlapping_patterns)]

use std::fmt;

#[derive(Debug)]
pub enum SectionType {
    Null,                // 0x00 Section header table entry unused
    ProgBits,            // 0x01 Program data
    SymTab,              // 0x02 Symbol table
    StrTab,              // 0x03 String table
    Rela,                // 0x04 Relocation entries with addends
    Hash,                // 0x05 Symbol hash table
    Dynamic,             // 0x06 Dynamic linking information
    Note,                // 0x07 Notes
    NoBits,              // 0x08 Program space with no data (bss)
    RelocationEnt,       // 0x09 Relocation entries, no addends
    ShLib,               // 0x0a Reserved
    DynSym,              // 0x0b Dynamic linker symbol table
    InitArray,           // 0x0e Array of constructors
    FiniArray,           // 0x0f Array of destructors
    PreinitArray,        // 0x10 Array of pre-constructors
    Group,               // 0x11 Section group
    SymTabShNdx,         // 0x12 Extended section indices  SYMTAB_SHNDX
    Num,                 // 0x13 Number of defined types
    OSSpecific,          // 0x60000000-0x6fffffff Start OS-specific
    GNUAttributes,       // 0x6ffffff5 Object attributes.
    GNUHash,             // 0x6ffffff6 GNU-style hash table.
    GNULibList,          // 0x6ffffff7 Prelink library list
    Checksum,            // 0x6ffffff8 Checksum for DSO content.
    SunWMove,            // 0x6ffffffa
    SunWCOMDAT,          // 0x6ffffffb
    SunWSyminfo,         // 0x6ffffffc
    GNUVersionDef,       // 0x6ffffffd Version definition section.
    GNUVersionNeeds,     // 0x6ffffffe Version needs section.
    GNUVersionSymTbl,    // 0x6fffffff Version symbol table.
    ProcessorSpecific,   // 0x70000000-0x7fffffff processor-specific
    ApplicationSpecific, // 0x80000000-0x8fffffff application-specific

    Unknown,
}
impl SectionType {
    pub fn from_u32(value: u32) -> SectionType {
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
            0x6fff_fff5 => SectionType::GNUAttributes,
            0x6fff_fff6 => SectionType::GNUHash,
            0x6fff_fff7 => SectionType::GNULibList,
            0x6fff_fff8 => SectionType::Checksum,
            0x6fff_fffa => SectionType::SunWMove,
            0x6fff_fffb => SectionType::SunWCOMDAT,
            0x6fff_fffc => SectionType::SunWSyminfo,
            0x6fff_fffd => SectionType::GNUVersionDef,
            0x6fff_fffe => SectionType::GNUVersionNeeds,
            0x6fff_ffff => SectionType::GNUVersionSymTbl,
            0x6000_0000..=0x6fff_ffff => SectionType::OSSpecific,
            0x7000_0000..=0x7fff_ffff => SectionType::ProcessorSpecific,
            0x8000_0000..=0x8fff_ffff => SectionType::ApplicationSpecific,
            _ => SectionType::Unknown,
        }
    }
}
impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            SectionType::Null => "Null",
            SectionType::ProgBits => "ProgBits",
            SectionType::SymTab => "SymTab",
            SectionType::StrTab => "StrTab",
            SectionType::Rela => "Rela",
            SectionType::Hash => "Hash",
            SectionType::Dynamic => "Dynamic",
            SectionType::Note => "Note",
            SectionType::NoBits => "NoBits",
            SectionType::RelocationEnt => "Relocation",
            SectionType::ShLib => "ShLib",
            SectionType::DynSym => "DynSym",
            SectionType::InitArray => "InitArray",
            SectionType::FiniArray => "FiniArray",
            SectionType::PreinitArray => "PreinitArray",
            SectionType::Group => "Group",
            SectionType::SymTabShNdx => "SymTabShNdx",
            SectionType::Num => "Num",
            SectionType::GNUAttributes => "GNU Object attributes",
            SectionType::GNUHash => "GNU Hash table",
            SectionType::GNULibList => "GNU Prelink library list",
            SectionType::Checksum => "Checksum for DSO content",
            SectionType::SunWMove => "SunWMove",
            SectionType::SunWCOMDAT => "SunWCOMDAT",
            SectionType::SunWSyminfo => "SunWsyminfo",
            SectionType::GNUVersionDef => "GNU Version definition section",
            SectionType::GNUVersionNeeds => "GNU needs section",
            SectionType::GNUVersionSymTbl => "GNU Version symbol table",
            SectionType::OSSpecific => "OS Specific",
            SectionType::ProcessorSpecific => "Processor-specific",
            SectionType::ApplicationSpecific => "Application-specific",
            SectionType::Unknown => "Unknown",
        };
        write!(f, "{}", value)
    }
}

pub enum SectionFlags {
    Write,           // (1 << 0)  Writable
    Alloc,           // (1 << 1)  Occupies memory during execution
    ExecInstr,       // (1 << 2)  Executable
    Merge,           // (1 << 4)  Might be merged
    Strings,         // (1 << 5)  Contains nul-terminated strings
    InfoLink,        // (1 << 6)  'sh_info' contains SHT index
    LinkOrder,       // (1 << 7)  Preserve order after combining
    OSNonconforming, // (1 << 8)  Non-standard OS specific handling required
    Group,           // (1 << 9)  Section is member of a group
    TLS,             // (1 << 10) Section hold thread-local data
    Compressed,      // (1 << 11) Section with compressed data
    MaskOS,          // 0x0ff00000 OS specific
    MaskProc,        // 0xf0000000 Processor specific
    Ordered,         // (1 << 30) Special ordering requirement (solaris)
    Exclude,         // (1 << 31) Section is excluded unless referenced or allocated (solaris)
}
impl SectionFlags {
    pub fn from_u32(value: u32) -> Vec<SectionFlags> {
        SectionFlags::from_u64(u64::from(value))
    }

    // TODO: this could probably be simpler by simply iterating over a list of flags
    //       and flag values and appending matches
    pub fn from_u64(value: u64) -> Vec<SectionFlags> {
        let mut flags: Vec<SectionFlags> = vec![];
        if value & (1 << 0) != 0 {
            flags.push(SectionFlags::Write);
        }
        if value & (1 << 1) != 0 {
            flags.push(SectionFlags::Alloc);
        }
        if value & (1 << 2) != 0 {
            flags.push(SectionFlags::ExecInstr);
        }
        if value & (1 << 4) != 0 {
            flags.push(SectionFlags::Merge);
        }
        if value & (1 << 5) != 0 {
            flags.push(SectionFlags::Strings);
        }
        if value & (1 << 6) != 0 {
            flags.push(SectionFlags::InfoLink);
        }
        if value & (1 << 7) != 0 {
            flags.push(SectionFlags::LinkOrder);
        }
        if value & (1 << 8) != 0 {
            flags.push(SectionFlags::OSNonconforming);
        }
        if value & (1 << 9) != 0 {
            flags.push(SectionFlags::Group);
        }
        if value & (1 << 10) != 0 {
            flags.push(SectionFlags::TLS);
        }
        if value & (1 << 11) != 0 {
            flags.push(SectionFlags::Compressed);
        }
        if value & 0x0ff0_0000 != 0 {
            flags.push(SectionFlags::MaskOS);
        }
        if value & 0xf000_0000 != 0 {
            flags.push(SectionFlags::MaskProc);
        }
        if value & (1 << 30) != 0 {
            flags.push(SectionFlags::Ordered);
        }
        if value & (1 << 31) != 0 {
            flags.push(SectionFlags::Exclude);
        }
        flags
    }
}
impl fmt::Debug for SectionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            SectionFlags::Write => "W",
            SectionFlags::Alloc => "A",
            SectionFlags::ExecInstr => "X",
            SectionFlags::Merge => "M",
            SectionFlags::Strings => "S",
            SectionFlags::InfoLink => "I",
            SectionFlags::LinkOrder => "L",
            SectionFlags::OSNonconforming => "O",
            SectionFlags::Group => "G",
            SectionFlags::TLS => "T",
            SectionFlags::Compressed => "C",
            SectionFlags::MaskOS => "o",
            SectionFlags::MaskProc => "p",
            SectionFlags::Ordered => "o",
            SectionFlags::Exclude => "E",
        };
        write!(f, "{}", value)
    }
}
