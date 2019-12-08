extern crate binload;
extern crate capstone;

use std::collections::{HashMap, VecDeque};

use capstone::*;

use binload::elf::section::get_section_by_name;
use binload::elf::ELF;
use binload::load_from_file;

fn get_instruction_string(cs: &Capstone, instruction: &Instruction) -> String {
    let mut byte_strings: Vec<String> = vec![];
    for byte in instruction.bytes {
        byte_strings.push(format!("{:02x}", byte));
    }
    format!(
        "{:#016x} - {:27} {:8} {:16}",
        instruction.address,
        byte_strings.join(" "),
        instruction.mnemonic,
        instruction.op_str,
    )
}

fn get_linear_disassembly(cs: &Capstone, elf: &ELF) {
    let text_section = get_section_by_name(".text", &elf.section_headers)
        .expect("there is no .text section in the executable");
    let text_binary = text_section.get_data(&elf.data);

    let instructions = cs.disassemble(text_binary, text_section.address);
    //        .disasm_all(text_binary, text_section.address)
    //        .expect("Failed to disassemeble");

    println!("Found {} instructions", instructions.len());

    for i in instructions.iter() {
        println!("{}", get_instruction_string(cs, &i));
    }
}

//fn get_basic_recurisive_disassembly(cs: &Capstone, elf: &ELF) {
//    let text_section = get_section_by_name(".text", &elf.section_headers)
//        .expect("there is no .text section in the executable");
//    let text_bytes = text_section.get_data(&elf.data);
//    let mut queue: VecDeque<u64> = VecDeque::new();
//
//    if text_section.contains_address(elf.elf_header.e_entry) {
//        queue.push_back(elf.elf_header.e_entry);
//    }
//
//    // TODO: iterate over symbols and add those addresses if they are in the .text section too
//
//    let mut seen: HashMap<u64, bool> = HashMap::new();
//    while (!queue.is_empty()) {
//        let address = queue
//            .pop_front()
//            .expect("but we just tested that this wasn't empty");
//        if seen.contains_key(&address) {
//            continue;
//        }
//
//        let offset = address - text_section.address;
//        loop {
//            let instructions = cs.disasm_count(text_bytes, address, 1).unwrap_or(break);
//            for instruction in instructions.iter() {
//                //                if is_cs_cflow_ins(cs, ins) {}
//            }
//        }
//    }
//}
//
//use capstone-sys::instruction::InsnGroupType;
//fn is_cs_cflow_group(group: capstone-sys::InsnGroupIdInt) -> bool {
//    match group {
//        InsnGroupType::CS_GRP_JUMP => true,
//
//        _ => false,
//    }
//}
//
//fn is_cs_cflow_ins(cs: &Capstone, instruction: &Insn) -> bool {
//    let detail = match cs.insn_detail(instruction) {
//        Ok(v) => v,
//        Err(_) => return false,
//    };
//    for group in detail.groups() {}
//    // TODO
//    true
//}

fn main() {
    // get raw binary
    let elf = load_from_file("/home/mdelaney/infosec/reverse/crackmes/IOLI/bin-linux/crackme0x00");

    //    capstone::
    // init Capstone
    let cs: Capstone = Capstone::new(Architecture::X86, Endian::LittleEndian);
    //        .x86()
    //        .mode(arch::x86::ArchMode::Mode32)
    //        .syntax(arch::x86::ArchSyntax::Intel)
    //        .detail(true)
    //        .build()
    //        .expect("Failed to create Capstone object");

    // do disassembly
    get_linear_disassembly(&cs, &elf);
}
