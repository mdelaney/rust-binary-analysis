pub mod elf_header;
pub mod section_header;

pub fn load_elf_from_buffer<T: std::io::Read>(buffer: &mut T) {
    let ident = elf_header::ELFIdent::parse_from_buffer(buffer);

    println!("Read struct: \n{:#?}", ident);
    match ident.ei_class {
        elf_header::EI_Class::ELF32 => {
            let elf_header = elf_header::ELFHeader32::parse_from_buffer(buffer, ident);
            println!("header32\n{:#?}", elf_header);
        }
        elf_header::EI_Class::ELF64 => {
            let elf_header = elf_header::ELFHeader64::parse_from_buffer(buffer, ident);
            println!("header32\n{:#?}", elf_header);
        }
    };
}
