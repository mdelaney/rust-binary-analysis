mod elf;
mod binary;

//use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
//use crate::binary::Binary;


pub fn load_from_file(path: &str) {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    let ident = elf::ELFIdent::parse_from_buffer(&mut reader);

    let binary = binary::Binary {
        filename: path.to_string(),
        binary_type: binary::BinaryType::ELF,
        arch: binary::Arch::UNKNOWN,
        bits: 0,
        entry_point: 0,
        sections: vec![],
        symbols: vec![]
    };
    println!("Read struct: {:#?}", ident);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_header_ident() {
        load_from_file("/bin/ls");
    }
}
