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

// This just dumps the disassembled instructions to stdout
fn get_linear_disassembly(cs: &Capstone, elf: &ELF) {
    let text_section = get_section_by_name(".text", &elf.section_headers)
        .expect("there is no .text section in the executable");
    let text_binary = text_section.get_data(&elf.data);

    // TODO REM START
    cs.set_option(OptionType::Detail, OptionValue::On);
    // TODO REM END
    let instructions = cs.disassemble(text_binary, text_section.address, 0);
    //        .disasm_all(text_binary, text_section.address)
    //        .expect("Failed to disassemeble");

    println!("Found {} instructions", instructions.len());

    for i in instructions.iter() {
        println!("{}", get_instruction_string(cs, &i));
        if let Some(groups) = i.groups {
            for group in groups {
                println!(
                    "\t{:?}",
                    capstone::arch::x86::instruction::GroupType::from_u8(*group)
                );
            }
        }
    }
}

fn get_basic_recurisive_disassembly(cs: &Capstone, elf: &ELF) {
    let text_section = get_section_by_name(".text", &elf.section_headers)
        .expect("there is no .text section in the executable");
    let text_bytes = text_section.get_data(&elf.data);
    let mut queue: VecDeque<u64> = VecDeque::new();

    if text_section.contains_address(elf.elf_header.e_entry) {
        queue.push_back(elf.elf_header.e_entry);
    }

    for address in get_symbols_in_text_section(&elf) {
        queue.push_back(address);
    }

    // TODO: iterate over symbols and add those addresses if they are in the .text section too

    let mut seen: HashMap<u64, bool> = HashMap::new();
    while !queue.is_empty() {
        let address = queue
            .pop_front()
            .expect("but we just tested that this wasn't empty");
        if seen.contains_key(&address) {
            continue;
        }

        let mut pc = (address - text_section.address) as usize;
        loop {
            let insn = &cs.disassemble(&text_bytes[pc..], address + pc as u64, 1)[0];
            pc += insn.size as usize;
            if pc >= text_bytes.len() {
                break;
            }
            if insn.id == arch::x86::instruction::InstructionId::INVALID as u32 || insn.size == 0 {
                break;
            }

            seen.insert(insn.address, true);
            println!("{}", get_instruction_string(cs, &insn));

            if is_cs_cflow_ins(&insn) {
                let target: u64 = get_cs_ins_immediate_target(cs, insn);
                // FIXME: we jump to the .plt section for __libc_start_main but because this is not
                // in the text section we just skip. If we followed, we'd get a simple jump
                if target != 0
                    && !seen.contains_key(&target)
                    && text_section.contains_address(target)
                {
                    queue.push_back(target);
                    println!(" -> new target {:#016x}", target);
                }
                if is_cs_unconditional_cflow_ins(&insn) {
                    break;
                }
            } else if insn.id == arch::x86::instruction::InstructionId::HLT as u32 {
                break;
            }
        }
        println!("-------------------");
    }
}

/// Returns bool indicating if the group id is a control flow group
fn is_cs_cflow_group(group: u8) -> bool {
    use arch::x86::instruction::GroupType;
    match GroupType::from_u8(group) {
        GroupType::JUMP => true,
        GroupType::CALL => true,
        GroupType::RET => true,
        GroupType::IRET => true,
        _ => false,
    }
}

/// Returns bool indicating if the instruction is a control flow instruction
fn is_cs_cflow_ins(insn: &Instruction) -> bool {
    let groups = match insn.groups {
        Some(v) => v,
        None => return false,
    };
    for group in groups {
        if is_cs_cflow_group(*group) {
            return true;
        }
    }
    false
}

/// Returns bool indicating if the instruction is a unconditional control flow instruction
fn is_cs_unconditional_cflow_ins(insn: &Instruction) -> bool {
    use arch::x86::instruction::InstructionId;
    match InstructionId::from_u32(insn.id) {
        InstructionId::JMP => true,
        InstructionId::LJMP => true,
        InstructionId::RET => true,
        InstructionId::RETF => true,
        InstructionId::RETFQ => true,
        _ => false,
    }
}

fn get_cs_ins_immediate_target(cs: &Capstone, insn: &Instruction) -> u64 {
    let groups = match insn.groups {
        Some(v) => v,
        None => return 0,
    };
    let detail = match insn.detail {
        Some(v) => v,
        None => return 0,
    };
    for group in groups {
        if is_cs_cflow_group(*group) {
            for i in 0..unsafe { detail.x86.op_count } {
                let op = unsafe { detail.x86.operands[i as usize] };
                if op.type_ as u8 == 2 {
                    //arch::x86::instruction::Operand::IMM {
                    return unsafe { op.__bindgen_anon_1.imm } as u64;
                }
            }
        }
    }
    0
}

fn get_symbols_in_text_section(elf: &ELF) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    let text_section = get_section_by_name(".text", &elf.section_headers)
        .expect("there is no .text section in the executable");

    for symbol in &elf.symbol_table {
        if text_section.contains_address(symbol.address) {
            result.push(symbol.address);
        }
    }
    result
}

fn main() {
    // get raw binary
    let elf = load_from_file("/Users/mdelaney/Development/binary-analysis/crackmes/IOLI/bin-linux/crackme0x00");

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
    //    get_linear_disassembly(&cs, &elf);
    get_basic_recurisive_disassembly(&cs, &elf);
}
