#![no_std]

//! spd: A no_std crate for Serial Presence Detect manipulation

pub use num_derive::{FromPrimitive, ToPrimitive};
pub use num_traits::{FromPrimitive, ToPrimitive};

type SelectAddress = u8;
type Block = u8;

pub const MAX_DEVICES: u8 = 8;
pub const MAX_SIZE: usize = 512;
pub const PAGE_SIZE: usize = 256;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Page(pub u8);

impl Page {
    pub fn offset(&self) -> usize {
        if self.0 == 0 {
            0
        } else {
            PAGE_SIZE
        }
    }
}

///
/// Functions as described in Table 2 of EE1004.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Function {
    Temperature(SelectAddress),
    Memory(SelectAddress),
    ProtectionStatus(Block),
    ClearAllWriteProtection,
    PageAddress(Page),
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
pub enum Offset {
    SPDDeviceSize = 0x000,
    SPDRevision = 0x001,
    DRAMDeviceType = 0x002,
    ModuleType = 0x003,
    SDRAMDensity = 0x004,
    PrimarySDRAMPackageType = 0x005,
    SDRAMOptionalFeatures = 0x007,
    SDRAMThermalOptions = 0x008,
    OtherSDRAMFeatures = 0x009,
    SecondarySDRAMPackageType = 0x00a,
    ModuleNominalVoltage = 0x00b,
    ModuleOrganization = 0x00c,
    ModuleMemoryBusWidth = 0x00d,
    ExtendedModuleType = 0x00f,
    Timebases = 0x011,
    TCkAvgMin = 0x012,
    TCkAvgMax = 0x013,
    CASLatencies0 = 0x014,
    CASLatencies1 = 0x015,
    CASLatencies2 = 0x016,
    CASLatencies3 = 0x017,
    TAAMin = 0x018,
    TRCDMin = 0x019,
    TRPMin = 0x01a,
    UpperNibblesTRASMin = 0x01b,
    TRASMin = 0x01c,

    TRCMin = 0x01d,
    TRFC1MinLSB = 0x01e,
    TRFC1MinMSB = 0x01f,
    TRFC2MinLSB = 0x020,
    TRFC2MinMSB = 0x021,
    TRFC4MinLSB = 0x022,
    TRFC4MinMSB = 0x023,
    TFAWminMSB = 0x024,
    TFAWminLSB = 0x025,
    TRRDSMin = 0x026,
    TRRDLMin = 0x027,
    UpperNibbleTWRMin = 0x29,
    TWRMin = 0x2a,
    UpperNibblesTWTRMin = 0x2b,
    TWTRSMin = 0x2c,
    TWTRLMin = 0x2d,
    TCCDLMinFine = 0x75,
    TRRDLMinFine = 0x76,
    TRRDSMinFine = 0x77,
    TRCMinFind = 0x78,
    TRPMinFine = 0x79,
    TRCDMinFine = 0x7a,
    TAAMinFine = 0x7b,
    TCkAvgMaxFine = 0x7c,
    TCkAvgMinFine = 0x7d,
    CRCBaseLSB = 0x7e,
    CRCBaseMSB = 0x7f,
    ModuleManufacturerIDCodeLSB = 0x140,
    ModuleManufacturerIDCodeMSB = 0x141,
    ModuleManufacturingLocation = 0x142,
    ModuleManufacturingDateYear = 0x143,
    ModuleManufacturingDateWeek = 0x144,
    ModuleSerialNumber0 = 0x145,
    ModuleSerialNumber1 = 0x146,
    ModuleSerialNumber2 = 0x147,
    ModuleSerialNumber3 = 0x148,
    PartNumberBase = 0x149,
    PartNumberLimit = 0x15c,
    DRAMManufacturerIDCodeLSB = 0x15e,
    DRAMManufacturerIDCodeMSB = 0x15f,
    DRAMStepping = 0x160,
}

impl Offset {
    pub fn to_usize(self) -> usize {
        self as usize
    }

    pub fn within(self, buf: &[u8]) -> u8 {
        buf[self as usize]
    }
}

impl Function {
    ///
    /// For a given function, return its device code.  This code follows the
    /// structure of Table 2 in EE1004.
    ///
    pub fn to_device_code(self) -> Option<u8> {
        match self {
            Function::Temperature(addr) => {
                if addr <= 0b111 {
                    Some((0b0011 << 3) | addr)
                } else {
                    None
                }
            }

            Function::Memory(addr) => {
                if addr <= 0b111 {
                    Some((0b1010 << 3) | addr)
                } else {
                    None
                }
            }

            Function::ProtectionStatus(block) => {
                let dtid = 0b0110 << 3;

                match block {
                    0 => Some(dtid | 0b001),
                    1 => Some(dtid | 0b100),
                    2 => Some(dtid | 0b101),
                    3 => Some(dtid | 0b000),
                    _ => None,
                }
            }

            Function::ClearAllWriteProtection => Some(0b0110_011),

            Function::PageAddress(page) => {
                let dtid = 0b0110 << 3;

                match page.0 {
                    0 => Some(dtid | 0b110),
                    1 => Some(dtid | 0b111),
                    _ => None,
                }
            }
        }
    }

    ///
    /// For a given device code, return the function (if any).  This code follows
    /// the structure of Table 2 in EE1004, which yes, is in function order
    /// not device code order.
    ///
    pub fn from_device_code(code: u8) -> Option<Self> {
        let device_type_identifier = code >> 3;
        let select_address = code & 0b111;

        match device_type_identifier {
            0b0011 => Some(Function::Temperature(select_address)),

            0b1010 => Some(Function::Memory(select_address)),

            0b0110 => match select_address {
                0b001 => Some(Function::ProtectionStatus(0)),
                0b100 => Some(Function::ProtectionStatus(1)),
                0b101 => Some(Function::ProtectionStatus(2)),
                0b000 => Some(Function::ProtectionStatus(3)),
                0b011 => Some(Function::ClearAllWriteProtection),
                0b110 => Some(Function::PageAddress(Page(0))),
                0b111 => Some(Function::PageAddress(Page(1))),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;
    use std::println;

    #[test]
    fn table() {
        assert_eq!(Function::Temperature(0).to_device_code(), Some(0b0011_000));
        assert_eq!(Function::Temperature(1).to_device_code(), Some(0b0011_001));
        assert_eq!(Function::Temperature(9).to_device_code(), None);
        assert_eq!(Function::Memory(0).to_device_code(), Some(0b1010_000));
        assert_eq!(Function::Memory(1).to_device_code(), Some(0b1010_001));
        assert_eq!(Function::Memory(9).to_device_code(), None);
    }

    #[test]
    fn alladdr() {
        for i in 0..=0xff {
            if let Some(func) = Function::from_device_code(i) {
                println!("0x{:02x} {:?}", i, func);
                assert_eq!(func.to_device_code(), Some(i));
            }
        }
    }
}
