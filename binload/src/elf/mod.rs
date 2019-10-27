use std::convert::TryInto;
use std::fmt;
use std::io::Read;
use std::mem;
use std::slice;


#[derive(Debug)]
pub enum EI_DATA {
    LITTLE_ENDIAN,
    BIG_ENDIAN,
}
impl EI_DATA {
    fn from_u8(value: u8) -> EI_DATA {
        match value {
            1 => EI_DATA::LITTLE_ENDIAN,
            2 => EI_DATA::LITTLE_ENDIAN,
            _ => panic!("Invalid ei_data value"),
        }
    }
}

#[derive(Debug)]
pub enum EI_CLASS {
    ELF32,
    ELF64,
}
impl EI_CLASS {
    fn from_u8(value: u8) -> EI_CLASS {
        match value {
            1 => EI_CLASS::ELF32,
            2 => EI_CLASS::ELF64,
            _ => panic!("Invalid ei_class value"),
        }
    }
}

#[derive(Debug)]
pub enum EI_OSABI {
    SYSTEM_V,
    HP_UX,
    NET_BSD,
    LINUX,
    GNU_HURD,
    SOLARIS,
    AIX,
    IRIX,
    FREE_BSD,
    TRU_64,
    NOVELL_MODESTO,
    OPEN_BSD,
    OPEN_VMS,
    NONSTOP_KERNEL,
    AROS,
    FENIX_OS,
    CLOUD_ABI,
    UNKNOWN,
}
impl EI_OSABI {
    fn from_u8(value: u8) -> EI_OSABI {
        match value {
            0x00 => EI_OSABI::SYSTEM_V,
            0x01 => EI_OSABI::HP_UX,
            0x02 => EI_OSABI::NET_BSD,
            0x03 => EI_OSABI::LINUX,
            0x04 => EI_OSABI::GNU_HURD,
            0x06 => EI_OSABI::SOLARIS,
            0x07 => EI_OSABI::AIX,
            0x08 => EI_OSABI::IRIX,
            0x09 => EI_OSABI::FREE_BSD,
            0x0A => EI_OSABI::TRU_64,
            0x0B => EI_OSABI::NOVELL_MODESTO,
            0x0C => EI_OSABI::OPEN_BSD,
            0x0D => EI_OSABI::OPEN_VMS,
            0x0E => EI_OSABI::NONSTOP_KERNEL,
            0x0F => EI_OSABI::AROS,
            0x10 => EI_OSABI::FENIX_OS,
            0x11 => EI_OSABI::CLOUD_ABI,
            _    => EI_OSABI::UNKNOWN,
        }
    }
}

pub struct ELFIdent {
    pub ei_magic: [u8; 4],
    pub ei_class: EI_CLASS,       // 1 == 32 bit, 2 == 64 bit
    pub ei_data: EI_DATA,        // 1 == little endian, 2 == big endian
    pub ei_version: u8,
    pub ei_os_abi: EI_OSABI,
    pub ei_abi_version: u8,
    pub ei_pad: [u8; 7],
}

impl ELFIdent {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T) -> ELFIdent {
        const size: usize = mem::size_of::<ELFIdent>();
        let mut raw_ident: [u8; size] = [0; size];
        buffer.read_exact(&mut raw_ident);

        let mut magic=[0;4];
        magic.copy_from_slice(&raw_ident[0..4]);

        let mut pad=[0;7];
        pad.copy_from_slice(&raw_ident[9..16]);

        let mut result: ELFIdent = ELFIdent {
            ei_magic:       magic,
            ei_class:       EI_CLASS::from_u8(raw_ident[4]),
            ei_data:        EI_DATA::from_u8(raw_ident[5]),
            ei_version:     raw_ident[6],
            ei_os_abi:      EI_OSABI::from_u8(raw_ident[7]),
            ei_abi_version: raw_ident[8],
            ei_pad:         pad,
        };
        // TODO: validate magic
        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = "";
        writeln!(f, "{:15}{:x?}", "Magic:", self.ei_magic);
        writeln!(f, "{:15}{:x?}", "Class:", self.ei_class);
        writeln!(f, "{:15}{:x?}", "Data:", self.ei_data);
        writeln!(f, "{:15}{:x?}", "Version:", self.ei_version);
        writeln!(f, "{:15}{:x?}", "OS ABI:", self.ei_os_abi);
        writeln!(f, "{:15}{:x?}", "ABI Version:", self.ei_abi_version);
        writeln!(f, "{:15}{:x?}", "Pad:", self.ei_pad)
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

#[derive(Debug)]
pub enum E_TYPE {
    NONE,      // 0x00
    REL,       // 0x01
    EXEC,      // 0x02
    DYN,       // 0x03
    CORE,      // 0x04
    LOOS,      // 0xfe00
    HIOS,      // 0xfeff
    LOPROC,    // 0xff00
    HIPROC,    // 0xffff
    UNKNOWN,
}
impl E_TYPE {
    fn from_u16(value:u16) -> E_TYPE {
        match value {
            0x0000 => E_TYPE::NONE,
            0x0001 => E_TYPE::REL,
            0x0002 => E_TYPE::EXEC,
            0x0003 => E_TYPE::DYN,
            0x0004 => E_TYPE::CORE,
            0xfe00 => E_TYPE::LOOS,
            0xfeff => E_TYPE::HIOS,
            0xff00 => E_TYPE::LOPROC,
            0xffff => E_TYPE::HIPROC,
            _ => E_TYPE::UNKNOWN,
        }
    }
}
impl fmt::Display for E_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            E_TYPE::NONE => "NONE",
            E_TYPE::REL => "Relocatable file (REL)",
            E_TYPE::EXEC => "Executable file (EXEC)",
            E_TYPE::DYN => "Shared object file (DYN)",
            E_TYPE::CORE => "CORE",
            E_TYPE::LOOS => "LOOS",
            E_TYPE::HIOS => "HIOS",
            E_TYPE::LOPROC => "LOPROC",
            E_TYPE::HIPROC => "HIPROC",
            E_TYPE::UNKNOWN => "Unknown",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub enum E_MACHINE {
    NONE,      // 0x00
    SPARC,     // 0x02
    X86,       // 0x03
    MIPS,      // 0x08
    POWER_PC,  // 0x14
    S390,      // 0x16
    ARM,       // 0x28
    SUPER_H,   // 0x2a
    IA_64,     // 0x32
    X86_64,    // 0x3e
    AARCH_64,  // 0xb7
    RISC_V,    // 0xf3
    UNKNOWN,
}
impl E_MACHINE {
    fn from_u16(value: u16) -> E_MACHINE {
        match value {
            0x0000 => E_MACHINE::NONE,
            0x0002 => E_MACHINE::SPARC,
            0x0003 => E_MACHINE::X86,
            0x0008 => E_MACHINE::MIPS,
            0x0014 => E_MACHINE::POWER_PC,
            0x0016 => E_MACHINE::S390,
            0x0028 => E_MACHINE::ARM,
            0x002a => E_MACHINE::SUPER_H,
            0x0032 => E_MACHINE::IA_64,
            0x003e => E_MACHINE::X86_64,
            0x00b7 => E_MACHINE::AARCH_64,
            0x00f3 => E_MACHINE::RISC_V,
            _ => E_MACHINE::UNKNOWN,
        }
    }
}

pub struct ELFHeader32 {
    pub e_type: E_TYPE,
    pub e_machine: E_MACHINE,
    pub e_version: u32,
    pub e_entry: u32, // class specific field
    pub e_phoff: u32, // class specific field
    pub e_shoff: u32, // class specific field
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}


impl ELFHeader32 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> ELFHeader32 {
        // First get the bytes for our header
        const size: usize = mem::size_of::<ELFHeader32>();
        let mut raw: [u8; size] = [0; size];
        buffer.read_exact(&mut raw);

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u16::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u32::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u32::from_be_bytes,
        };

        // Finally we can create our header
        let mut result: ELFHeader32 = ELFHeader32 {
            e_type:      E_TYPE::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
            e_machine:   E_MACHINE::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
            e_version:   u32_from_bytes(raw[4..8].try_into().unwrap()),
            e_entry:     u32_from_bytes(raw[8..12].try_into().unwrap()),
            e_phoff:     u32_from_bytes(raw[12..16].try_into().unwrap()),
            e_shoff:     u32_from_bytes(raw[16..20].try_into().unwrap()),
            e_flags:     u32_from_bytes(raw[20..24].try_into().unwrap()),
            e_ehsize:    u16_from_bytes(raw[24..26].try_into().unwrap()),
            e_phentsize: u16_from_bytes(raw[26..28].try_into().unwrap()),
            e_phnum:     u16_from_bytes(raw[28..30].try_into().unwrap()),
            e_shentsize: u16_from_bytes(raw[30..32].try_into().unwrap()),
            e_shnum:     u16_from_bytes(raw[32..34].try_into().unwrap()),
            e_shstrndx:  u16_from_bytes(raw[34..36].try_into().unwrap()),
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:35}{:}",     "Type:", self.e_type);
        writeln!(f, "{:35}{:?}",    "Machine:", self.e_machine);
        writeln!(f, "{:35}{:?}",    "Version:", self.e_version);
        writeln!(f, "{:35}{:?}",    "Entry point address:", self.e_entry);
        writeln!(f, "{:35}{:?} {}", "Start of program headers:", self.e_phoff, "(bytes into file)");
        writeln!(f, "{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)");
        writeln!(f, "{:35}{:?}",    "Flags:", self.e_flags);
        writeln!(f, "{:35}{:?} {}", "Size of this header:", self.e_ehsize, "(bytes)");
        writeln!(f, "{:35}{:?} {}", "Size of program headers:", self.e_phentsize, "(bytes)");
        writeln!(f, "{:35}{:?}",    "Number of program headers:", self.e_phnum);
        writeln!(f, "{:35}{:?} {}", "Size of section headers:", self.e_shentsize, "(bytes)");
        writeln!(f, "{:35}{:?}",    "Number of section headers:", self.e_shnum);
        writeln!(f, "{:35}{:?}",    "Section header string table index:", self.e_shstrndx)
    }
}

impl fmt::Display for ELFHeader32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for ELFHeader32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}


pub struct ELFHeader64 {
    pub e_type: E_TYPE,
    pub e_machine: E_MACHINE,
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

impl ELFHeader64 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> ELFHeader64 {
        // First get the bytes for our header
        const size: usize = mem::size_of::<ELFHeader64>();
        let mut raw: [u8; size] = [0; size];
        buffer.read_exact(&mut raw);

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u16::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u32::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EI_DATA::LITTLE_ENDIAN => u64::from_le_bytes,
            EI_DATA::BIG_ENDIAN => u64::from_be_bytes,
        };

        // Finally we can create our header
        let mut result: ELFHeader64 = ELFHeader64 {
            e_type:      E_TYPE::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
            e_machine:   E_MACHINE::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
            e_version:   u32_from_bytes(raw[4..8].try_into().unwrap()),
            e_entry:     u64_from_bytes(raw[8..16].try_into().unwrap()),
            e_phoff:     u64_from_bytes(raw[16..24].try_into().unwrap()),
            e_shoff:     u64_from_bytes(raw[24..32].try_into().unwrap()),
            e_flags:     u32_from_bytes(raw[32..36].try_into().unwrap()),
            e_ehsize:    u16_from_bytes(raw[36..38].try_into().unwrap()),
            e_phentsize: u16_from_bytes(raw[38..40].try_into().unwrap()),
            e_phnum:     u16_from_bytes(raw[40..42].try_into().unwrap()),
            e_shentsize: u16_from_bytes(raw[42..44].try_into().unwrap()),
            e_shnum:     u16_from_bytes(raw[44..46].try_into().unwrap()),
            e_shstrndx:  u16_from_bytes(raw[46..48].try_into().unwrap()),
        };

        result
    }


    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:35}{:}",     "Type:", self.e_type);
        writeln!(f, "{:35}{:?}",    "Machine:", self.e_machine);
        writeln!(f, "{:35}{:?}",    "Version:", self.e_version);
        writeln!(f, "{:35}{:?}",    "Entry point address:", self.e_entry);
        writeln!(f, "{:35}{:?} {}", "Start of program headers:", self.e_phoff, "(bytes into file)");
        writeln!(f, "{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)");
        writeln!(f, "{:35}{:?}",    "Flags:", self.e_flags);
        writeln!(f, "{:35}{:?} {}", "Size of this header:", self.e_ehsize, "(bytes)");
        writeln!(f, "{:35}{:?} {}", "Size of program headers:", self.e_phentsize, "(bytes)");
        writeln!(f, "{:35}{:?}",    "Number of program headers:", self.e_phnum);
        writeln!(f, "{:35}{:?} {}", "Size of section headers:", self.e_shentsize, "(bytes)");
        writeln!(f, "{:35}{:?}",    "Number of section headers:", self.e_shnum);
        writeln!(f, "{:35}{:?}",    "Section header string table index:", self.e_shstrndx)
    }
}

impl fmt::Display for ELFHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for ELFHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

