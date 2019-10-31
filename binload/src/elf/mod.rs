pub mod elf_header;
pub mod section_header;

use std::io::SeekFrom;

use section_header::SectionHeader;

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
        println!("PARSING SH {}", _i);
        section_headers.push(SectionHeader::parse_from_buffer(buffer, &elf_header));
    }

    for i in section_headers {
        println!("section header\n{:#?}", i);
    }

    // TODO: you are here.... Section headers flags shouldn't be a single value but rather
    //       a list of values
    // TODO: next up.... function to get the bytes for a section header
    // TODO: get names for section headers and use them in the section header formatter
}
