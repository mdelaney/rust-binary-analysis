// TODO - remove allow dead code
#![allow(dead_code)]
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
    NONE,   // 0x00
    REL,    // 0x01
    EXEC,   // 0x02
    DYN,    // 0x03
    CORE,   // 0x04
    LOOS,   // 0xfe00
    HIOS,   // 0xfeff
    LOPROC, // 0xff00
    HIPROC, // 0xffff
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
            0xfe00 => E_Type::LOOS,
            0xfeff => E_Type::HIOS,
            0xff00 => E_Type::LOPROC,
            0xffff => E_Type::HIPROC,
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
            E_Type::LOOS => "LOOS",
            E_Type::HIOS => "HIOS",
            E_Type::LOPROC => "LOPROC",
            E_Type::HIPROC => "HIPROC",
            E_Type::UNKNOWN => "Unknown",
        };
        write!(f, "{}", value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum E_Machine {
    None,    // 0x00
    Sparc,   // 0x02
    X86,     // 0x03
    MIPS,    // 0x08
    PowerPC, // 0x14
    S390,    // 0x16
    ARM,     // 0x28
    SuperH,  // 0x2a
    IA64,    // 0x32
    X86_64,  // 0x3e
    Aarch64, // 0xb7
    RiscV,   // 0xf3
    Unknown,
}
impl E_Machine {
    fn from_u16(value: u16) -> E_Machine {
        match value {
            0x0000 => E_Machine::None,
            0x0002 => E_Machine::Sparc,
            0x0003 => E_Machine::X86,
            0x0008 => E_Machine::MIPS,
            0x0014 => E_Machine::PowerPC,
            0x0016 => E_Machine::S390,
            0x0028 => E_Machine::ARM,
            0x002a => E_Machine::SuperH,
            0x0032 => E_Machine::IA64,
            0x003e => E_Machine::X86_64,
            0x00b7 => E_Machine::Aarch64,
            0x00f3 => E_Machine::RiscV,
            _ => E_Machine::Unknown,
        }
    }
}

pub struct ELFHeader {
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
