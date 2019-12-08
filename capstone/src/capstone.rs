//use crate::capstone::Error::Detail;
use crate::instruction::Instruction;
use capstone_sys::bindings::*;
use std::ffi::CStr;
use std::mem;

// maps to cs_err
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Ok = 0,
    Mem = 1,
    Arch = 2,
    Handle = 3,
    Csh = 4,
    Mode = 5,
    Option = 6,
    Detail = 7,
    MemSetup = 8,
    Version = 9,
    Diet = 10,
    SkipData = 11,
    X86ATT = 12,
    X86Intel = 13,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Architecture {
    ARM = 0,
    ARM64 = 1,
    MIPS = 2,
    X86 = 3,
    PPC = 4,
    SPARC = 5,
    SYSZ = 6,
    XCORE = 7,
    MAX = 8,
    ALL = 65535,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// Endian is part of the capstone_sys cs_mode enum
pub enum Endian {
    LittleEndian = 0,
    BigEndian = -2147483648,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// Mode is part of the capstone_sys cs_mode enum
pub enum Mode {
    _16 = 2,
    _32 = 4,
    _64 = 8,
    THUMB = 16,
    MCLASS = 32,
    V8 = 64,
    MIPSGP64 = 128,
}

// maps to cs_opt_type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionType {
    Invalid = 0,
    Syntax = 1,
    Detail = 2,
    Mode = 3,
    Mem = 4,
    SkipData = 5,
    SkipDataSetup = 6,
}

// maps to cs_opt_value
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionValue {
    Off = 0,
    On = 3,
    SyntaxIntel = 1,
    SyntaxATT = 2,
}

pub struct Capstone {
    csh: csh,
    pub architecture: Architecture,
    pub endian: Endian,
}
impl Capstone {
    #[allow()]
    pub fn new(architecture: Architecture, endian: Endian) -> Capstone {
        let mut csh: csh = 0;
        unsafe {
            cs_open(
                mem::transmute(architecture), // we're converting to cs_arch
                mem::transmute(endian),       // we're converting to cs_mode
                &mut csh,
            );
        }
        Capstone {
            csh,
            architecture,
            endian,
        }
    }

    pub fn disassemble(&self, binary: &[u8], address: u64) -> Vec<Instruction> {
        let mut insn: *mut cs_insn = unsafe { mem::zeroed() };
        let count;
        unsafe {
            count = cs_disasm(
                self.csh,
                binary.as_ptr(),
                binary.len(),
                address,
                0,
                &mut insn,
            );
        }
        let cs_instructions = unsafe { std::slice::from_raw_parts(insn, count) };
        let mut result: Vec<Instruction> = vec![];
        for instruction in cs_instructions {
            result.push(Instruction::create_from_cs_insn(instruction))
        }
        result
    }

    pub fn disassemble_iter(&self, binary: &[u8]) {
        // TODO
        // the function signature is
        //     pub fn cs_disasm_iter(
        //         handle: csh,
        //         code: *mut *const u8,
        //         size: *mut usize,
        //         address: *mut u64,
        //         insn: *mut cs_insn,
        //     ) -> bool;
    }

    /// get the string representing a given instruction
    pub fn get_mnemonic(&self, instruction: &Instruction) -> &str {
        let name: &str;
        unsafe {
            name = CStr::from_ptr(cs_insn_name(self.csh, instruction.id))
                .to_str()
                .unwrap();
        }
        name
    }
}

impl Drop for Capstone {
    fn drop(&mut self) {
        unsafe {
            cs_close(&mut self.csh);
        }
    }
}
