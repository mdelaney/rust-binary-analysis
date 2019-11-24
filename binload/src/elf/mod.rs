pub mod elf_header;
pub mod program_header;
pub mod section;
pub mod symbol;
pub mod utils;

use crate::elf::section::get_dynamic_symbols;
use crate::elf::symbol::Symbol;
use program_header::ProgramHeader;
use section::SectionHeader;
use utils::get_null_terminated_string_from_vec;

pub struct ELF {
    pub elf_header: elf_header::ELFHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub data: Vec<u8>,
}

pub fn load_elf_from_buffer<T: std::io::Read + std::io::Seek>(buffer: &mut T) -> Result<ELF, &str> {
    let mut data: Vec<u8> = vec![];
    buffer.read_to_end(&mut data).unwrap();

    // First we get ELF headers
    let elf_ident = elf_header::ELFIdent::parse_from_buffer(&data)?;

    println!("Read struct: \n{:#?}", elf_ident);
    let elf_header = elf_header::ELFHeader::parse_from_buffer(&data, elf_ident);
    println!("header32\n{:#?}", elf_header);

    // Now we get section headers
    let mut section_headers: Vec<SectionHeader> = Vec::new();
    for i in 0..elf_header.e_shnum {
        section_headers.push(SectionHeader::parse_from_buffer(i, &data, &elf_header));
    }

    // get names for section headers
    {
        let name_data = section_headers[elf_header.e_shstrndx as usize].get_data(&data);
        for sh in &mut section_headers {
            sh.name_string = get_null_terminated_string_from_vec(&name_data, sh.name as usize);
        }
    }

    // get program headers
    let mut program_headers: Vec<ProgramHeader> = Vec::new();
    for i in 0..elf_header.e_phnum {
        program_headers.push(ProgramHeader::parse_from_buffer(i, &data, &elf_header));
    }
    println!("Section Headers");
    println!("{}", get_section_headers_print_string(&section_headers));
    println!();
    println!("Program Headers");
    println!("{}", get_program_headers_print_string(&program_headers));
    println!();

    // TODO: you are here - time to get ELF symbols!!
    // TODO: do we have a .dynsym section? If so lets get the symbols
    let dynsym = section::get_dynamic_symbols(&data, &section_headers, &elf_header)?;
    println!("Dynamic Symbols");
    println!("{}", get_symbol_print_string(&dynsym));
    println!();
    //    for sym in dynsym {
    //        println!("{:#?}", sym);
    //    }

    // TODO: do we have a .symtab section? If so lets get the symbols
    //    .symtab is quite likely to not exist as it isn't needed for execution
    Ok(ELF {
        elf_header,
        program_headers,
        section_headers,
        data,
    })
}

fn get_section_headers_print_string(section_headers: &[SectionHeader]) -> String {
    let mut strings: Vec<String> = vec![];
    strings.push(format!(
        "{:15} {:15} {:10} {:18} {:18} {:10} {:10} {:10} {:10} {:10}",
        "Name", "Type", "Flags", "Address", "Offset", "Size", "Link", "Info", "Align", "Entsize",
    ));
    for i in section_headers {
        strings.push(format!(
            "{:15} {:15} {:10} {:#018x} {:#018x} {:#010x} {:#010x} {:#010x} {:#010x} {:#010x}",
            i.name_string,
            i.section_type.to_string(),
            format!("{:?}", i.flags),
            i.address,
            i.offset,
            i.size,
            i.link,
            i.info,
            i.addralign,
            i.entsize,
        ));
    }
    strings.join("\n")
}

fn get_symbol_print_string(symbols: &[Symbol]) -> String {
    let mut strings: Vec<String> = vec![];
    strings.push(format!(
        "{:25} {:18} {:18} {:4} {:4} {:6}",
        "Name", "Address", "size", "Info", "Other", "SectionIdx",
    ));
    for i in symbols {
        strings.push(format!(
            "{:<25} {:#018x} {:#018x} {:#04x} {:#04x} {:#06x}",
            i.name_string,   //: String,
            i.address,       //: u64,       // Symbol value
            i.size,          //: u64,          // Symbol size
            i.info,          //: u8,           // Symbol type and binding
            i.other,         //: u8,          // Symbol visibility
            i.section_index, //: u16, // Section index
        ));
    }
    strings.join("\n")
}

fn get_program_headers_print_string(program_headers: &[ProgramHeader]) -> String {
    let mut strings: Vec<String> = vec![];
    strings.push(format!(
        "{:15} {:10} {:18} {:18} {:10} {:10} {:6} {:10}",
        "Type",
        "Offset",
        "Virtual Address",
        "Physical Address",
        "FileSize",
        "Mem Size",
        "Flag",
        "Align",
    ));
    for i in program_headers {
        strings.push(format!(
            "{:<15} {:#010x} {:#018x} {:#018x} {:010} {:010} {:6} {:#010x}",
            i.header_type.to_string(),
            i.offset,
            i.virtual_address,
            i.physical_address,
            i.file_size,
            i.memory_size,
            i.flags,
            i.align,
        ));
    }
    strings.join("\n")
}
