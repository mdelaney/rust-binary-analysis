mod binary;
pub mod elf;
pub mod macho;

use std::fs::File;
use std::io::BufReader;

pub fn load_from_file(path: &str) -> elf::ELF {
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    elf::load_elf_from_buffer(&mut reader).expect("failed to load ELF from file")
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_header_ident() {
        // load_from_file("/bin/ls");
    }

    #[test]
    fn can_parse_elf_32() {
        // load_from_file("/home/mdelaney/infosec/reverse/crackmes/IOLI/bin-linux/crackme0x00");
    }
}
