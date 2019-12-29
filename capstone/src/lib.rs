extern crate capstone_sys;

pub mod arch;
mod capstone;
mod instruction;

pub use capstone::*;
pub use instruction::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        const X86_CODE: &'static [u8] =
            b"\x55\x48\x8b\x05\xb8\x13\x00\x00\xe9\x14\x9e\x08\x00\x45\x31\xe4";
        let cs = Capstone::new(Architecture::X86, Endian::LittleEndian);
        let insn = cs.disassemble(X86_CODE);
        for i in insn {
            println!("{} {} {}", i.id, i.address, i.size);
        }
        println!("{:?} {:?}", cs.architecture, cs.endian);
    }
}
