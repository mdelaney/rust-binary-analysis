use std::io::Read;
use std::mem;
use std::slice;


#[derive(Debug)]
pub enum EI_DATA {
    LITTLE_ENDIAN,
    BIG_ENDIAN,
}

#[derive(Debug)]
pub struct ELFIdent {
    pub ei_magic: [u8; 4],
    pub ei_class: u8,       // 1 == 32 bit, 2 == 64 bit
    pub ei_data: EI_DATA,        // 1 == little endian, 2 == big endian
    pub ei_version: u8,
    pub ei_os_abi: u8,
    pub ei_abi_version: u8,
    pub ei_pad: [u8; 7],
}

impl ELFIdent {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T) -> ELFIdent {
        const size: usize = mem::size_of::<ELFIdent>();
        let mut raw_ident: [u8; size] = [0; size];
        buffer.read_exact(&mut raw_ident);

        let mut magic=[0;4];
        magic.copy_from_slice(&raw_ident[0..4]);

        let mut pad=[0;7];
        pad.copy_from_slice(&raw_ident[9..16]);

        let mut result: ELFIdent = ELFIdent {
            ei_magic: magic,
            ei_class: raw_ident[4],
            ei_data: match raw_ident[5] {
                1 => EI_DATA::LITTLE_ENDIAN,
                2 => EI_DATA::LITTLE_ENDIAN,
                _ => panic!("Invalid ei_data value"),
            },
            ei_version: raw_ident[6],
            ei_os_abi: raw_ident[7],
            ei_abi_version: raw_ident[8],
            ei_pad: pad,
        };
        // TODO: validate magic
        result
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct ELFHeader32 {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32, // arch
    pub e_phoff: u32, // arch
    pub e_shoff: u32, // arch
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

use std::convert::TryInto;

impl ELFHeader32 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> ELFHeader32 {
        // First get the bytes for our header
        const size: usize = mem::size_of::<ELFHeader32>();
        let mut raw: [u8; size] = [0; size];
        buffer.read_exact(&mut raw);

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u16::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u32::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u32::from_be_bytes,
        };

        // Finally we can create our header
        let mut result: ELFHeader32 = Default::default();
        result.e_type      = u16_from_bytes(raw[0..2].try_into().unwrap());
        result.e_machine   = u16_from_bytes(raw[2..4].try_into().unwrap());
        result.e_version   = u32_from_bytes(raw[4..8].try_into().unwrap());
        result.e_entry     = u32_from_bytes(raw[8..12].try_into().unwrap());
        result.e_phoff     = u32_from_bytes(raw[12..16].try_into().unwrap());
        result.e_shoff     = u32_from_bytes(raw[16..20].try_into().unwrap());
        result.e_flags     = u32_from_bytes(raw[20..24].try_into().unwrap());
        result.e_ehsize    = u16_from_bytes(raw[24..26].try_into().unwrap());
        result.e_phentsize = u16_from_bytes(raw[26..28].try_into().unwrap());
        result.e_phnum     = u16_from_bytes(raw[28..30].try_into().unwrap());
        result.e_shentsize = u16_from_bytes(raw[30..32].try_into().unwrap());
        result.e_shnum     = u16_from_bytes(raw[32..34].try_into().unwrap());
        result.e_shstrndx  = u16_from_bytes(raw[34..36].try_into().unwrap());

        result
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct ELFHeader64 {
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64, // arch
    pub e_phoff: u64, // arch
    pub e_shoff: u64, // arch
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ELFHeader64 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> ELFHeader64 {
        // First get the bytes for our header
        const size: usize = mem::size_of::<ELFHeader64>();
        let mut raw: [u8; size] = [0; size];
        buffer.read_exact(&mut raw);

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u16::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u32::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u64::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u64::from_be_bytes,
        };

        // Finally we can create our header
        let mut result: ELFHeader64 = Default::default();
        result.e_type      = u16_from_bytes(raw[0..2].try_into().unwrap());
        result.e_machine   = u16_from_bytes(raw[2..4].try_into().unwrap());
        result.e_version   = u32_from_bytes(raw[4..8].try_into().unwrap());
        result.e_entry     = u64_from_bytes(raw[8..16].try_into().unwrap());
        result.e_phoff     = u64_from_bytes(raw[16..24].try_into().unwrap());
        result.e_shoff     = u64_from_bytes(raw[24..32].try_into().unwrap());
        result.e_flags     = u32_from_bytes(raw[32..36].try_into().unwrap());
        result.e_ehsize    = u16_from_bytes(raw[36..38].try_into().unwrap());
        result.e_phentsize = u16_from_bytes(raw[38..40].try_into().unwrap());
        result.e_phnum     = u16_from_bytes(raw[40..42].try_into().unwrap());
        result.e_shentsize = u16_from_bytes(raw[42..44].try_into().unwrap());
        result.e_shnum     = u16_from_bytes(raw[44..46].try_into().unwrap());
        result.e_shstrndx  = u16_from_bytes(raw[46..48].try_into().unwrap());

        result
    }
}

