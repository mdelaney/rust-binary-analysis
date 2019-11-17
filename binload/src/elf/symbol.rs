// TODO - remove allow dead code
#![allow(dead_code)]
#![allow(overlapping_patterns)]

use std::convert::TryInto;
use std::fmt;

use super::elf_header::{EI_Class, EI_Data, ELFHeader};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Bind {
    Local,     // 0 - Local symbol
    Global,    // 1 - Global symbol
    Weak,      // 2 - Weak symbol
    Num,       // 3 - Number of defined types
    GNUUnique, // 10 - Unique symbol
    OS,        // 10-12 - OS specific
    Proc,      // 13-15 - Processor specific
    Unknown,
}
impl Bind {
    fn from_u8(value: u8) -> Bind {
        match value {
            0 => Bind::Local,
            1 => Bind::Global,
            2 => Bind::Weak,
            3 => Bind::Num,
            10 => Bind::GNUUnique,
            10..=12 => Bind::OS,
            13..=15 => Bind::Proc,
            _ => Bind::Unknown,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SymbolType {
    NoType,    // 0 - Symbol type is unspecified
    Object,    // 1 - Symbol is a data object
    Function,  // 2 - Symbol is a code object
    Section,   // 3 - Symbol associated with a section
    File,      // 4 - Symbol's name is file name
    Common,    // 5 - Symbol is a common data object
    TLS,       // 6 - Symbol is thread-local data object
    Num,       // 7 - Number of defined types
    GNUIFunct, // 10 - Symbol is indirect code object
    OS,        // 10-12 - OS specific
    Proc,      // 13-15 - Processor specific
}
impl SymbolType {
    fn from_u8(value: u8) -> SymbolType {
        match value {
            0 => SymbolType::NoType,
            1 => SymbolType::Object,
            2 => SymbolType::Function,
            3 => SymbolType::Section,
            4 => SymbolType::File,
            5 => SymbolType::Common,
            6 => SymbolType::TLS,
            7 => SymbolType::Num,
            10 => SymbolType::GNUIFunct,
            10..=12 => SymbolType::OS,
            13..=15 => SymbolType::Proc,
            _ => SymbolType::NoType,
        }
    }
}

//  line 519 of elf.h
pub struct Symbol {
    pub name: u32,          // Symbol name (string table index)
    pub address: u64,       // Symbol value
    pub size: u64,          // Symbol size
    pub info: u8,           // Symbol type and binding
    pub other: u8,          // Symbol visibility
    pub section_index: u16, // Section index
    pub name_string: String,
}

impl Symbol {
    pub fn parse_from_buffer(index: u16, binary: &[u8], header: &ELFHeader) -> Symbol {
        // First get the bytes for our header
        let size: usize = match header.ident.ei_class {
            EI_Class::ELF32 => 16,
            EI_Class::ELF64 => 24,
        };

        let start_index = index as usize * size;
        let end_index = start_index + size as usize;
        let raw: &[u8] = &binary[start_index..end_index];

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match header.ident.ei_data {
            EI_Data::LittleEndian => u16::from_le_bytes,
            EI_Data::BigEndian => u16::from_be_bytes,
        };
        let u32_from_bytes = match header.ident.ei_data {
            EI_Data::LittleEndian => u32::from_le_bytes,
            EI_Data::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match header.ident.ei_data {
            EI_Data::LittleEndian => u64::from_le_bytes,
            EI_Data::BigEndian => u64::from_be_bytes,
        };

        match header.ident.ei_class {
            EI_Class::ELF32 => Symbol {
                name: u32_from_bytes(raw[0..4].try_into().unwrap()),
                address: u64::from(u32_from_bytes(raw[4..8].try_into().unwrap())),
                size: u64::from(u32_from_bytes(raw[8..12].try_into().unwrap())),
                info: raw[13],
                other: raw[14],
                section_index: u16_from_bytes(raw[15..16].try_into().unwrap()),
                name_string: std::string::String::new(),
            },
            EI_Class::ELF64 => Symbol {
                name: u32_from_bytes(raw[0..4].try_into().unwrap()),
                info: raw[4],
                other: raw[5],
                section_index: u16_from_bytes(raw[6..8].try_into().unwrap()),
                address: u64_from_bytes(raw[8..16].try_into().unwrap()),
                size: u64_from_bytes(raw[16..24].try_into().unwrap()),
                name_string: std::string::String::new(),
            },
        }
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:?}", "Name Index:", self.name),
            format!("{:15}{:?}", "Name:", self.name_string),
            format!("{:15}0x{:x?}", "Address:", self.address),
            format!(
                "{:15}0x{:x?} {:?} {}",
                "Size:", self.size, self.size, "(bytes)"
            ),
            format!("{:15}{:?}", "Info:", self.info),
            format!("{:15}{:?}", "Other:", self.other),
            format!("{:15}{:?}", "Section Index:", self.section_index),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
