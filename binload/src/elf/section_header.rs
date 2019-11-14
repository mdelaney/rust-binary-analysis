// TODO - remove allow dead code
#![allow(dead_code)]

use std::convert::TryInto;
use std::fmt;

use super::elf_header::{EI_Class, EI_Data, ELFHeader};

#[allow(dead_code)]
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
            0x6ffffff5 => SectionType::GNUAttributes,
            0x6ffffff6 => SectionType::GNUHash,
            0x6ffffff7 => SectionType::GNULibList,
            0x6ffffff8 => SectionType::Checksum,
            0x6ffffffa => SectionType::SunWMove,
            0x6ffffffb => SectionType::SunWCOMDAT,
            0x6ffffffc => SectionType::SunWSyminfo,
            0x6ffffffd => SectionType::GNUVersionDef,
            0x6ffffffe => SectionType::GNUVersionNeeds,
            0x6fffffff => SectionType::GNUVersionSymTbl,
            0x60000000..=0x6fffffff => SectionType::OSSpecific,
            0x70000000..=0x7fffffff => SectionType::ProcessorSpecific,
            0x80000000..=0x8fffffff => SectionType::ApplicationSpecific,
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

#[allow(dead_code)]
//#[derive(Debug)]
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
}
impl SectionFlags {
    fn from_u32(value: u32) -> Vec<SectionFlags> {
        SectionFlags::from_u64(u64::from(value))
    }

    // TODO: this could probably be simpler by simply iterating over a list of flags
    //       and flag values and appending matches
    fn from_u64(value: u64) -> Vec<SectionFlags> {
        let mut flags: Vec<SectionFlags> = vec![];
        if value & 0x01 != 0 {
            flags.push(SectionFlags::Write);
        }
        if value & 0x02 != 0 {
            flags.push(SectionFlags::Alloc);
        }
        if value & 0x04 != 0 {
            flags.push(SectionFlags::ExecInstr);
        }
        if value & 0x10 != 0 {
            flags.push(SectionFlags::Merge);
        }
        if value & 0x20 != 0 {
            flags.push(SectionFlags::Strings);
        }
        if value & 0x40 != 0 {
            flags.push(SectionFlags::InfoLink);
        }
        if value & 0x80 != 0 {
            flags.push(SectionFlags::LinkOrder);
        }
        if value & 0x100 != 0 {
            flags.push(SectionFlags::OSNonconforming);
        }
        if value & 0x200 != 0 {
            flags.push(SectionFlags::Group);
        }
        if value & 0x400 != 0 {
            flags.push(SectionFlags::TLS);
        }
        if value & 0x0ff00000 != 0 {
            flags.push(SectionFlags::MaskOS);
        }
        if value & 0xf0000000 != 0 {
            flags.push(SectionFlags::MaskProc);
        }
        if value & 0x40000000 != 0 {
            flags.push(SectionFlags::Ordered);
        }
        if value & 0x80000000 != 0 {
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
            SectionFlags::MaskOS => "o",
            SectionFlags::MaskProc => "p",
            SectionFlags::Ordered => "o",
            SectionFlags::Exclude => "E",
        };
        write!(f, "{}", value)
    }
}

pub struct SectionHeader {
    pub name: u32,                 // Offset to string in .shstrtab section
    pub section_type: SectionType, // Type of this header
    pub flags: Vec<SectionFlags>,  // Attributes of the section u32 or u64
    pub address: u64, // Virtual address of section in memory, for sections that are loaded u32 or u64
    pub offset: u64,  // Offset of the section in file u32 or u64
    pub size: u64,    // Size in bytes of the section u32 or u64
    pub link: u32,    // Section index of an associated section
    pub info: u32,    // Extra info about the section
    pub addralign: u64, // Required alignment of the section (power of 2) u32 or 64
    pub entsize: u64, // Size in bytes of each entry for sections that contain fixed size entries, else 0 u32 or 64
    pub name_string: String,
}

impl SectionHeader {
    pub fn get_data<'a>(&self, binary: &'a [u8]) -> &'a [u8] {
        let start = self.offset as usize;
        let end = start + self.size as usize;
        &binary[start..end]
    }

    pub fn parse_from_buffer(index: u16, binary: &Vec<u8>, header: &ELFHeader) -> SectionHeader {
        // First get the bytes for our header
        let start_index = header.e_shoff as usize + (index * header.e_shentsize) as usize;
        let end_index = start_index + header.e_shentsize as usize;
        let raw: &[u8] = &binary[start_index..end_index];

        // Now get our conversion functions to read numbers based on endianness
        let u32_from_bytes = match header.ident.ei_data {
            EI_Data::LittleEndian => u32::from_le_bytes,
            EI_Data::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match header.ident.ei_data {
            EI_Data::LittleEndian => u64::from_le_bytes,
            EI_Data::BigEndian => u64::from_be_bytes,
        };

        // Finally we can create our header
        let result = match header.ident.ei_class {
            EI_Class::ELF32 => SectionHeader {
                name: u32_from_bytes(raw[0..4].try_into().unwrap()),
                section_type: SectionType::from_u32(u32_from_bytes(raw[4..8].try_into().unwrap())),
                flags: SectionFlags::from_u64(u64::from(u32_from_bytes(
                    raw[8..12].try_into().unwrap(),
                ))),
                address: u64::from(u32_from_bytes(raw[12..16].try_into().unwrap())),
                offset: u64::from(u32_from_bytes(raw[16..20].try_into().unwrap())),
                size: u64::from(u32_from_bytes(raw[20..24].try_into().unwrap())),
                link: u32_from_bytes(raw[24..28].try_into().unwrap()),
                info: u32_from_bytes(raw[28..32].try_into().unwrap()),
                addralign: u64::from(u32_from_bytes(raw[32..36].try_into().unwrap())),
                entsize: u64::from(u32_from_bytes(raw[36..40].try_into().unwrap())),
                name_string: std::string::String::new(),
            },
            EI_Class::ELF64 => SectionHeader {
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
                name_string: std::string::String::new(),
            },
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:?}", "Name Index:", self.name),
            format!("{:15}{:?}", "Name:", self.name_string),
            format!("{:15}{:?}", "Type:", self.section_type),
            format!("{:15}{:?}", "Flags:", self.flags),
            format!("{:15}0x{:x?}", "Address:", self.address),
            format!("{:15}0x{:x?} {}", "Offset:", self.offset, "(bytes)"),
            format!(
                "{:15}0x{:x?} {:?} {}",
                "Size:", self.size, self.size, "(bytes)"
            ),
            format!("{:15}{:?}", "Link:", self.link),
            format!("{:15}{:?}", "Info:", self.info),
            format!("{:15}{:?}", "Address Align:", self.addralign),
            format!("{:15}{:?} {}", "Entity Size", self.entsize, "(bytes)"),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for SectionHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
