pub mod instruction;

use capstone_sys::bindings::cs_detail;
use instruction::GroupType;

// maps to x86_reg
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Register {
    INVALID = 0,
    AH = 1,
    AL = 2,
    AX = 3,
    BH = 4,
    BL = 5,
    BP = 6,
    BPL = 7,
    BX = 8,
    CH = 9,
    CL = 10,
    CS = 11,
    CX = 12,
    DH = 13,
    DI = 14,
    DIL = 15,
    DL = 16,
    DS = 17,
    DX = 18,
    EAX = 19,
    EBP = 20,
    EBX = 21,
    ECX = 22,
    EDI = 23,
    EDX = 24,
    EFLAGS = 25,
    EIP = 26,
    EIZ = 27,
    ES = 28,
    ESI = 29,
    ESP = 30,
    FPSW = 31,
    FS = 32,
    GS = 33,
    IP = 34,
    RAX = 35,
    RBP = 36,
    RBX = 37,
    RCX = 38,
    RDI = 39,
    RDX = 40,
    RIP = 41,
    RIZ = 42,
    RSI = 43,
    RSP = 44,
    SI = 45,
    SIL = 46,
    SP = 47,
    SPL = 48,
    SS = 49,
    CR0 = 50,
    CR1 = 51,
    CR2 = 52,
    CR3 = 53,
    CR4 = 54,
    CR5 = 55,
    CR6 = 56,
    CR7 = 57,
    CR8 = 58,
    CR9 = 59,
    CR10 = 60,
    CR11 = 61,
    CR12 = 62,
    CR13 = 63,
    CR14 = 64,
    CR15 = 65,
    DR0 = 66,
    DR1 = 67,
    DR2 = 68,
    DR3 = 69,
    DR4 = 70,
    DR5 = 71,
    DR6 = 72,
    DR7 = 73,
    FP0 = 74,
    FP1 = 75,
    FP2 = 76,
    FP3 = 77,
    FP4 = 78,
    FP5 = 79,
    FP6 = 80,
    FP7 = 81,
    K0 = 82,
    K1 = 83,
    K2 = 84,
    K3 = 85,
    K4 = 86,
    K5 = 87,
    K6 = 88,
    K7 = 89,
    MM0 = 90,
    MM1 = 91,
    MM2 = 92,
    MM3 = 93,
    MM4 = 94,
    MM5 = 95,
    MM6 = 96,
    MM7 = 97,
    R8 = 98,
    R9 = 99,
    R10 = 100,
    R11 = 101,
    R12 = 102,
    R13 = 103,
    R14 = 104,
    R15 = 105,
    ST0 = 106,
    ST1 = 107,
    ST2 = 108,
    ST3 = 109,
    ST4 = 110,
    ST5 = 111,
    ST6 = 112,
    ST7 = 113,
    XMM0 = 114,
    XMM1 = 115,
    XMM2 = 116,
    XMM3 = 117,
    XMM4 = 118,
    XMM5 = 119,
    XMM6 = 120,
    XMM7 = 121,
    XMM8 = 122,
    XMM9 = 123,
    XMM10 = 124,
    XMM11 = 125,
    XMM12 = 126,
    XMM13 = 127,
    XMM14 = 128,
    XMM15 = 129,
    XMM16 = 130,
    XMM17 = 131,
    XMM18 = 132,
    XMM19 = 133,
    XMM20 = 134,
    XMM21 = 135,
    XMM22 = 136,
    XMM23 = 137,
    XMM24 = 138,
    XMM25 = 139,
    XMM26 = 140,
    XMM27 = 141,
    XMM28 = 142,
    XMM29 = 143,
    XMM30 = 144,
    XMM31 = 145,
    YMM0 = 146,
    YMM1 = 147,
    YMM2 = 148,
    YMM3 = 149,
    YMM4 = 150,
    YMM5 = 151,
    YMM6 = 152,
    YMM7 = 153,
    YMM8 = 154,
    YMM9 = 155,
    YMM10 = 156,
    YMM11 = 157,
    YMM12 = 158,
    YMM13 = 159,
    YMM14 = 160,
    YMM15 = 161,
    YMM16 = 162,
    YMM17 = 163,
    YMM18 = 164,
    YMM19 = 165,
    YMM20 = 166,
    YMM21 = 167,
    YMM22 = 168,
    YMM23 = 169,
    YMM24 = 170,
    YMM25 = 171,
    YMM26 = 172,
    YMM27 = 173,
    YMM28 = 174,
    YMM29 = 175,
    YMM30 = 176,
    YMM31 = 177,
    ZMM0 = 178,
    ZMM1 = 179,
    ZMM2 = 180,
    ZMM3 = 181,
    ZMM4 = 182,
    ZMM5 = 183,
    ZMM6 = 184,
    ZMM7 = 185,
    ZMM8 = 186,
    ZMM9 = 187,
    ZMM10 = 188,
    ZMM11 = 189,
    ZMM12 = 190,
    ZMM13 = 191,
    ZMM14 = 192,
    ZMM15 = 193,
    ZMM16 = 194,
    ZMM17 = 195,
    ZMM18 = 196,
    ZMM19 = 197,
    ZMM20 = 198,
    ZMM21 = 199,
    ZMM22 = 200,
    ZMM23 = 201,
    ZMM24 = 202,
    ZMM25 = 203,
    ZMM26 = 204,
    ZMM27 = 205,
    ZMM28 = 206,
    ZMM29 = 207,
    ZMM30 = 208,
    ZMM31 = 209,
    R8B = 210,
    R9B = 211,
    R10B = 212,
    R11B = 213,
    R12B = 214,
    R13B = 215,
    R14B = 216,
    R15B = 217,
    R8D = 218,
    R9D = 219,
    R10D = 220,
    R11D = 221,
    R12D = 222,
    R13D = 223,
    R14D = 224,
    R15D = 225,
    R8W = 226,
    R9W = 227,
    R10W = 228,
    R11W = 229,
    R12W = 230,
    R13W = 231,
    R14W = 232,
    R15W = 233,
    ENDING = 234,
}

#[derive(Clone)]
pub struct Detail {
    //    pub regs_read: [u8; 12usize],
    //    pub regs_read_count: u8,
    //    pub regs_write: [u8; 20usize],
    //    pub regs_write_count: u8,
    pub groups: Vec<GroupType>,
    //    pub __bindgen_anon_1: cs_detail__bindgen_ty_1,
}
impl Detail {
    pub fn create_from_cs_detail(detail: &cs_detail) -> Detail {
        /* NOTE TO SELF:
        For arch specific stuff here, can probably just use macros to obtain functions that will
        convert groups or register numbers to something more generally useful than ints
        */
        // first get groups

        // TODO: this doesn't do anything with arch specific groups yet
        let mut groups: Vec<GroupType> = vec![];
        for i in 0..detail.groups_count as usize {
            match <GroupType>::from_u8(detail.groups[i]) {
                Some(v) => groups.push(v),
                None => (),
            };
        }
        Detail {
            //            regs_read: detail.regs_read,
            //            regs_read_count: detail.regs_read_count,
            //            regs_write: detail.regs_write,
            //            regs_write_count: detail.regs_write_count,
            groups: groups,
        }
    }
}