#![macro_use]
extern crate bitflags;
extern crate enum_primitive;

use bitflags::bitflags;
use enum_primitive::enum_from_primitive;
use enum_primitive::enum_from_primitive_impl;
use enum_primitive::enum_from_primitive_impl_ty;
use enum_primitive::FromPrimitive;
use std::convert::TryInto;
use std::fmt;
use std::result::Result::Err;

// This makes it easy to get the function to convert from bytes to a number type
// with a specified endianness
// use like:
//   let u32_from_bytes = get_num_from_bytes!(u32, ident.ei_data)
macro_rules! get_num_from_bytes {
    ( $size:ident, $x:expr ) => {
        match $x {
            Endian::LittleEndian => $size::from_le_bytes,
            Endian::BigEndian => $size::from_be_bytes,
        }
    };
}

#[derive(Debug, Eq, PartialEq)]
pub enum Endian {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArchSize {
    _32,
    _64,
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuType {
    Any = 0xffffffff,
    VAX = 0x00000001,
    MC680 = 0x00000006,
    X86 = 0x00000007,
    X86_64 = 0x01000007,
    MC98000 = 0x0000000a,
    HPPA = 0x0000000b,
    ARM = 0x0000000c,
    ARM64 = 0x0100000c,
    ARM64_32 = 0x0200000c,
    MC88000 = 0x0000000d,
    Sparc = 0x0000000e,
    I860 = 0x0000000f,
    PowerPC = 0x00000012,
    PowerPC64 = 0x01000012,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeVAX {
    VAX_ALL = 0,
    VAX780  = 1,
    VAX785  = 2,
    VAX750  = 3,
    VAX730  = 4,
    UVAXI   = 5,
    UVAXII  = 6,
    VAX8200 = 7,
    VAX8500 = 8,
    VAX8600 = 9,
    VAX8650 = 10,
    VAX8800 = 11,
    UVAXIII = 12,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeMC680 {
    MC680x0_ALL  = 1,
    MC68040      = 2,
    MC68030_ONLY = 3,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeIntel {
    INTEL_MODEL_ALL   = 0,
    I386              = 3 + (0<<4),
    I486              = 4 + (0<<4),
    I486SX            = 4 + (8<<4),
    PENT              = 5 + (0<<4),
    PENTPRO           = 6 + (1<<4),
    PENTII_M3         = 6 + (3<<4),
    PENTII_M5         = 6 + (5<<4),
    CELERON           = 7 + (6<<4),
    CELERON_MOBILE    = 7 + (7<<4),
    PENTIUM_3         = 8 + (0<<4),
    PENTIUM_3_M       = 8 + (1<<4),
    PENTIUM_3_XEON    = 8 + (2<<4),
    PENTIUM_M         = 9 + (0<<4),
    PENTIUM_4         = 10 + (0<<4),
    PENTIUM_4_M       = 10 + (1<<4),
    ITANIUM           = 11 + (0<<4),
    ITANIUM_2         = 11 + (1<<4),
    XEON              = 12 + (0<<4),
    XEON_MP           = 12 + (1<<4),
    INTEL_FAMILY_MAX  = 15,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeX86 {
    X86_ALL    = 3,
    X86_ARCH1  = 4,
    X86_64_H   = 8, /* Haswell feature subset */
}
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeMIPS {
    ALL    = 0,
    R2300  = 1,
    R2600  = 2,
    R2800  = 3,
    R2000a = 4,
    R2000  = 5,
    R3000a = 6,
    R3000  = 7,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeHPA {
    HPPA_ALL    = 0,
    HPPA_7100LC = 1,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeMC88000 {
    MC88000_ALL = 0,
    MC88100     = 1,
    MC88110     = 2,
}
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeSparc {
    ALL = 0,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeI860 {
    I860_ALL = 0,
    I860_860 = 1,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypePowerPC {
    POWERPC_ALL   = 0,
    POWERPC_601   = 1,
    POWERPC_602   = 2,
    POWERPC_603   = 3,
    POWERPC_603e  = 4,
    POWERPC_603ev = 5,
    POWERPC_604   = 6,
    POWERPC_604e  = 7,
    POWERPC_620   = 8,
    POWERPC_750   = 9,
    POWERPC_7400  = 10,
    POWERPC_7450  = 11,
    POWERPC_970   = 100,
}
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeARM {
    ALL    = 0,
    V4T    = 5,
    V6     = 6,
    V5TEJ  = 7,
    XSCALE = 8,
    V7     = 9,  /* ARMv7-A and ARMv7-R */
    V7F    = 10, /* Cortex A9 */
    V7S    = 11, /* Swift */
    V7K    = 12,
    V8     = 13,
    V6M    = 14, /* Not meant to be run under xnu */
    V7M    = 15, /* Not meant to be run under xnu */
    V7EM   = 16, /* Not meant to be run under xnu */
    V8M    = 17, /* Not meant to be run under xnu */
}
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeARM32 {
    ALL = 0,
    V8  = 1,
}
}

enum_from_primitive! {
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum CpuSubtypeARM64 {
    ARM646_ALL = 0,
    ARM646_V8  = 1,
    ARM646_E   = 2,
}
}

enum_from_primitive! {
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum FileType {
    Object              = 0x01, // relocatable object file
    Execute             = 0x02, // demand paged executable file
    FixedVmLibrary      = 0x03, // fixed VM shared library file
    Core                = 0x04, // core file
    Preload             = 0x05, // preloaded executable file
    DynamicLibrary      = 0x06, // preloaded executable file
    DynamicLinker       = 0x07, // dynamic link editor
    Bundle              = 0x08, // dynamically bound bundle file
    DynamicLibraryStub  = 0x09, // shared library stub for static linking only, no section contents
    DebugSymbols        = 0x0a, // companion file with only debug sections
    KextBundle          = 0x0b, // x86_64 kexts
}
}

bitflags! {
    pub struct Flags: u32 {
        /* the object file has no undefined references */
        const NOUNDEFS = 0x0000001;
        /* the object file is the output of an incremental link against a base file and can't be
           link edited again*/
        const INCRLINK = 0x0000002;
        /* the object file is input for the dynamic linker and can't be statically link edited
           again */
        const DYLDLINK = 0x0000004;
        /* the object file's undefined references are bound by the dynamics linked when loaded */
        const BINDATLOAD = 0x0000008;
        /* the file has its dynamic undefined references prebound */
        const PREBOUND = 0x0000010;
        /* the file has its read-only and read-write segments split */
        const SPLIT_SEGS = 0x0000020;
        /* the shared library init routine is to be run lazily via catching memory faults to its
           writeable segments (obsolete)
         */
        const LAZY_INIT = 0x0000040;
        /* the image is using two-level name space bindings */
        const TWOLEVEL = 0x0000080;
        /* the executable is forcing all images to use flat name space bindings */
        const FORCE_FLAT = 0x0000100;
        /* this umbrella guarantees no multiple definitions of symbols in its sub-images so the
           two-level namespace hints can always be used */
        const NOMULTIDEFS = 0x0000200;
        /* do not have dyld notify the prebinding agent about this executable */
        const NOFIXPREBINDING = 0x0000400;
        /* the binary is not prebound but can have its prebinding redone.
           Only used when PREBOUND is not set. */
        const PREBINDABLE = 0x0000800;
        /* indicates that this binary binds to all two-level namespace modules of its dependent
           libraries. Only used when PREBINDABLE and TWOLEVEL are both set. */
        const ALLMODSBOUND = 0x0001000;
        /* safe to divide up the sections into sub-sections via symbols for dead code stripping */
        const SUBSECTIONS_VIA_SYMBOLS = 0x0002000;
        /* the binary has been canonicalized via the unprebind operation */
        const CANONICAL = 0x0004000;
        /* the final linked image contains external weak symbols */
        const WEAK_DEFINES = 0x0008000;
        /* the final linked image uses weak symbols */
        const BINDS_TO_WEAK = 0x0010000;
        /* when this bit is set, all stacks in the task will be given stack execution privilege.
           Only used in EXECUTE filetypes */
        const ALLOW_STACK_EXECUTION = 0x0020000;
        /* the binary declares it is safe for use in processes with uid zero */
        const ROOT_SAFE = 0x0040000;
        /* the binary declares it is safe for use in processes when issetugid() is true */
        const SETUID_SAFE = 0x0080000;
        /* when set on a dylib, the static linker doesn't need to examine dependent dylibs to see
           if any are re-exported */
        const NO_REEXPORTED_DYLIBS = 0x0100000;
        /* the OS will load the main executable at a random address. Only used in EXECUTE
           filetypes */
        const PIE = 0x0200000;
        /* only for use on dylibs. When linking against a dylib that has this bit set, the static
           linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no
           symbols are being referenced from the dylib */
        const DEAD_STRIPPABLE_DYLIB = 0x0400000;
        /* contains a section of type S_THREAD_LOCAL_VARIABLES */
        const HAS_TLV_DESCRIPTORS = 0x0800000;
        /* When this bit is set, the OS will run the main executable with a non-executable heap
        even on platforms (e.g. i386) that don't require it. Only used in EXECUTE filetypes. */
        const NO_HEAP_EXECUTION = 0x1000000;
        /* the code was linked for use in an application extension */
        const APP_EXTENSION_SAFE = 0x2000000;
        /* the external symbols listed in the nlist symbol table don't include all the symbols
           listed in the dyld info */
        const NLIST_OUTOFSYNC_WITH_DYLDINFO = 0x4000000;
        /* allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with the platforms macOS,
           iOSMac, iOSSimulator, tvOSSimuluator and watchOSSimulator. */
        const SIM_SUPPORT = 0x8000000;
    }
}

#[derive(Eq, PartialEq)]
pub struct Header {
    pub magic: [u8; 4],
    pub endian: Endian,
    pub arch_size: ArchSize,
    pub cpu_type: CpuType,
    pub cpu_subtype: u32,
    pub file_type: FileType,
    pub number_of_commands: u32,
    pub size_of_commands: u32,
    pub flags: Flags,
    pub reserved: u32,
}

impl Header {
    pub fn parse_from_buffer(binary: &[u8]) -> Result<Header, &'static str> {
        const SIZE: usize = 32;
        let raw_header: &[u8] = &binary[0..SIZE];

        let mut magic = [0; 4];
        magic.copy_from_slice(&raw_header[0..4]);
        let (endian, arch_size) = match magic {
            [0xfe, 0xed, 0xfa, 0xce] => (Endian::BigEndian, ArchSize::_32),
            [0xce, 0xfa, 0xed, 0xfe] => (Endian::LittleEndian, ArchSize::_32),
            [0xfe, 0xed, 0xfa, 0xcf] => (Endian::BigEndian, ArchSize::_64),
            [0xcf, 0xfa, 0xed, 0xfe] => (Endian::LittleEndian, ArchSize::_64),
            _ => panic!("Invalid magic bytes for Mach-O file!"),
        };

        let u32_from_bytes = get_num_from_bytes!(u32, endian);

        let result: Header = Header {
            magic,
            endian,
            arch_size,
            cpu_type: CpuType::from_u32(u32_from_bytes(raw_header[4..8].try_into().unwrap()))
                .unwrap(),
            // TODO: eventually should probably do something with cpu_subtype
            //       all the enums have been created but needs to be generic...
            //       not important yet, but may be useful eventually
            cpu_subtype: u32_from_bytes(raw_header[8..12].try_into().unwrap()),
            file_type: FileType::from_u32(u32_from_bytes(raw_header[12..16].try_into().unwrap()))
                .unwrap(),
            number_of_commands: u32_from_bytes(raw_header[16..20].try_into().unwrap()),
            size_of_commands: u32_from_bytes(raw_header[20..24].try_into().unwrap()),
            flags: Flags::from_bits(u32_from_bytes(raw_header[24..28].try_into().unwrap()))
                .unwrap(),
            reserved: 0, //u32_from_bytes(raw_header[28..32].try_into().unwrap()),
        };
        Ok(result)
    }

    fn formatter(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let strings = [
            format!("{:15}{:x?}", "Magic:", self.magic),
            format!("{:15}{:x?}", "Arch Size:", self.arch_size),
            format!("{:15}{:x?}", "Endian:", self.endian),
            format!("{:15}{:x?}", "CPU Type:", self.cpu_type),
            format!("{:15}{:x?}", "CPU Subtype:", self.cpu_subtype),
            format!("{:15}{:x?}", "File Type:", self.file_type),
            format!("{:15}{:x?}", "Num of Commnads:", self.number_of_commands),
            format!("{:15}{:x?}", "Size of Commnads:", self.size_of_commands),
            format!("{:15}{:x?}", "Flags:", self.flags),
            format!("{:15}{:x?}", "Reserved:", self.reserved),
        ];
        writeln!(f, "{}", strings.join("\n"))
    }
}
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}
impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.formatter(f)
    }
}

#[cfg(test)]
mod macho_header_tests {
    use super::*;

    #[test]
    fn can_parse_basic_ident_section_x86() {
        let raw = [
            0xcf, 0xfa, 0xed, 0xfe, // magic bytes
            0x07, 0x00, 0x00, 0x01, // cpu_type
            0x03, 0x00, 0x00, 0x00, // cpu_subtype
            0x02, 0x00, 0x00, 0x00, // file type
            0x13, 0x00, 0x00, 0x00, // number of commands
            0x10, 0x07, 0x00, 0x00, // size of commands
            0x85, 0x00, 0x20, 0x00, // flags
            0x00, 0x00, 0x00, 0x00, // reserved
        ];
        let expected = Header {
            magic: [0xcf, 0xfa, 0xed, 0xfe],
            endian: Endian::LittleEndian,
            arch_size: ArchSize::_64,
            cpu_type: CpuType::X86_64,
            cpu_subtype: 0x03,
            file_type: FileType::Execute,
            flags: Flags::from_bits(0x200085).unwrap(),
            number_of_commands: 0x13,
            size_of_commands: 0x710,
            reserved: 0,
        };
        let header = Header::parse_from_buffer(&raw);
        match header {
            Ok(v) => assert_eq!(v, expected),
            Err(_) => assert!(false),
        };
    }

    // #[test]
    // fn can_parse_basic_ident_section_arm64() {
    //     let raw = [
    //         0xca, 0xfe, 0xba, 0xbe,
    //         0x00, 0x00, 0x00, 0x02,
    //         0x01, 0x00, 0x00, 0x07,
    //         0x00, 0x00, 0x00, 0x03,
    //         0x00, 0x00, 0x40, 0x00,
    //         0x00, 0x01, 0x1c, 0x60,
    //         0x00, 0x00, 0x00, 0x0e,
    //         0x01, 0x00, 0x00, 0x0c,
    //     ];
    //     let expected = Header {
    //         magic: [0xcf, 0xfa, 0xed, 0xfe],
    //         endian: Endian::LittleEndian,
    //         arch_size: ArchSize::_64,
    //         cpu_type: CpuType::X86_64,
    //         cpu_subtype: 0x03,
    //         file_type: FileType::Execute,
    //         flags: Flags::from_bits(0x200085).unwrap(),
    //         number_of_commands: 0x13,
    //         size_of_commands: 0x710,
    //         reserved: 0,
    //     };
    //     let header = Header::parse_from_buffer(&raw);
    //     match header {
    //         Ok(v) => assert_eq!(v, expected),
    //         Err(_) => assert!(false),
    //     };
    // }
}
