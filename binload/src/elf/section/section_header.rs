use std::convert::TryInto;
use std::fmt;

use super::super::elf_header::{EI_Class, EI_Data, ELFHeader};
use super::enums::{SectionFlags, SectionType};

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
    pub fn contains_address(&self, address: u64) -> bool {
        address > self.address && address < (self.address + self.size)
    }

    pub fn get_data<'a>(&self, binary: &'a [u8]) -> &'a [u8] {
        let start = self.offset as usize;
        let end = start + self.size as usize;
        &binary[start..end]
    }

    pub fn parse_from_buffer(index: u16, binary: &[u8], header: &ELFHeader) -> SectionHeader {
        // First get the bytes for our header
        let start_index = header.e_shoff as usize + (index * header.e_shentsize) as usize;
        let end_index = start_index + header.e_shentsize as usize;
        let raw: &[u8] = &binary[start_index..end_index];

        // Now get our conversion functions to read numbers based on endianness
        let u32_from_bytes = get_num_from_bytes!(u32, header.ident.ei_data);
        let u64_from_bytes = get_num_from_bytes!(u64, header.ident.ei_data);

        // Finally we can create our header
        match header.ident.ei_class {
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
        }
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
