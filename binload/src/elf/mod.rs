use std::io::Read;
use std::mem;
use std::slice;


#[repr(C, packed)]
#[derive(Debug)]
pub struct ELFIdent {
    ei_magic: [i8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_os_abi: u8,
    ei_abi_version: u8,
    ei_pad: [i8; 7],
}

impl ELFIdent {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T) -> ELFIdent {
        let mut result: ELFIdent = unsafe { mem::zeroed() };
        let result_size = mem::size_of::<ELFIdent>();
        unsafe {
            let result_slice = slice::from_raw_parts_mut(
                &mut result as *mut _ as *mut u8,
                result_size
            );
            buffer.read_exact(result_slice).unwrap();
        }
        result
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct ELFHeader32 {
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32, // arch
    e_phoff: u32, // arch
    e_shoff: u32, // arch
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ELFHeader32 {
    pub fn parse_from_buffer(mut buffer: &[u8]) -> ELFHeader32 {
        let mut result: ELFHeader32 = unsafe { mem::zeroed() };
        let size = mem::size_of::<ELFIdent>();
        unsafe {
            let result_slice = slice::from_raw_parts_mut(
                &mut result as *mut _ as *mut u8,
                size
            );
            buffer.read_exact(result_slice).unwrap();
        }
        result
    }
}

#[repr(C, packed)]
#[derive(Debug)]
pub struct ELFHeader64 {
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64, // arch
    e_phoff: u64, // arch
    e_shoff: u64, // arch
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

impl ELFHeader64 {
    pub fn parse_from_buffer(mut buffer: &[u8]) -> ELFHeader64 {
        let mut result: ELFHeader64 = unsafe { mem::zeroed() };
        let size = mem::size_of::<ELFIdent>();
        unsafe {
            let result_slice = slice::from_raw_parts_mut(
                &mut result as *mut _ as *mut u8,
                size
            );
            buffer.read_exact(result_slice).unwrap();
        }
        result
    }
}

