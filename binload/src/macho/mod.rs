pub mod header;
pub mod fat;

pub struct MACHO {
    pub header: header::Header,
}

pub fn load_macho_from_buffer<T: std::io::Read + std::io::Seek>(buffer: &mut T) -> Result<MACHO, &str> {
    let mut data: Vec<u8> = vec![];
    buffer.read_to_end(&mut data).unwrap();

    let header = if fat::is_fat_binary(&data) {
        // TODO: we need to decide what architecture we want to proceed with, for now we'll just use
        //       the first one
        let architectures = fat::FatArchitecture::parse_from_buffer(&data)?;
        // TODO: probably want to handle the case where there are no architectures present

        header::Header::parse_from_buffer(&architectures[0].get_binary(&data))
    } else {
        header::Header::parse_from_buffer(&data)
    }?;


    // println!("Read struct: \n{:#?}", elf_ident);
    // let elf_header = elf_header::ELFHeader::parse_from_buffer(&data, elf_ident);
    println!("header32\n{:#?}", header);

    Ok(MACHO {
        header,
    })
}

mod macho_full_tests {
    use std::fs::File;
    use std::io::BufReader;
    use super::*;

    #[test]
    fn can_load_fat_binary() {
        let f = File::open("/bin/ls").unwrap();
        let mut reader = BufReader::new(f);

        load_macho_from_buffer(&mut reader).expect("failed to load ELF from file");
    }
}