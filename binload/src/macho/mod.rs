pub mod header;
pub mod fat;

pub struct MACHO {
    pub header: header::Header,
}

pub fn load_macho_from_buffer<T: std::io::Read + std::io::Seek>(buffer: &mut T) -> Result<MACHO, &str> {
    let mut data: Vec<u8> = vec![];
    buffer.read_to_end(&mut data).unwrap();

    // First we get MACHO headers
    let header = header::Header::parse_from_buffer(&data)?;

    // println!("Read struct: \n{:#?}", elf_ident);
    // let elf_header = elf_header::ELFHeader::parse_from_buffer(&data, elf_ident);
    println!("header32\n{:#?}", header);

    Ok(MACHO {
        header,
    })
}
