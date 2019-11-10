// TODO - remove allow dead code
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::convert::TryInto;
use std::fmt;

use super::elf_header::{EI_Class, EI_Data, ELFHeader};
use std::io::Read;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ProgramHeaderType {
    Null,                   // 0x00 Program header table entry unused
    Load,                   // 0x01 Loadable segment
    Dynamic,                // 0x02 Dynamic linking information
    Interp,                 // 0x03 Interpreter information
    Note,                   // 0x04 Auxiliary information
    ShLib,                  // 0x05 Reserved
    ProgramHeader,          // 0x06 Segment containing the program header table
    ThreadLocalStorage,     // 0x07 Thread-local storage segment
    NumDefinedTypes,        // 0x08 Number of defined types
    GNU_EH_Frame,           // 0x6474e550 GCC .eh_frame_hdr segment
    GNU_Stack,              // 0x6474e551 Indicates stack executability
    GNU_RO_AfterRelocation, // 0x6474e552 Read-only after relocation
    SunWBSS,                // 0x6ffffffa Sun Specific segment
    SunWStack,              // 0x6ffffffb Stack segment
    SunWSpecific,           // 0x6ffffffa-0x6fffffff Sun specific
    OS,                     // 0x60000000-0x6fffffff Start OS specific
    PROC,                   // 0x70000000-0x7fffffff Start processor specific
    Unknown,
}
impl ProgramHeaderType {
    fn from_u32(value: u32) -> ProgramHeaderType {
        match value {
            0x00 => ProgramHeaderType::Null,
            0x01 => ProgramHeaderType::Load,
            0x02 => ProgramHeaderType::Dynamic,
            0x03 => ProgramHeaderType::Interp,
            0x04 => ProgramHeaderType::Note,
            0x05 => ProgramHeaderType::ShLib,
            0x06 => ProgramHeaderType::ProgramHeader,
            0x07 => ProgramHeaderType::ThreadLocalStorage,
            0x08 => ProgramHeaderType::NumDefinedTypes,
            0x6474e550 => ProgramHeaderType::GNU_EH_Frame,
            0x6474e551 => ProgramHeaderType::GNU_Stack,
            0x6474e552 => ProgramHeaderType::GNU_RO_AfterRelocation,
            0x6ffffffa => ProgramHeaderType::SunWBSS,
            0x6ffffffb => ProgramHeaderType::SunWStack,
            0x6ffffffa..=0x6fffffff => ProgramHeaderType::SunWSpecific,
            0x60000000..=0x6fffffff => ProgramHeaderType::OS,
            0x70000000..=0x7fffffff => ProgramHeaderType::PROC,
            _ => ProgramHeaderType::Unknown,
        }
    }
}

impl fmt::Display for ProgramHeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            ProgramHeaderType::Null => "Null",
            ProgramHeaderType::Load => "Load",
            ProgramHeaderType::Dynamic => "Dynamic",
            ProgramHeaderType::Interp => "Interp",
            ProgramHeaderType::Note => "Note",
            ProgramHeaderType::ShLib => "ShLib",
            ProgramHeaderType::ProgramHeader => "ProgramHeader",
            ProgramHeaderType::ThreadLocalStorage => "Thread-local storage segment",
            ProgramHeaderType::NumDefinedTypes => "Number of defined types",
            ProgramHeaderType::GNU_EH_Frame => "GCC .eh_frame_hdr segment",
            ProgramHeaderType::GNU_Stack => "Indicates stack executability",
            ProgramHeaderType::GNU_RO_AfterRelocation => "Read-only after relocation",
            ProgramHeaderType::SunWBSS => "Sun Specific segment",
            ProgramHeaderType::SunWStack => "Stack segment",
            ProgramHeaderType::SunWSpecific => "Sun specific",
            ProgramHeaderType::OS => "OS",
            ProgramHeaderType::PROC => "PROC",
            ProgramHeaderType::Unknown => "Unknown",
        };
        write!(f, "{}", value)
    }
}

// TODO: add flags

pub struct ProgramHeader {
    pub header_type: ProgramHeaderType, // Type of the segment
    pub flags: u32,                     // Segment dependent flags
    pub offset: u64,                    // Offset of the segment in the file
    pub virtual_address: u64,           // Virtual address of segment in memory
    pub physical_address: u64,          // On systems where relevant, phisical address in memory
    pub file_size: u64,                 // Size in bytes of the segment in the file image
    pub memory_size: u64,               // Size in bytes of the segment in memory
    pub align: u64, // 0 and 1 are no alignment, otherwise should be a positive power of 2 with virtual address equating offset modulus align
}

impl ProgramHeader {
    pub fn parse_from_buffer<T: std::io::Read>(
        buffer: &mut T,
        header: &ELFHeader,
    ) -> ProgramHeader {
        // First get the bytes for our header
        let mut raw: [u8; 128] = [0; 128];
        let mut data = buffer.take(u64::from(header.e_phentsize));
        //        buffer.read_exact(&mut raw).unwrap();
        data.read(&mut raw).unwrap();

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
            EI_Class::ELF32 => ProgramHeader {
                header_type: ProgramHeaderType::from_u32(u32_from_bytes(
                    raw[0x00..0x04].try_into().unwrap(),
                )),
                offset: u32_from_bytes(raw[0x04..0x08].try_into().unwrap()) as u64,
                virtual_address: u32_from_bytes(raw[0x08..0x0c].try_into().unwrap()) as u64,
                physical_address: u32_from_bytes(raw[0x0c..0x10].try_into().unwrap()) as u64,
                file_size: u32_from_bytes(raw[0x10..0x14].try_into().unwrap()) as u64,
                memory_size: u32_from_bytes(raw[0x14..0x18].try_into().unwrap()) as u64,
                flags: u32_from_bytes(raw[0x18..0x1c].try_into().unwrap()),
                align: u32_from_bytes(raw[0x1c..0x20].try_into().unwrap()) as u64,
            },
            EI_Class::ELF64 => ProgramHeader {
                header_type: ProgramHeaderType::from_u32(u32_from_bytes(
                    raw[0x00..0x04].try_into().unwrap(),
                )),
                flags: u32_from_bytes(raw[0x04..0x08].try_into().unwrap()),
                offset: u64_from_bytes(raw[0x08..0x10].try_into().unwrap()),
                virtual_address: u64_from_bytes(raw[0x10..0x18].try_into().unwrap()),
                physical_address: u64_from_bytes(raw[0x18..0x20].try_into().unwrap()),
                file_size: u64_from_bytes(raw[0x20..0x28].try_into().unwrap()),
                memory_size: u64_from_bytes(raw[0x28..0x30].try_into().unwrap()),
                align: u64_from_bytes(raw[0x30..0x38].try_into().unwrap()),
            },
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:?}", "Header Type:", self.header_type),
            format!("{:15}{:?}", "Flags:", self.flags),
            format!("{:15}0x{:x?}", "Offset:", self.offset),
            format!("{:15}0x{:x?}", "Virtual address:", self.virtual_address),
            format!("{:15}0x{:x?}", "Physical address:", self.physical_address),
            format!("{:15}{:x?} (bytes)", "File size:", self.file_size),
            format!("{:15}{:x?} (bytes)", "Memory size:", self.memory_size),
            format!("{:15}{:x?}", "Align:", self.align),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for ProgramHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
