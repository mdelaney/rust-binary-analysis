// TODO - remove allow dead code
#![allow(non_camel_case_types)]
#![macro_use]

use std::convert::TryInto;
use std::fmt;
use std::mem;
use std::result::Result::Err;

// This makes it easy to get the function to convert from bytes to a number type
// with a specified endianness
// use like:
//   let u32_from_bytes = get_num_from_bytes!(u32, ident.ei_data)
macro_rules! get_num_from_bytes {
    ( $size:ident, $x:expr ) => {
        match $x {
            EI_Data::LittleEndian => $size::from_le_bytes,
            EI_Data::BigEndian => $size::from_be_bytes,
        }
    };
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
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
#[derive(Debug, Eq, PartialEq)]
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
#[derive(Debug, Eq, PartialEq)]
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

#[derive(Eq, PartialEq)]
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
    pub fn parse_from_buffer(binary: &[u8]) -> Result<ELFIdent, &'static str> {
        const SIZE: usize = mem::size_of::<ELFIdent>();
        let raw_ident: &[u8] = &binary[0..SIZE];

        let mut magic = [0; 4];
        magic.copy_from_slice(&raw_ident[0..4]);
        if magic != [0x7F, 0x45, 0x4C, 0x46] {
            return Err("invalid magic bytes!");
        }

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
        Ok(result)
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

#[cfg(test)]
mod elf_ident_tests {
    use super::*;

    #[test]
    fn can_parse_basic_ident_section() {
        let raw = &[
            0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        let expected = ELFIdent {
            ei_magic: [0x7F, 0x45, 0x4C, 0x46],
            ei_class: EI_Class::ELF64,
            ei_data: EI_Data::LittleEndian,
            ei_version: 1,
            ei_os_abi: EI_OSABI::SystemV,
            ei_abi_version: 0,
            ei_pad: [0; 7],
        };
        match ELFIdent::parse_from_buffer(raw) {
            Ok(v) => assert_eq!(v, expected),
            Err(_) => assert!(false),
        };
    }

    #[test]
    fn fails_to_parse_with_bad_magic_bytes() {
        let raw = &[
            0x6F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        match ELFIdent::parse_from_buffer(raw) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        };
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
#[derive(Debug, Eq, PartialEq)]
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
#[derive(Debug, Eq, PartialEq)]
pub enum E_Machine {
    None,                   // 0x00 No Machine
    M32,                    // 0x01 AT&T WE 32100
    Sparc,                  // 0x02 Sun Sparc
    X86,                    // 0x03 Intel 80386
    Motorola_m68K,          // 0x04 Motorola m68k family
    Motorola_m88K,          // 0x05 Motorola m88k family
    Intel_MCU,              // 0x06 Intel MCU
    Intel_860,              // 0x07 Intel 80860
    MIPS_R3000_BE,          // 0x08 MIPS R3000 big-endian
    IBM_System_370,         // 0x09 IBM System 370
    MIPS_R3000_LE,          // 0x0a MIPS R3000 little-endian
    HPPA,                   // 0x0f HPPA
    Fujitsu_VPP500,         // 0x11 Fujitsu VPP500
    SunV8Plus,              // 0x12 Sun's "v8plus"
    Intel_80960,            // 0x13 Intel 80960
    PowerPC,                // 0x14 Power PC
    PowerPC_64,             // 0x15 PowerPC 64-bit
    IBM_S390,               // 0x16 IBM S390
    IBM_SPU,                // 0x17 IBM SPU/SPC
    NEC_V800,               // 0x24 NEC V800 series
    Fujitsu_FR20,           // 0x25 Fujitsu FR20
    TRW_RH32,               // 0x26 TRW RH-32
    Motorola_RCE,           // 0x27 Motorola RCE
    ARM,                    // 0x28 ARM
    DigitalAlpha,           // 0x29 Digital Alpha
    HitachiSuperH,          // 0x2a Hitachi SH
    SPARC_V9,               // 0x2b SPARC v9 64-bit
    Tricore,                // 0x2c Siemens Tricore
    ARC,                    // 0x2d Argonaut RISC Core
    Hitachi_H8_300,         // 0x2e Hitachi H8/300
    Hitachi_H8_300H,        // 0x2f Hitachi H8/300H
    Hitachi_H8S,            // 0x30 Hitachi H8S
    Hitachi_H8_500,         // 0x31 Hitachi H8/500
    IA_64,                  // 0x32 Intel Merced
    MIPS_X,                 // 0x33 Stanford MIPS-X
    Motorola_Coldfire,      // 0x34 Motorola Coldfire
    Motorola_68HC12,        // 0x35 Motorola M68HC12
    Fujitsu_MMA,            // 0x36 Fujitsu MMA Multimedia Accelerator
    Siemens_PCP,            // 0x37 Siemens PCP
    Sony_nCPU,              // 0x38 Sony nCPU embeeded RISC
    Denso_NDR1,             // 0x39 Denso NDR1 microprocessor
    Motorola_StartCore,     // 0x3a Motorola Start*Core processor
    Toyota_ME16,            // 0x3b Toyota ME16 processor
    STM_ST100,              // 0x3c STMicroelectronic ST100 processor
    TinyJ,                  // 0x3d Advanced Logic Corp. Tinyj emb.fam
    X86_64,                 // 0x3e AMD x86-64 architecture
    Sony_PDSP,              // 0x3f Sony DSP Processor
    Digital_PDP10,          // 0x40 Digital PDP-10
    Digital_PDP11,          // 0x41 Digital PDP-11
    Siemens_FX66,           // 0x42 Siemens FX66 microcontroller
    STM_ST9Plus,            // 0x43 STMicroelectronics ST9+ 8/16 mc
    STM_ST7,                // 0x44 STmicroelectronics ST7 8 bit mc
    Motorola_68HC16,        // 0x45 Motorola MC68HC16 microcontroller
    Motorola_68HC11,        // 0x46 Motorola MC68HC11 microcontroller
    Motorola_68HC08,        // 0x47 Motorola MC68HC08 microcontroller
    Motorola_68HC05,        // 0x48 Motorola MC68HC05 microcontroller
    SiliconGraphics_SVX,    // 0x49 Silicon Graphics SVx
    STM_ST19,               // 0x4a STMicroelectronics ST19 8 bit mc
    Digital_VAX,            // 0x4b Digital VAX
    Axis_CRIS,              // 0x4c Axis Communications 32-bit emb.proc
    InfineonJavelin,        // 0x4d Infineon Technologies 32-bit emb.proc
    Element14Firepath,      // 0x4e Element 14 64-bit DSP Processor
    LSI_ZSP,                // 0x4f LSI Logic 16-bit DSP Processor
    MMIX,                   // 0x50 Donald Knuth's educational 64-bit proc
    HUANY,                  // 0x51 Harvard University machine-independent object files
    SiTera_Prism,           // 0x52 SiTera Prism
    Atmel_AVR,              // 0x53 Atmel AVR 8-bit microcontroller
    Fujitsu_FR30,           // 0x54 Fujitsu FR30
    Mitsubishi_D10V,        // 0x55 Mitsubishi D10V
    Mitsubishi_D30V,        // 0x56 Mitsubishi D30V
    NEC_V850,               // 0x57 NEC v850
    Mitsubishi_M32R,        // 0x58 Mitsubishi M32R
    Matsushita_MN10300,     // 0x59 Matsushita MN10300
    Matsushita_MN10200,     // 0x5a Matsushita MN10200
    PicoJava,               // 0x5b picoJava
    OpenRISC,               // 0x5c OpenRISC 32-bit embedded processor
    ARC_Compact,            // 0x5d ARC International ARCompact
    TensilicaXtensa,        // 0x5e Tensilica Xtensa Architecture
    AlphamosaicVideoCore,   // 0x5f Alphamosaic VideoCore
    TMM_GPP,                // 0x60 Thompson Multimedia General Purpose Proc
    NationalSemi_32K,       // 0x61 National Semi. 32000
    TenorNetwork_TPC,       // 0x62 Tenor Network TPC
    Trebia_SNP1K,           // 0x63 Trebia SNP 1000
    STM_ST200,              // 0x64 STMicroelectronics ST200
    Ubicom_IP2K,            // 0x65 Ubicom IP2xxx
    MAX,                    // 0x66 MAX processor
    NationalSemi_CR,        // 0x67 National Semi. CompactRISC
    Fujitsu_F2MC16,         // 0x68 Fujitsu F2MC16
    TI_MSP430,              // 0x69 Texas Instruments msp430
    AnalogDevices_Blackfin, // 0x6a Analog Devices Blackfin DSP
    Epson_SE_C33,           // 0x6b Seiko Epson S1C33 family
    SharpEmbeddedProcessor, // 0x6c Sharp embedded microprocessor
    Arca,                   // 0x6d Arca RISC
    Unicore,                // 0x6e PKU-Unity & MPRC Peking Uni. mc series
    Excess,                 // 0x6f eXcess configurable cpu
    Icera_DXP,              // 0x70 Icera Semi. Deep Execution Processor
    Altera_NIOS2,           // 0x71 Altera Nios II
    NationalSemi_CRX,       // 0x72 National Semi. CompactRISC CRX
    Motorola_XGATE,         // 0x73 Motorola XGATE
    Infineon_C166,          // 0x74 Infineon C16x/XC16x
    Renesas_M16C,           // 0x75 Renesas M16C
    MicrochipTech_dsPIC30F, // 0x76 Microchip Technology dsPIC30F
    Freescale_CE,           // 0x77 Freescale Communication Engine RISC
    Renesas_M32C,           // 0x78 Renesas M32C
    // 0x78 - 0x82 reserved
    Altium_TSK3000,      // 0x83 Altium TSK3000
    Freescale_RS08,      // 0x84 Freescale RS08
    AnalogDevices_SHARC, // 0x85 Analog Devices SHARC family
    Cyan_ECOG2,          // 0x86 Cyan Technology eCOG2
    Sunplus_SCORE7,      // 0x87 Sunplus S+core7 RISC
    NewJapanRadio_DSP24, // 0x88 New Japan Radio (NJR) 24-bit DSP
    Broadcom_VideoCore3, // 0x89 Broadcom VideoCore III
    LatticeMICO32,       // 0x8a RISC for Lattice FPGA
    SeiekoEpson_C17,     // 0x8b Seiko Epson C17
    TI_C6000,            // 0x8c Texas Instruments TMS320C6000 DSP
    TI_C2000,            // 0x8d Texas Instruments TMS320C2000 DSP
    TI_C5500,            // 0x8e Texas Instruments TMS320C55x DSP
    TI_ARP32,            // 0x8f Texas Instruments App. Specific RISC
    TI_PRU,              // 0x90 Texas Instruments Prog. Realtime Unit
    // 0x91 -0x9f reserved
    STM_MMDSP_Plus,        // 0xa0 STMicroelectronics 64bit VLIW DSP
    Cypress_M8C,           // 0xa1 Cypress M8C
    Renesas_R32C,          // 0xa2 Renesas R32C
    NXP_TriMedia,          // 0xa3 NXP Semi. TriMedia
    Qualcomm_DSP6,         // 0xa4 QUALCOMM DSP6
    Intel_8051,            // 0xa5 Intel 8051 and variants
    STM_STxP7x,            // 0xa6 STMicroelectronics STxP7x
    AndesTech_NDS32,       // 0xa7 Andes Tech. compact code emb. RISC
    Cyan_ECOG1X,           // 0xa8 Cyan Technology eCOG1X
    DallasSemi_MAXQ30,     // 0xa9 Dallas Semi. MAXQ30 mc
    NewJapanRadio_XIMO16,  // 0xaa New Japan Radio (NJR) 16-bit DSP
    MANIK,                 // 0xab M2000 Reconfigurable RISC
    Cray_NV2,              // 0xac Cray NV2 vector architecture
    Renesas_RX,            // 0xad Renesas RX
    ImaginationTech_METAG, // 0xae Imagination Tech. META
    MCST_Elbrus,           // 0xaf MCST Elbrus
    Cyan_eCOG16,           // 0xb0 Cyan Technology eCOG16
    NationialSemi_CR16,    // 0xb1 National Semi. CompactRISC CR16
    Freescale_ETPU,        // 0xb2 Freescale Extended Time Processing Unit
    Infineon_SLE9X,        // 0xb3 Infineon Tech. SLE9X
    Intel_L10M,            // 0xb4 Intel L10M
    Intel_K10M,            // 0xb5 Intel K10M
    // 0xb6 reserved
    ARM_AARCH64, // 0xb7 ARM AARCH64
    // 0xb8 reserved
    Atmel_AVR32,           // 0xb9 Amtel 32-bit microprocessor
    STM8,                  // 0xba STMicroelectronics STM8
    Tileta_TILE64,         // 0xbb Tileta TILE64
    Tileta_TILEPro,        // 0xbc Tilera TILEPro
    Xilinx_MicroBlaze,     // 0xbd Xilinx MicroBlaze
    NVIDIA_CUDA,           // 0xbe NVIDIA CUDA
    Tilera_TILEGx,         // 0xbf Tilera TILE-Gx
    CloudShield,           // 0xc0 CloudShield
    CoreA_1ST,             // 0xc1 KIPO-KAIST Core-A 1st gen.
    CoreA_2ND,             // 0xc2 KIPO-KAIST Core-A 2nd gen.
    Synopsys_ARC_COMPACT2, // 0xc3 Synopsys ARCompact V2
    Open8,                 // 0xc4 Open8 RISC
    Renesas_RL78,          // 0xc5 Renesas RL78
    Broadcom_VideoCore5,   // 0xc6 Broadcom VideoCore V
    Renesas_78KOR,         // 0xc7 Renesas 78KOR
    Freescale_56800EX,     // 0xc8 Freescale 56800EX DSC
    Beyond_BA1,            // 0xc9 Beyond BA1
    Beyond_BA2,            // 0xca Beyond BA2
    XMOS_xCORE,            // 0xcb XMOS xCORE
    Microchip_PIC,         // 0xcc Microchip 8-bit PIC(r)
    // 0xcd - 0xd1 reserved
    KM211_KM32,                    // 0xd2 KM211 KM32
    KM211_KMX32,                   // 0xd3 KM211 KMX32
    KM211_EMX16,                   // 0xd4 KM211 KMX16
    KM211_EMX8,                    // 0xd5 KM211 KMX8
    KM211_KVARC,                   // 0xd6 KM211 KVARC
    Paneve_CDP,                    // 0xd7 Paneve CDP
    CognitiveSmartMemoryProcessor, // 0xd8 Cognitive Smart Memory Processor
    Bluechip_Cool,                 // 0xd9 Bluechip CoolEngine
    NanoradioOptimizedRISC,        // 0xda Nanoradio Optimized RISC
    CSR_Kalimba,                   // 0xdb CSR Kalimba
    Zilog_Z80,                     // 0xdc Zilog Z80
    VISIUM,                        // 0xdd Controls and Data Services VISIUMcore
    FTDI_FT32,                     // 0xde FTDI Chip FT32
    Moxie,                         // 0xdf Moxie processor
    AMD_GPU,                       // 0xe0 AMD GPU
    // 0xe1 - 0xf2
    RiscV,    // 0xf3 Risk-V
    LinuxBPF, // 0xf7 Linux BPF -- in-kernel virtual machine
    C_SKY,    // 0xfc C-SKY
    NUM,      // 0xfd
    Alpha,    // 0x9026 Alpha,
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
            0x0041 => E_Machine::Digital_PDP11,
            0x0042 => E_Machine::Siemens_FX66,
            0x0043 => E_Machine::STM_ST9Plus,
            0x0044 => E_Machine::STM_ST7,
            0x0045 => E_Machine::Motorola_68HC16,
            0x0046 => E_Machine::Motorola_68HC11,
            0x0047 => E_Machine::Motorola_68HC08,
            0x0048 => E_Machine::Motorola_68HC05,
            0x0049 => E_Machine::SiliconGraphics_SVX,
            0x004a => E_Machine::STM_ST19,
            0x004b => E_Machine::Digital_VAX,
            0x004c => E_Machine::Axis_CRIS,
            0x004d => E_Machine::InfineonJavelin,
            0x004e => E_Machine::Element14Firepath,
            0x004f => E_Machine::LSI_ZSP,
            0x0050 => E_Machine::MMIX,
            0x0051 => E_Machine::HUANY,
            0x0052 => E_Machine::SiTera_Prism,
            0x0053 => E_Machine::Atmel_AVR,
            0x0054 => E_Machine::Fujitsu_FR30,
            0x0055 => E_Machine::Mitsubishi_D10V,
            0x0056 => E_Machine::Mitsubishi_D30V,
            0x0057 => E_Machine::NEC_V850,
            0x0058 => E_Machine::Mitsubishi_M32R,
            0x0059 => E_Machine::Matsushita_MN10300,
            0x005a => E_Machine::Matsushita_MN10200,
            0x005b => E_Machine::PicoJava,
            0x005c => E_Machine::OpenRISC,
            0x005d => E_Machine::ARC_Compact,
            0x005e => E_Machine::TensilicaXtensa,
            0x005f => E_Machine::AlphamosaicVideoCore,
            0x0060 => E_Machine::TMM_GPP,
            0x0061 => E_Machine::NationalSemi_32K,
            0x0062 => E_Machine::TenorNetwork_TPC,
            0x0063 => E_Machine::Trebia_SNP1K,
            0x0064 => E_Machine::STM_ST200,
            0x0065 => E_Machine::Ubicom_IP2K,
            0x0066 => E_Machine::MAX,
            0x0067 => E_Machine::NationalSemi_CR,
            0x0068 => E_Machine::Fujitsu_F2MC16,
            0x0069 => E_Machine::TI_MSP430,
            0x006a => E_Machine::AnalogDevices_Blackfin,
            0x006b => E_Machine::Epson_SE_C33,
            0x006c => E_Machine::SharpEmbeddedProcessor,
            0x006d => E_Machine::Arca,
            0x006e => E_Machine::Unicore,
            0x006f => E_Machine::Excess,
            0x0070 => E_Machine::Icera_DXP,
            0x0071 => E_Machine::Altera_NIOS2,
            0x0072 => E_Machine::NationalSemi_CRX,
            0x0073 => E_Machine::Motorola_XGATE,
            0x0074 => E_Machine::Infineon_C166,
            0x0075 => E_Machine::Renesas_M16C,
            0x0076 => E_Machine::MicrochipTech_dsPIC30F,
            0x0077 => E_Machine::Freescale_CE,
            0x0078 => E_Machine::Renesas_M32C,
            0x0083 => E_Machine::Altium_TSK3000,
            0x0084 => E_Machine::Freescale_RS08,
            0x0085 => E_Machine::AnalogDevices_SHARC,
            0x0086 => E_Machine::Cyan_ECOG2,
            0x0087 => E_Machine::Sunplus_SCORE7,
            0x0088 => E_Machine::NewJapanRadio_DSP24,
            0x0089 => E_Machine::Broadcom_VideoCore3,
            0x008a => E_Machine::LatticeMICO32,
            0x008b => E_Machine::SeiekoEpson_C17,
            0x008c => E_Machine::TI_C6000,
            0x008d => E_Machine::TI_C2000,
            0x008e => E_Machine::TI_C5500,
            0x008f => E_Machine::TI_ARP32,
            0x0090 => E_Machine::TI_PRU,
            0x00a0 => E_Machine::STM_MMDSP_Plus,
            0x00a1 => E_Machine::Cypress_M8C,
            0x00a2 => E_Machine::Renesas_R32C,
            0x00a3 => E_Machine::NXP_TriMedia,
            0x00a4 => E_Machine::Qualcomm_DSP6,
            0x00a5 => E_Machine::Intel_8051,
            0x00a6 => E_Machine::STM_STxP7x,
            0x00a7 => E_Machine::AndesTech_NDS32,
            0x00a8 => E_Machine::Cyan_ECOG1X,
            0x00a9 => E_Machine::DallasSemi_MAXQ30,
            0x00aa => E_Machine::NewJapanRadio_XIMO16,
            0x00ab => E_Machine::MANIK,
            0x00ac => E_Machine::Cray_NV2,
            0x00ad => E_Machine::Renesas_RX,
            0x00ae => E_Machine::ImaginationTech_METAG,
            0x00af => E_Machine::MCST_Elbrus,
            0x00b0 => E_Machine::Cyan_eCOG16,
            0x00b1 => E_Machine::NationialSemi_CR16,
            0x00b2 => E_Machine::Freescale_ETPU,
            0x00b3 => E_Machine::Infineon_SLE9X,
            0x00b4 => E_Machine::Intel_L10M,
            0x00b5 => E_Machine::Intel_K10M,
            0x00b7 => E_Machine::ARM_AARCH64,
            0x00b9 => E_Machine::Atmel_AVR32,
            0x00ba => E_Machine::STM8,
            0x00bb => E_Machine::Tileta_TILE64,
            0x00bc => E_Machine::Tileta_TILEPro,
            0x00bd => E_Machine::Xilinx_MicroBlaze,
            0x00be => E_Machine::NVIDIA_CUDA,
            0x00bf => E_Machine::Tilera_TILEGx,
            0x00c0 => E_Machine::CloudShield,
            0x00c1 => E_Machine::CoreA_1ST,
            0x00c2 => E_Machine::CoreA_2ND,
            0x00c3 => E_Machine::Synopsys_ARC_COMPACT2,
            0x00c4 => E_Machine::Open8,
            0x00c5 => E_Machine::Renesas_RL78,
            0x00c6 => E_Machine::Broadcom_VideoCore5,
            0x00c7 => E_Machine::Renesas_78KOR,
            0x00c8 => E_Machine::Freescale_56800EX,
            0x00c9 => E_Machine::Beyond_BA1,
            0x00ca => E_Machine::Beyond_BA2,
            0x00cb => E_Machine::XMOS_xCORE,
            0x00cc => E_Machine::Microchip_PIC,
            0x00d2 => E_Machine::KM211_KM32,
            0x00d3 => E_Machine::KM211_KMX32,
            0x00d4 => E_Machine::KM211_EMX16,
            0x00d5 => E_Machine::KM211_EMX8,
            0x00d6 => E_Machine::KM211_KVARC,
            0x00d7 => E_Machine::Paneve_CDP,
            0x00d8 => E_Machine::CognitiveSmartMemoryProcessor,
            0x00d9 => E_Machine::Bluechip_Cool,
            0x00da => E_Machine::NanoradioOptimizedRISC,
            0x00db => E_Machine::CSR_Kalimba,
            0x00dc => E_Machine::Zilog_Z80,
            0x00dd => E_Machine::VISIUM,
            0x00de => E_Machine::FTDI_FT32,
            0x00df => E_Machine::Moxie,
            0x00e0 => E_Machine::AMD_GPU,
            0x00f3 => E_Machine::RiscV,
            0x00f7 => E_Machine::LinuxBPF,
            0x00fc => E_Machine::C_SKY,
            0x00fd => E_Machine::NUM,
            0x9026 => E_Machine::Alpha,
            _ => E_Machine::Unknown,
        }
    }
}

#[derive(Eq, PartialEq)]
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
    pub fn parse_from_buffer(binary: &[u8], ident: ELFIdent) -> ELFHeader {
        // First get the bytes for our header
        const SIZE: usize = mem::size_of::<ELFHeader>();
        let raw: &[u8] = &binary[mem::size_of::<ELFIdent>()..SIZE];

        // Now get our conversion functions to read numbers based on endianness
        let u16_from_bytes = get_num_from_bytes!(u16, ident.ei_data);
        let u32_from_bytes = get_num_from_bytes!(u32, ident.ei_data);
        let u64_from_bytes = get_num_from_bytes!(u64, ident.ei_data);

        // Finally we can create our header
        // We use 64bit values here as we can avoid duplicating everything for 32bit
        // files (given the numeric conversion is lossless)
        let result: ELFHeader = match ident.ei_class {
            EI_Class::ELF32 => ELFHeader {
                ident,
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
                ident,
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

#[cfg(test)]
mod elf_header_tests {
    use super::*;

    #[test]
    fn can_parse_basic_64_bit_little_endian_header() {
        let raw = &[
            0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x03, 0x00, 0x3E, 0x00, 0x01, 0x00, 0x00, 0x00, 0xD0, 0x67, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x23,
            0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x38, 0x00,
            0x0D, 0x00, 0x40, 0x00, 0x1E, 0x00, 0x1D, 0x00,
        ];
        let ident = match ELFIdent::parse_from_buffer(&raw[0..16]) {
            Ok(v) => v,
            Err(_) => panic!("Unable to parse valid ident"),
        };
        let expected = ELFHeader {
            ident,
            e_type: E_Type::DYN,
            e_machine: E_Machine::X86_64,
            e_version: 1,
            e_entry: 26576,
            e_phoff: 64,
            e_shoff: 140224,
            e_flags: 0,
            e_ehsize: 64,
            e_phentsize: 56,
            e_phnum: 13,
            e_shentsize: 64,
            e_shnum: 30,
            e_shstrndx: 29,
        };
        let ident = match ELFIdent::parse_from_buffer(&raw[0..16]) {
            Ok(v) => v,
            Err(_) => panic!("Unable to parse valid ident"),
        };
        let header = ELFHeader::parse_from_buffer(raw, ident);
        assert_eq!(expected, header);
    }
}
