// TODO - remove allow dead code
#![allow(dead_code)]

use std::convert::TryInto;
use std::fmt;
use std::mem;

#[allow(dead_code)]
#[derive(Debug)]
pub enum EiData {
    LittleEndian,
    BigEndian,
}
impl EiData {
    fn from_u8(value: u8) -> EiData {
        match value {
            1 => EiData::LittleEndian,
            2 => EiData::LittleEndian,
            _ => panic!("Invalid ei_data value"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EiClass {
    ELF32,
    ELF64,
}
impl EiClass {
    fn from_u8(value: u8) -> EiClass {
        match value {
            1 => EiClass::ELF32,
            2 => EiClass::ELF64,
            _ => panic!("Invalid ei_class value"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EiOSABI {
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
impl EiOSABI {
    fn from_u8(value: u8) -> EiOSABI {
        match value {
            0x00 => EiOSABI::SystemV,
            0x01 => EiOSABI::HPUX,
            0x02 => EiOSABI::NetBSD,
            0x03 => EiOSABI::Linux,
            0x04 => EiOSABI::GNUHurd,
            0x06 => EiOSABI::Solaris,
            0x07 => EiOSABI::AIX,
            0x08 => EiOSABI::IRIX,
            0x09 => EiOSABI::FreeBSD,
            0x0A => EiOSABI::TRU64,
            0x0B => EiOSABI::NovellModesto,
            0x0C => EiOSABI::OpenBSD,
            0x0D => EiOSABI::OpenVMS,
            0x0E => EiOSABI::NonstopKernel,
            0x0F => EiOSABI::AROS,
            0x10 => EiOSABI::FenixOS,
            0x11 => EiOSABI::CloudABI,
            _    => EiOSABI::Unknown,
        }
    }
}

pub struct ELFIdent {
    pub ei_magic: [u8; 4],
    pub ei_class: EiClass,       // 1 == 32 bit, 2 == 64 bit
    pub ei_data: EiData,        // 1 == little endian, 2 == big endian
    pub ei_version: u8,
    pub ei_os_abi: EiOSABI,
    pub ei_abi_version: u8,
    pub ei_pad: [u8; 7],
}

impl ELFIdent {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T) -> ELFIdent {
        const SIZE: usize = mem::size_of::<ELFIdent>();
        let mut raw_ident: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw_ident).unwrap();

        let mut magic=[0;4];
        magic.copy_from_slice(&raw_ident[0..4]);

        let mut pad=[0;7];
        pad.copy_from_slice(&raw_ident[9..16]);

        let result: ELFIdent = ELFIdent {
            ei_magic:       magic,
            ei_class:       EiClass::from_u8(raw_ident[4]),
            ei_data:        EiData::from_u8(raw_ident[5]),
            ei_version:     raw_ident[6],
            ei_os_abi:      EiOSABI::from_u8(raw_ident[7]),
            ei_abi_version: raw_ident[8],
            ei_pad:         pad,
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
pub enum EType {
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
impl EType {
    fn from_u16(value:u16) -> EType {
        match value {
            0x0000 => EType::NONE,
            0x0001 => EType::REL,
            0x0002 => EType::EXEC,
            0x0003 => EType::DYN,
            0x0004 => EType::CORE,
            0xfe00 => EType::LOOS,
            0xfeff => EType::HIOS,
            0xff00 => EType::LOPROC,
            0xffff => EType::HIPROC,
            _ => EType::UNKNOWN,
        }
    }
}
impl fmt::Display for EType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value: &str = match self {
            EType::NONE => "NONE",
            EType::REL => "Relocatable file (REL)",
            EType::EXEC => "Executable file (EXEC)",
            EType::DYN => "Shared object file (DYN)",
            EType::CORE => "CORE",
            EType::LOOS => "LOOS",
            EType::HIOS => "HIOS",
            EType::LOPROC => "LOPROC",
            EType::HIPROC => "HIPROC",
            EType::UNKNOWN => "Unknown",
        };
        write!(f, "{}", value)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum EMachine {
    None,     // 0x00
    Sparc,    // 0x02
    X86,      // 0x03
    MIPS,     // 0x08
    PowerPC,  // 0x14
    S390,     // 0x16
    ARM,      // 0x28
    SuperH,   // 0x2a
    IA64,     // 0x32
    X86_64,   // 0x3e
    Aarch64,  // 0xb7
    RiscV,    // 0xf3
    Unknown,
}
impl EMachine {
    fn from_u16(value: u16) -> EMachine {
        match value {
            0x0000 => EMachine::None,
            0x0002 => EMachine::Sparc,
            0x0003 => EMachine::X86,
            0x0008 => EMachine::MIPS,
            0x0014 => EMachine::PowerPC,
            0x0016 => EMachine::S390,
            0x0028 => EMachine::ARM,
            0x002a => EMachine::SuperH,
            0x0032 => EMachine::IA64,
            0x003e => EMachine::X86_64,
            0x00b7 => EMachine::Aarch64,
            0x00f3 => EMachine::RiscV,
            _ => EMachine::Unknown,
        }
    }
}

pub struct ELFHeader32 {
    pub e_type: EType,
    pub e_machine: EMachine,
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
        const SIZE: usize = mem::size_of::<ELFHeader32>();
        let mut raw: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw).unwrap();

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u16::from_le_bytes,
            EiData::BigEndian => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u32::from_le_bytes,
            EiData::BigEndian => u32::from_be_bytes,
        };

        // Finally we can create our header
        let result: ELFHeader32 = ELFHeader32 {
            e_type:      EType::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
            e_machine:   EMachine::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
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
        let strings = [
            format!("{:35}{:}",     "Type:", self.e_type),
            format!("{:35}{:?}",    "Machine:", self.e_machine),
            format!("{:35}{:?}",    "Version:", self.e_version),
            format!("{:35}{:?}",    "Entry point address:", self.e_entry),
            format!("{:35}{:?} {}", "Start of program headers:", self.e_phoff, "(bytes into file)"),
            format!("{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)"),
            format!("{:35}{:?}",    "Flags:", self.e_flags),
            format!("{:35}{:?} {}", "Size of this header:", self.e_ehsize, "(bytes)"),
            format!("{:35}{:?} {}", "Size of program headers:", self.e_phentsize, "(bytes)"),
            format!("{:35}{:?}",    "Number of program headers:", self.e_phnum),
            format!("{:35}{:?} {}", "Size of section headers:", self.e_shentsize, "(bytes)"),
            format!("{:35}{:?}",    "Number of section headers:", self.e_shnum),
            format!("{:35}{:?}",    "Section header string table index:", self.e_shstrndx),
        ];
        writeln!(f, "{}", strings.join("\n"))
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
    pub e_type: EType,
    pub e_machine: EMachine,
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
        const SIZE: usize = mem::size_of::<ELFHeader64>();
        let mut raw: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw).unwrap();

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u16::from_le_bytes,
            EiData::BigEndian => u16::from_be_bytes,
        };
        let u32_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u32::from_le_bytes,
            EiData::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u64::from_le_bytes,
            EiData::BigEndian => u64::from_be_bytes,
        };

        // Finally we can create our header
        let result: ELFHeader64 = ELFHeader64 {
            e_type:      EType::from_u16(u16_from_bytes(raw[0..2].try_into().unwrap())),
            e_machine:   EMachine::from_u16(u16_from_bytes(raw[2..4].try_into().unwrap())),
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
        let strings = [
            format!("{:35}{:}",     "Type:", self.e_type),
            format!("{:35}{:?}",    "Machine:", self.e_machine),
            format!("{:35}{:?}",    "Version:", self.e_version),
            format!("{:35}{:?}",    "Entry point address:", self.e_entry),
            format!("{:35}{:?} {}", "Start of program headers:", self.e_phoff, "(bytes into file)"),
            format!("{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)"),
            format!("{:35}{:?}",    "Flags:", self.e_flags),
            format!("{:35}{:?} {}", "Size of this header:", self.e_ehsize, "(bytes)"),
            format!("{:35}{:?} {}", "Size of program headers:", self.e_phentsize, "(bytes)"),
            format!("{:35}{:?}",    "Number of program headers:", self.e_phnum),
            format!("{:35}{:?} {}", "Size of section headers:", self.e_shentsize, "(bytes)"),
            format!("{:35}{:?}",    "Number of section headers:", self.e_shnum),
            format!("{:35}{:?}",    "Section header string table index:", self.e_shstrndx),
        ];
        writeln!(f, "{}", strings.join("\n"))
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


#[allow(dead_code)]
#[derive(Debug)]
pub enum SectionType {
    Null,            // 0x00 Section header table entry unused
    ProgBits,        // 0x01 Program data
    SymTab,          // 0x02 Symbol table
    StrTab,          // 0x03 String table
    Rela,            // 0x04 Relocation entries with addends
    Hash,            // 0x05 Symbol hash table
    Dynamic,         // 0x06 Dynamic linking information
    Note,            // 0x07 Notes
    NoBits,          // 0x08 Program space with no data (bss)
    RelocationEnt,   // 0x09 Relocation entries, no addends
    ShLib,           // 0x0a Reserved
    DynSym,          // 0x0b Dynamic linker symbol table
    InitArray,       // 0x0e Array of constructors
    FiniArray,       // 0x0f Array of destructors
    PreinitArray,    // 0x10 Array of pre-constructors
    Group,           // 0x11 Section group
    SymTabShNdx,     // 0x12 Extended section indices  SYMTAB_SHNDX
    Num,             // 0x13 Number of defined types
    LOOS,            // 0x60000000 Start OS-specific
    Unknown,
}
impl SectionType {
    fn from_u32(value: u32) -> SectionType {
        match value {
            0x00 => SectionType::Null,
            0x01 => SectionType::ProgBits,
            0x02 => SectionType::SymTab,
            0x03 => SectionType::StrTab,
            0x04 => SectionType::Rela,
            0x05 => SectionType::Hash,
            0x06 => SectionType::Dynamic,
            0x07 => SectionType::Note,
            0x08 => SectionType::NoBits,
            0x09 => SectionType::RelocationEnt,
            0x0a => SectionType::ShLib,
            0x0b => SectionType::DynSym,
            0x0e => SectionType::InitArray,
            0x0f => SectionType::FiniArray,
            0x10 => SectionType::PreinitArray,
            0x11 => SectionType::Group,
            0x12 => SectionType::SymTabShNdx,
            0x13 => SectionType::Num,
            0x60000000 => SectionType::LOOS,
            _ => SectionType::Unknown,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SectionFlags {
    Write,              // 0x01 Writable
    Alloc,              // 0x02 Occupies memory during execution
    ExecInstr,          // 0x04 Executable
    Merge,              // 0x10 Might be merged
    Strings,            // 0x20 Contains nul-terminated strings
    InfoLink,          // 0x40 'sh_info' contains SHT index
    LinkOrder,         // 0x80 Preserve order after combining
    OSNonconforming,   // 0x100 Non-standard OS specific handling required
    Group,              // 0x200 Section is member of a group
    TLS,                // 0x400 Section hold thread-local data
    MaskOS,             // 0x0ff00000 OS specific
    MaskProc,           // 0xf0000000 Processor specific
    Ordered,            // 0x40000000 Special ordering requirement (solaris)
    Exclude,            // 0x80000000 Section is excluded unless referenced or allocated (solaris)
    Unknown,            // unknown
}
impl SectionFlags {
//    fn from_u32(value: u32) -> SECTION_FLAGS {
//        SECTION_FLAGS::from_u64(u64::from(value))
//    }
    fn from_u64(value: u64) -> SectionFlags {
        match value {
            0x01 => SectionFlags::Write,
            0x02 => SectionFlags::Alloc,
            0x04 => SectionFlags::ExecInstr,
            0x10 => SectionFlags::Merge,
            0x20 => SectionFlags::Strings,
            0x40 => SectionFlags::InfoLink,
            0x80 => SectionFlags::LinkOrder,
            0x100 => SectionFlags::OSNonconforming,
            0x200 => SectionFlags::Group,
            0x400 => SectionFlags::TLS,
            0x0ff00000 => SectionFlags::MaskOS,
            0xf0000000 => SectionFlags::MaskProc,
            0x40000000 => SectionFlags::Ordered,
            0x80000000 => SectionFlags::Exclude,
            _ => SectionFlags::Unknown,
        }
    }
}

pub struct SectionHeader64 {
    pub name: u32,                   // Offset to string in .shstrtab section
    pub section_type: SectionType,  // Type of this header
    pub flags: SectionFlags,        // Attributes of the section u32 or u64
    pub address: u64,                // Virtual address of section in memory, for sections that are loaded u32 or u64
    pub offset: u64,                 // Offset of the section in file u32 or u64
    pub size: u64,                   // Size in bytes of the section u32 or u64
    pub link: u32,                   // Section index of an associated section
    pub info: u32,                   // Extra info about the section
    pub addralign: u64,              // Required alignment of the section (power of 2) u32 or 64
    pub entsize: u64,                // Size in bytes of each entry for sections that contain fixed size entries, else 0 u32 or 64
}

impl SectionHeader64 {
    pub fn parse_from_buffer<T: std::io::Read>(buffer: &mut T, ident: ELFIdent) -> SectionHeader64 {
        // First get the bytes for our header
        const SIZE: usize = mem::size_of::<SectionHeader64>();
        let mut raw: [u8; SIZE] = [0; SIZE];
        buffer.read_exact(&mut raw).unwrap();

        // Now get our conversion functions to read numbers based on endianness
        let u32_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u32::from_le_bytes,
            EiData::BigEndian => u32::from_be_bytes,
        };
        let u64_from_bytes = match ident.ei_data {
            EiData::LittleEndian => u64::from_le_bytes,
            EiData::BigEndian => u64::from_be_bytes,
        };

        // Finally we can create our header
        let result = SectionHeader64 {
            name:         u32_from_bytes(raw[0..4].try_into().unwrap()),
            section_type: SectionType::from_u32(u32_from_bytes(raw[4..8].try_into().unwrap())),
            flags:        SectionFlags::from_u64(u64_from_bytes(raw[8..16].try_into().unwrap())),
            address:      u64_from_bytes(raw[16..24].try_into().unwrap()),
            offset:       u64_from_bytes(raw[24..32].try_into().unwrap()),
            size:         u64_from_bytes(raw[32..40].try_into().unwrap()),
            link:         u32_from_bytes(raw[40..44].try_into().unwrap()),
            info:         u32_from_bytes(raw[44..48].try_into().unwrap()),
            addralign:    u64_from_bytes(raw[48..56].try_into().unwrap()),
            entsize:      u64_from_bytes(raw[56..64].try_into().unwrap()),
        };

        result
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement section header formatter
        // writeln!(f, "{:35}{:?} {}", "Start of section headers:", self.e_shoff, "(bytes into file)");
        let strings = [
            format!("{:15}{:?}",    "Name Index:",    self.name),
            format!("{:15}{:?}",    "Type:",          self.section_type),
            format!("{:15}{:?}",    "Flags:",         self.flags),
            format!("{:15}{:?}",    "Address:",       self.address),
            format!("{:15}{:?} {}", "Offset:",        self.offset, "(bytes)"),
            format!("{:15}{:?} {}", "Size:",          self.size, "(bytes)"),
            format!("{:15}{:?}",    "Link:",          self.link),
            format!("{:15}{:?}",    "Info:",          self.info),
            format!("{:15}{:?}",    "Address Align:", self.addralign),
            format!("{:15}{:?} {}", "Entity Size",    self.entsize, "(bytes)"),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}

impl fmt::Display for SectionHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

impl fmt::Debug for SectionHeader64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

