#![cfg_attr(debug_assertions, allow(dead_code))]

pub const ELF_EI_MAG0: u8 = 0;
pub const ELF_EI_MAG1: u8 = 1;
pub const ELF_EI_MAG2: u8 = 2;
pub const ELF_EI_MAG3: u8 = 3;
pub const ELF_EI_CLASS: u8 = 4;
pub const ELF_EI_DATA: u8 = 5;
pub const ELF_EI_VERSION: u8 = 6;
pub const ELF_EI_OSABI: u8 = 7;
pub const ELF_EI_ABIVERSION: u8 = 8;
pub const ELF_EI_PAD: u8 = 9;
pub const ELF_EI_NIDENT: u8 = 16;

pub const ELF_CLASS32: u8 = 1;
pub const ELF_CLASS64: u8 = 2;

pub const ELF_DATA2LSB: u8 = 1;
pub const ELF_DATA2MSB: u8 = 2;

pub const ELF_EV_CURRENT: u8 = 1;

pub const ELF_OSABI_SYSV: u8 = 0;
pub const ELF_OSABI_HPUX: u8 = 1;
pub const ELF_OSABI_STANDALONE: u8 = 255;

pub const ELF_ET_NONE: u16 = 0;
pub const ELF_ET_REL: u16 = 1;
pub const ELF_ET_EXEC: u16 = 2;
pub const ELF_ET_DYN: u16 = 3;
pub const ELF_ET_CORE: u16 = 4;
pub const ELF_ET_LOOS: u16 = 0xfe00;
pub const ELF_ET_HIOS: u16 = 0xfeff;
pub const ELF_ET_LOPROC: u16 = 0xff00;
pub const ELF_ET_HIPROC: u16 = 0xffff;
