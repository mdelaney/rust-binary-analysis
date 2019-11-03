pub mod elf_header;
pub mod section_header;

use std::io::SeekFrom;

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
        buffer.seek(SeekFrom::Start(section_headers[i].offset));
        let mut data = vec![0; section_headers[i].size as usize];
        buffer.read_exact(&mut data);
        section_headers[i].data = data;
    }

    // get names for section headers
    for i in 0..elf_header.e_shnum as usize {
        section_headers[i].name_string = get_null_terminated_string_from_vec(
            &section_headers[elf_header.e_shstrndx as usize].data,
            section_headers[i].name as usize,
        );
    }

    for i in section_headers {
        println!("section header\n{:#?}", i);
    }
}
