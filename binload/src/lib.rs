mod binary;
mod elf;

use std::fs::File;
use std::io::BufReader;

pub fn load_from_file(path: &str) {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    match elf::load_elf_from_buffer(&mut reader) {
        Ok(_) => {}
        Err(_) => {}
    }

    //    let binary = binary::Binary {
    //        filename: path.to_string(),
    //        binary_type: binary::BinaryType::ELF,
    //        arch: binary::Arch::UNKNOWN,
    //        bits: 0,
    //        entry_point: 0,
    //        sections: vec![],
    //        symbols: vec![]
    //    };
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_header_ident() {
        load_from_file("/bin/ls");
    }

    #[test]
    fn can_parse_elf_32() {
        //        load_from_file("/home/mdelaney/infosec/reverse/crackmes/IOLI/bin-linux/crackme0x00")
    }
}
