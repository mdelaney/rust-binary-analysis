extern crate binload;
extern crate capstone;

use capstone::prelude::*;

use binload::elf::section::get_section_by_name;
use binload::load_from_file;

fn get_instruction_string(instruction: &capstone::Insn) -> String {
    let mut byte_strings: Vec<String> = vec![];
    for byte in instruction.bytes() {
        byte_strings.push(format!("{:02x}", byte));
    }
    format!(
        "{:#016x} - {:25} {:8} {:16}",
        instruction.address(),
        byte_strings.join(" "),
        instruction.mnemonic().unwrap(),
        instruction.op_str().unwrap()
    )
}

fn main() {
    // get raw binary
    let elf = load_from_file("/home/mdelaney/infosec/reverse/crackmes/IOLI/bin-linux/crackme0x00");
    let text_section = get_section_by_name(".text", &elf.section_headers).unwrap();
    let text_binary = text_section.get_data(&elf.data);

    // init Capstone
    let cs: Capstone = Capstone::new()
        .x86()
        .mode(arch::x86::ArchMode::Mode32)
        .syntax(arch::x86::ArchSyntax::Intel)
        .detail(true)
        .build()
        .expect("Failed to create Capstone object");

    // do disassembly
    let instructions = cs
        .disasm_all(text_binary, text_section.address)
        .expect("Failed to disassemeble");

    println!("Found {} instructions", instructions.len());

    for i in instructions.iter() {
        println!("{}", get_instruction_string(&i));

        //let detail: InsnDetail = cs.insn_detail(&i).expect("Failed to get insn detail");
        //let arch_detail: ArchDetail = detail.arch_detail();
        //let ops = arch_detail.operands();
    }
}
