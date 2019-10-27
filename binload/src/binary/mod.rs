// TODO - remove allow dead code
#![allow(dead_code)]

mod section;
mod symbol;

#[allow(dead_code)]
pub enum Arch {
    UNKNOWN,
    X86,
    X86_64,
}

#[allow(dead_code)]
pub enum BinaryType {
    ELF,
    PE,
}

pub struct Binary {
    pub filename: std::string::String,
    pub binary_type: BinaryType,
    pub arch: Arch,
    pub bits: u8,
    pub entry_point: u64,
    pub sections: Vec<section::Section>,
    pub symbols: Vec<symbol::Symbol>,
}
