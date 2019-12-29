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
// TODO: YOU ARE HERE! you want to create a macro to get appropriate group type based on arch
//#[repr(u32)]
//#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
//pub enum GroupType {
//    INVALID = 0,
//    JUMP = 1,
//    CALL = 2,
//    RET = 3,
//    INT = 4,
//    IRET = 5,
//}
//impl GroupType {
//    fn from_u8(value: u8) -> Option<Self> {
//        match value {
//            0 => Some(GroupType::INVALID),
//            1 => Some(GroupType::JUMP),
//            2 => Some(GroupType::CALL),
//            3 => Some(GroupType::RET),
//            4 => Some(GroupType::INT),
//            5 => Some(GroupType::IRET),
//            _ => None,
//        }
//    }
//}

#[derive(Clone)]
pub struct Instruction<'a> {
    pub id: ::std::os::raw::c_uint,
    pub address: u64,
    pub size: u16,
    pub bytes: &'a [u8], //[u8; 16usize],
    pub mnemonic: &'a str,
    pub op_str: &'a str,
    pub groups: Option<&'a [u8]>,
    pub regs_read: Option<&'a [u8]>,
    pub regs_write: Option<&'a [u8]>,
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

        let groups: Option<&[u8]>;
        let regs_read: Option<&[u8]>;
        let regs_write: Option<&[u8]>;
        if let Some(detail) = unsafe { insn.detail.as_ref() } {
            groups = Some(&detail.groups[..detail.groups_count as usize]);
            regs_read = Some(&detail.regs_read[..detail.regs_read_count as usize]);
            regs_write = Some(&detail.regs_write[..detail.regs_write_count as usize]);
        } else {
            groups = None;
            regs_read = None;
            regs_write = None;
        }

        Instruction {
            id: insn.id,
            address: insn.address,
            size: insn.size,
            bytes: &insn.bytes[0..insn.size as usize],
            mnemonic,
            op_str,
            groups,
            regs_read,
            regs_write,
        }
    }
}
