pub mod elf_header;
pub mod program_header;
pub mod section_header;

use std::io::SeekFrom;

use program_header::ProgramHeader;
use section_header::SectionHeader;

fn get_null_terminated_string_from_vec(vec: &Vec<u8>, offset: usize) -> String {
    let mut length: usize = 0;
    for i in offset..vec.len() {
        if vec[i] == 0x00 {
            length = i;
            break;
        }
    }
    let mut string = std::string::String::with_capacity(length);
    for i in offset..length {
        string.push(vec[i] as char);
    }
    string
}

pub fn load_elf_from_buffer<T: std::io::Read + std::io::Seek>(buffer: &mut T) {
    // First we get ELF headers
    let ident = elf_header::ELFIdent::parse_from_buffer(buffer);
    println!("Read struct: \n{:#?}", ident);
    let elf_header = elf_header::ELFHeader::parse_from_buffer(buffer, ident);
    println!("header32\n{:#?}", elf_header);

    // Now we get section headers
    let mut section_headers: Vec<SectionHeader> = Vec::new();
    buffer.seek(SeekFrom::Start(elf_header.e_shoff)).unwrap();
    for _i in 0..elf_header.e_shnum {
        section_headers.push(SectionHeader::parse_from_buffer(buffer, &elf_header));
    }

    // get data for section headers
    for i in 0..elf_header.e_shnum as usize {
        buffer
            .seek(SeekFrom::Start(section_headers[i].offset))
            .unwrap();
        let mut data = vec![0; section_headers[i].size as usize];
        buffer.read_exact(&mut data).unwrap();
        section_headers[i].data = data;
    }

    // get names for section headers
    for i in 0..elf_header.e_shnum as usize {
        section_headers[i].name_string = get_null_terminated_string_from_vec(
            &section_headers[elf_header.e_shstrndx as usize].data,
            section_headers[i].name as usize,
        );
    }

    // get program headers
    let mut program_headers: Vec<ProgramHeader> = Vec::new();
    buffer.seek(SeekFrom::Start(elf_header.e_phoff)).unwrap();
    for _ in 0..elf_header.e_phnum as usize {
        program_headers.push(ProgramHeader::parse_from_buffer(buffer, &elf_header));
    }
    println!("Section Headers");
    println!("{}", get_section_headers_print_string(&section_headers));
    println!();
    println!("Program Headers");
    println!("{}", get_program_headers_print_string(&program_headers));
    println!();
}

fn get_section_headers_print_string(section_headers: &[SectionHeader]) -> String {
    let mut strings: Vec<String> = vec![];
    strings.push(format!(
        "{:20} {:10} {:10} {:18} {:18} {:10} {:10} {:10} {:10} {:10}",
        "Name", "Type", "Flags", "Address", "Offset", "Size", "Link", "Info", "Align", "Entsize",
    ));
    for i in section_headers {
        strings.push(format!(
            "{:20} {:10} {:10} {:#018x} {:#018x} {:#010x} {:#010x} {:#010x} {:#010x} {:#010x}",
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
