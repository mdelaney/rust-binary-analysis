pub mod elf_header;
pub mod section_header;

pub fn load_elf_from_buffer<T: std::io::Read>(buffer: &mut T) {
    let ident = elf_header::ELFIdent::parse_from_buffer(buffer);

    println!("Read struct: \n{:#?}", ident);
    let elf_header = elf_header::ELFHeader::parse_from_buffer(buffer, ident);
    println!("header32\n{:#?}", elf_header);

    // TODO: you are here.... Time to get section headers!!
}
