#![macro_use]
extern crate enum_primitive;

use std::convert::TryInto;
use std::fmt;
use enum_primitive::FromPrimitive;
use crate::macho::header::{CpuType, Endian};

#[derive(Debug, Eq, PartialEq)]
pub struct Architecture {
    pub cpu_type: CpuType,
    pub cpu_subtype: u32,
    pub offset: u32,
    pub size: u32,
    pub align: u32,
}

impl Architecture {
    pub fn parse_from_buffer(binary: &[u8]) -> Result<Vec<Architecture>, &'static str> {
        let mut result: Vec<Architecture> = Vec::new();

        // First we verify that this is in fact a fat file
        let mut magic = [0; 4];
        magic.copy_from_slice(&binary[0..4]);
        let endian = match magic {
            [0xbe, 0xba, 0xfe, 0xca] => Endian::LittleEndian,
            [0xca, 0xfe, 0xba, 0xbe] => Endian::BigEndian,
            _ => return Err("This is not a fat file!"),
        };

        let u32_from_bytes = get_num_from_bytes!(u32, endian);
        // now we see how many architectures are present here and parse each one.
        const ARCH_STRUCT_SIZE: usize = 5 * 4; // 5 fields, each 4 bytes
        let count: usize = u32_from_bytes(binary[4..8].try_into().unwrap()) as usize;
        for i in 0..count {
            let offset: usize = 8 + i * ARCH_STRUCT_SIZE;
            let raw: &[u8] = &binary[offset..offset + ARCH_STRUCT_SIZE];
            result.push(Architecture {
                cpu_type: CpuType::from_u32(u32_from_bytes(raw[0..4].try_into().unwrap())).unwrap(),
                cpu_subtype: u32_from_bytes(raw[4..8].try_into().unwrap()),
                offset: u32_from_bytes(raw[8..12].try_into().unwrap()),
                size: u32_from_bytes(raw[12..16].try_into().unwrap()),
                align: u32_from_bytes(raw[16..20].try_into().unwrap()),
            })
        }
        Ok(result)
    }

    pub fn get_binary<'a>(&self, binary: &'a[u8]) -> &'a[u8] {
        &binary[self.offset as usize..(self.offset + self.size) as usize]
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:x?}", "CPU Type:", self.cpu_type),
            format!("{:15}{:x?}", "CPU Subtype:", self.cpu_subtype),
            format!("{:15}{:x?}", "Offset:", self.offset),
            format!("{:15}{:x?}", "Size:", self.size),
            format!("{:15}{:x?}", "Align:", self.align),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}
impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

mod macho_fat_tests {
    use std::fmt;
    use super::*;

    #[test]
    fn can_parse_basic_ident_section_arm64() {
        let raw = [
            0xca, 0xfe, 0xba, 0xbe,
            0x00, 0x00, 0x00, 0x02,
            0x01, 0x00, 0x00, 0x07,
            0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x40, 0x00,
            0x00, 0x01, 0x1c, 0x60,
            0x00, 0x00, 0x00, 0x0e,
            0x01, 0x00, 0x00, 0x0c,
            0x80, 0x00, 0x00, 0x02,
            0x00, 0x01, 0x80, 0x00,
            0x00, 0x01, 0x5a, 0xa0,
            0x00, 0x00, 0x00, 0x0e,
            0x00, 0x00, 0x00, 0x00,
        ];
        let expected: Vec<Architecture> = vec![
            Architecture {
                cpu_type: CpuType::X86_64,
                cpu_subtype: 0x03,
                offset: 0x00004000,
                size: 0x00011c60,
                align: 0x0000000e,
            },
            Architecture {
                cpu_type: CpuType::ARM64,
                cpu_subtype: 0x80000002,
                offset: 0x00018000,
                size: 0x00015aa0,
                align: 0x0000000e,
            },
        ];
        let header = Architecture::parse_from_buffer(&raw);
        match header {
            Ok(v) => assert_eq!(v, expected),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn can_retrieve_architecture_binary() {
        let raw = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x40, 0x00, 0x00, 0x01, 0x1c, 0x60, 0x00, 0x00, 0x00, 0x0e, 0x01, 0x00, 0x00, 0x0c,
            0x80, 0x00, 0x00, 0x02, 0x00, 0x01, 0x80, 0x00, 0x00, 0x01, 0x5a, 0xa0, 0x00, 0x00, 0x00, 0x0e,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            // 64 offset
            0xcf, 0xfa, 0xed, 0xfe, 0x07, 0x00, 0x00, 0x01, 0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00,
            0x12, 0x00, 0x00, 0x00, 0x18, 0x07, 0x00, 0x00, 0x85, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,

            // 96 offset
            0xcf, 0xfa, 0xed, 0xfe, 0x0c, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x80, 0x02, 0x00, 0x00, 0x00,
            0x13, 0x00, 0x00, 0x00, 0xc0, 0x06, 0x00, 0x00, 0x85, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let arch: Vec<Architecture> = vec![
            Architecture {
                cpu_type: CpuType::X86_64,
                cpu_subtype: 0x03,
                offset: 64,
                size: 32,
                align: 0x0000000e,
            },
            Architecture {
                cpu_type: CpuType::ARM64,
                cpu_subtype: 0x80000002,
                offset: 96,
                size: 32,
                align: 0x0000000e,
            },
        ];
        assert_eq!(&raw[64..96], arch[0].get_binary(&raw));
        assert_eq!(&raw[96..128], arch[1].get_binary(&raw));
    }
}