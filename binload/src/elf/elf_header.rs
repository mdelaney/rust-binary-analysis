// TODO - remove allow dead code
#![allow(non_camel_case_types)]

use std::convert::TryInto;
use std::fmt;
use std::mem;

#[allow(dead_code)]
#[derive(Debug)]
pub enum EI_Data {
    LittleEndian,
    BigEndian,
}
impl EI_Data {
    fn from_u8(value: u8) -> EI_Data {
        match value {
            1 => EI_Data::LittleEndian,
            2 => EI_Data::LittleEndian,
            _ => panic!("Invalid ei_data value"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EI_Class {
    ELF32,
    ELF64,
}
impl EI_Class {
    fn from_u8(value: u8) -> EI_Class {
        match value {
            1 => EI_Class::ELF32,
            2 => EI_Class::ELF64,
            _ => panic!("Invalid ei_class value"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EI_OSABI {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    TRU64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonstopKernel,
    AROS,
    FenixOS,
    CloudABI,
    ARM_AEABI,
    ARM,
    Standalone,
    Unknown,
}
impl EI_OSABI {
    fn from_u8(value: u8) -> EI_OSABI {
        match value {
            0x00 => EI_OSABI::SystemV,
            0x01 => EI_OSABI::HPUX,
            0x02 => EI_OSABI::NetBSD,
            0x03 => EI_OSABI::Linux,
            0x04 => EI_OSABI::GNUHurd,
            0x06 => EI_OSABI::Solaris,
            0x07 => EI_OSABI::AIX,
            0x08 => EI_OSABI::IRIX,
            0x09 => EI_OSABI::FreeBSD,
            0x0A => EI_OSABI::TRU64,
            0x0B => EI_OSABI::NovellModesto,
            0x0C => EI_OSABI::OpenBSD,
            0x0D => EI_OSABI::OpenVMS,
            0x0E => EI_OSABI::NonstopKernel,
            0x0F => EI_OSABI::AROS,
            0x10 => EI_OSABI::FenixOS,
            0x11 => EI_OSABI::CloudABI,
            0x40 => EI_OSABI::ARM_AEABI,
            0x61 => EI_OSABI::ARM,
            0xff => EI_OSABI::Standalone,
            _ => EI_OSABI::Unknown,
        }
    }
}

pub struct ELFIdent {
    pub ei_magic: [u8; 4],
    pub ei_class: EI_Class, // 1 == 32 bit, 2 == 64 bit
    pub ei_data: EI_Data,   // 1 == little endian, 2 == big endian
    pub ei_version: u8,
    pub ei_os_abi: EI_OSABI,
    pub ei_abi_version: u8,
    pub ei_pad: [u8; 7],
}

impl ELFIdent {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T) -> ELFIdent {
        const SIZE: usize = mem::size_of::<ELFIdent>();
        let mut raw_ident: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw_ident).unwrap();

        let mut magic = [0; 4];
        magic.copy_from_slice(&raw_ident[0..4]);

        let mut pad = [0; 7];
        pad.copy_from_slice(&raw_ident[9..16]);

        let result: ELFIdent = ELFIdent {
            ei_magic: magic,
            ei_class: EI_Class::from_u8(raw_ident[4]),
            ei_data: EI_Data::from_u8(raw_ident[5]),
            ei_version: raw_ident[6],
            ei_os_abi: EI_OSABI::from_u8(raw_ident[7]),
            ei_abi_version: raw_ident[8],
            ei_pad: pad,
        };
        // TODO: validate magic
        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:x?}", "Magic:", self.ei_magic),
            format!("{:15}{:x?}", "Class:", self.ei_class),
            format!("{:15}{:x?}", "Data:", self.ei_data),
            format!("{:15}{:x?}", "Version:", self.ei_version),
            format!("{:15}{:x?}", "OS ABI:", self.ei_os_abi),
            format!("{:15}{:x?}", "ABI Version:", self.ei_abi_version),
            format!("{:15}{:x?}", "Pad:", self.ei_pad),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for ELFIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for ELFIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum E_Type {
    NONE, // 0x00
    REL,  // 0x01
    EXEC, // 0x02
    DYN,  // 0x03
    CORE, // 0x04
    NUM,  // 0x05
    OS,   // 0xfe00 to 0xfeff
    PROC, // 0xff00 to 0xffff
    UNKNOWN,
}
impl E_Type {
    fn from_u16(value: u16) -> E_Type {
        match value {
            0x0000 => E_Type::NONE,
            0x0001 => E_Type::REL,
            0x0002 => E_Type::EXEC,
            0x0003 => E_Type::DYN,
            0x0004 => E_Type::CORE,
            0x0005 => E_Type::NUM,
            0xfe00..=0xfeff => E_Type::OS,
            0xff00..=0xffff => E_Type::PROC,
            _ => E_Type::UNKNOWN,
        }
    }
}
impl fmt::Display for E_Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            E_Type::NONE => "NONE",
            E_Type::REL => "Relocatable file (REL)",
            E_Type::EXEC => "Executable file (EXEC)",
            E_Type::DYN => "Shared object file (DYN)",
            E_Type::CORE => "CORE",
            E_Type::NUM => "NUM",
            E_Type::OS => "OS specific",
            E_Type::PROC => "Processor specific",
            E_Type::UNKNOWN => "Unknown",
        };
        write!(f, "{}", value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum E_Machine {
    None,               // 0x00 No Machine
    M32,                // 0x01 AT&T WE 32100
    Sparc,              // 0x02 Sun Sparc
    X86,                // 0x03 Intel 80386
    Motorola_m68K,      // 0x04 Motorola m68k family
    Motorola_m88K,      // 0x05 Motorola m88k family
    Intel_MCU,          // 0x06 Intel MCU
    Intel_860,          // 0x07 Intel 80860
    MIPS_R3000_BE,      // 0x08 MIPS R3000 big-endian
    IBM_System_370,     // 0x09 IBM System 370
    MIPS_R3000_LE,      // 0x0a MIPS R3000 little-endian
    HPPA,               // 0x0f HPPA
    Fujitsu_VPP500,     // 0x11 Fujitsu VPP500
    SunV8Plus,          // 0x12 Sun's "v8plus"
    Intel_80960,        // 0x13 Intel 80960
    PowerPC,            // 0x14 Power PC
    PowerPC_64,         // 0x15 PowerPC 64-bit
    IBM_S390,           // 0x16 IBM S390
    IBM_SPU,            // 0x17 IBM SPU/SPC
    NEC_V800,           // 0x24 NEC V800 series
    Fujitsu_FR20,       // 0x25 Fujitsu FR20
    TRW_RH32,           // 0x26 TRW RH-32
    Motorola_RCE,       // 0x27 Motorola RCE
    ARM,                // 0x28 ARM
    DigitalAlpha,       // 0x29 Digital Alpha
    HitachiSuperH,      // 0x2a Hitachi SH
    SPARC_V9,           // 0x2b SPARC v9 64-bit
    Tricore,            // 0x2c Siemens Tricore
    ARC,                // 0x2d Argonaut RISC Core
    Hitachi_H8_300,     // 0x2e Hitachi H8/300
    Hitachi_H8_300H,    // 0x2f Hitachi H8/300H
    Hitachi_H8S,        // 0x30 Hitachi H8S
    Hitachi_H8_500,     // 0x31 Hitachi H8/500
    IA_64,              // 0x32 Intel Merced
    MIPS_X,             // 0x33 Stanford MIPS-X
    Motorola_Coldfire,  // 0x34 Motorola Coldfire
    Motorola_68HC12,    // 0x35 Motorola M68HC12
    Fujitsu_MMA,        // 0x36 Fujitsu MMA Multimedia Accelerator
    Siemens_PCP,        // 0x37 Siemens PCP
    Sony_nCPU,          // 0x38 Sony nCPU embeeded RISC
    Denso_NDR1,         // 0x39 Denso NDR1 microprocessor
    Motorola_StartCore, // 0x3a Motorola Start*Core processor
    Toyota_ME16,        // 0x3b Toyota ME16 processor
    STM_ST100,          // 0x3c STMicroelectronic ST100 processor
    TinyJ,              // 0x3d Advanced Logic Corp. Tinyj emb.fam
    X86_64,             // 0x3e AMD x86-64 architecture
    Sony_PDSP,          // 0x3f Sony DSP Processor
    Digital_PDP10,      // 0x40 Digital PDP-10
    // TODO
    Aarch64,  // 0xb7
    RiscV,    // 0xf3 Risk-V
    LinuxBPF, // 0xf7 Linux BPF -- in-kernel virtual machine
    C_SKY,    // 0xfc C-SKY
    Unknown,
}

impl E_Machine {
    fn from_u16(value: u16) -> E_Machine {
        match value {
            0x0000 => E_Machine::None,
            0x0001 => E_Machine::M32,
            0x0002 => E_Machine::Sparc,
            0x0003 => E_Machine::X86,
            0x0004 => E_Machine::Motorola_m68K,
            0x0005 => E_Machine::Motorola_m88K,
            0x0006 => E_Machine::Intel_MCU,
            0x0007 => E_Machine::Intel_860,
            0x0008 => E_Machine::MIPS_R3000_BE,
            0x0009 => E_Machine::IBM_System_370,
            0x000a => E_Machine::MIPS_R3000_LE,
            0x000f => E_Machine::HPPA,
            0x0011 => E_Machine::Fujitsu_VPP500,
            0x0012 => E_Machine::SunV8Plus,
            0x0013 => E_Machine::Intel_80960,
            0x0014 => E_Machine::PowerPC,
            0x0015 => E_Machine::PowerPC_64,
            0x0016 => E_Machine::IBM_S390,
            0x0017 => E_Machine::IBM_SPU,
            0x0024 => E_Machine::NEC_V800,
            0x0025 => E_Machine::Fujitsu_FR20,
            0x0026 => E_Machine::TRW_RH32,
            0x0027 => E_Machine::Motorola_RCE,
            0x0028 => E_Machine::ARM,
            0x0029 => E_Machine::DigitalAlpha,
            0x002a => E_Machine::HitachiSuperH,
            0x002b => E_Machine::SPARC_V9,
            0x002c => E_Machine::Tricore,
            0x002d => E_Machine::ARC,
            0x002e => E_Machine::Hitachi_H8_300,
            0x002f => E_Machine::Hitachi_H8_300H,
            0x0030 => E_Machine::Hitachi_H8S,
            0x0031 => E_Machine::Hitachi_H8_500,
            0x0032 => E_Machine::IA_64,
            0x0033 => E_Machine::MIPS_X,
            0x0034 => E_Machine::Motorola_Coldfire,
            0x0035 => E_Machine::Motorola_68HC12,
            0x0036 => E_Machine::Fujitsu_MMA,
            0x0037 => E_Machine::Siemens_PCP,
            0x0038 => E_Machine::Sony_nCPU,
            0x0039 => E_Machine::Denso_NDR1,
            0x003a => E_Machine::Motorola_StartCore,
            0x003b => E_Machine::Toyota_ME16,
            0x003c => E_Machine::STM_ST100,
            0x003d => E_Machine::TinyJ,
            0x003e => E_Machine::X86_64,
            0x003f => E_Machine::Sony_PDSP,
            0x0040 => E_Machine::Digital_PDP10,
            0x00b7 => E_Machine::Aarch64,
            0x00f3 => E_Machine::RiscV,
            0x00f7 => E_Machine::LinuxBPF,
            0x00fc => E_Machine::C_SKY,
            _ => E_Machine::Unknown,
        }
    }
}

pub struct ELFHeader {
    pub ident: ELFIdent,
    pub e_type: E_Type,
    pub e_machine: E_Machine,
    pub e_version: u32,
    pub e_entry: u64, // class specific field
    pub e_phoff: u64, // class specific field
    pub e_shoff: u64, // class specific field
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ELFHeader {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> ELFHeader {
        // First get the bytes for our header
        const SIZE: usize = mem::size_of::<ELFHeader>();
        let mut raw: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw).unwrap();

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EI_Data::LittleEndian => u16::from_le_bytes,
            EI_Data::BigEndian => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EI_Data::LittleEndian => u32::from_le_bytes,
            EI_Data::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EI_Data::LittleEndian => u64::from_le_bytes,
            EI_Data::BigEndian => u64::from_be_bytes,
        };

        // Finally we can create our header
        // We use 64bit values here as we can avoid duplicating everything for 32bit
        // files (given the numeric conversion is lossless)
        let result: ELFHeader = match ident.ei_class {
            EI_Class::ELF32 => ELFHeader {
                ident: ident,
                e_type: E_Type::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
                e_machine: E_Machine::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
                e_version: u32_from_bytes(raw[4..8].try_into().unwrap()),
                e_entry: u64::from(u32_from_bytes(raw[8..12].try_into().unwrap())),
                e_phoff: u64::from(u32_from_bytes(raw[12..16].try_into().unwrap())),
                e_shoff: u64::from(u32_from_bytes(raw[16..20].try_into().unwrap())),
                e_flags: u32_from_bytes(raw[20..24].try_into().unwrap()),
                e_ehsize: u16_from_bytes(raw[24..26].try_into().unwrap()),
                e_phentsize: u16_from_bytes(raw[26..28].try_into().unwrap()),
                e_phnum: u16_from_bytes(raw[28..30].try_into().unwrap()),
                e_shentsize: u16_from_bytes(raw[30..32].try_into().unwrap()),
                e_shnum: u16_from_bytes(raw[32..34].try_into().unwrap()),
                e_shstrndx: u16_from_bytes(raw[34..36].try_into().unwrap()),
            },
            EI_Class::ELF64 => ELFHeader {
                ident: ident,
                e_type: E_Type::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
                e_machine: E_Machine::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
                e_version: u32_from_bytes(raw[4..8].try_into().unwrap()),
                e_entry: u64_from_bytes(raw[8..16].try_into().unwrap()),
                e_phoff: u64_from_bytes(raw[16..24].try_into().unwrap()),
                e_shoff: u64_from_bytes(raw[24..32].try_into().unwrap()),
                e_flags: u32_from_bytes(raw[32..36].try_into().unwrap()),
                e_ehsize: u16_from_bytes(raw[36..38].try_into().unwrap()),
                e_phentsize: u16_from_bytes(raw[38..40].try_into().unwrap()),
                e_phnum: u16_from_bytes(raw[40..42].try_into().unwrap()),
                e_shentsize: u16_from_bytes(raw[42..44].try_into().unwrap()),
                e_shnum: u16_from_bytes(raw[44..46].try_into().unwrap()),
                e_shstrndx: u16_from_bytes(raw[46..48].try_into().unwrap()),
            },
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:35}{:}", "Type:", self.e_type),
            format!("{:35}{:?}", "Machine:", self.e_machine),
            format!("{:35}{:?}", "Version:", self.e_version),
            format!("{:35}{:?}", "Entry point address:", self.e_entry),
            format!(
                "{:35}{:?} {}",
                "Start of program headers:", self.e_phoff, "(bytes into file)"
            ),
            format!(
                "{:35}{:?} {}",
                "Start of section headers:", self.e_shoff, "(bytes into file)"
            ),
            format!("{:35}{:?}", "Flags:", self.e_flags),
            format!(
                "{:35}{:?} {}",
                "Size of this header:", self.e_ehsize, "(bytes)"
            ),
            format!(
                "{:35}{:?} {}",
                "Size of program headers:", self.e_phentsize, "(bytes)"
            ),
            format!("{:35}{:?}", "Number of program headers:", self.e_phnum),
            format!(
                "{:35}{:?} {}",
                "Size of section headers:", self.e_shentsize, "(bytes)"
            ),
            format!("{:35}{:?}", "Number of section headers:", self.e_shnum),
            format!(
                "{:35}{:?}",
                "Section header string table index:", self.e_shstrndx
            ),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for ELFHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for ELFHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
