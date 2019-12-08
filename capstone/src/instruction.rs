use capstone_sys::bindings::cs_insn;
use std::ffi::CStr;

// maps to cs_op_type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OpType {
    Invalid = 0,
    Reg = 1,
    Imm = 2,
    Mem = 3,
    FP = 4,
}

// maps to cs_group_type
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GroupType {
    Invalid = 0,
    Jump = 1,
    Call = 2,
    Ret = 3,
    Int = 4,
    Iret = 5,
}

#[derive(Clone)]
pub struct Instruction<'a> {
    pub id: ::std::os::raw::c_uint,
    pub address: u64,
    pub size: u16,
    pub bytes: &'a [u8], //[u8; 16usize],
    pub mnemonic: &'a str,
    pub op_str: &'a str,
    //    pub detail: *mut Detail,
}
impl<'a> Instruction<'a> {
    pub fn create_from_cs_insn(insn: &'a cs_insn) -> Instruction<'a> {
        let mnemonic: &str;
        unsafe {
            mnemonic = CStr::from_ptr(insn.mnemonic.as_ptr()).to_str().unwrap();
        }
        let op_str: &str;
        unsafe {
            op_str = CStr::from_ptr(insn.op_str.as_ptr()).to_str().unwrap();
        }
        Instruction {
            id: insn.id,
            address: insn.address,
            size: insn.size,
            bytes: &insn.bytes[0..insn.size as usize],
            mnemonic,
            op_str,
            //                detail: (),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Detail {
    pub regs_read: [u8; 12usize],
    pub regs_read_count: u8,
    pub regs_write: [u8; 20usize],
    pub regs_write_count: u8,
    pub groups: [u8; 8usize],
    pub groups_count: u8,
    //    pub __bindgen_anon_1: cs_detail__bindgen_ty_1,
}
