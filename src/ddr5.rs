//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//

//! DDR5 SPD, based on JESD400-5C

/// Memory offsets within the 1024-byte data array
///
/// See Tables 17 and 91 in JESD400-5C
#[derive(Copy, Clone, Debug)]
pub enum Offset {
    /// Number of Bytes in SPD Device and Beta Level
    SPDDeviceSize = 0x000,
    /// SPD Revision for Base Configuration Parameters
    BaseSPDRevision = 0x001,
    /// Key Byte / Host Bus Command Protocol Type
    HostBusCommandProtocolType = 0x002,
    /// Key Byte / Module Type
    ModuleType = 0x003,
    /// First SDRAM Density and Package
    FirstSDRAMDensityAndPackage = 0x004,
    /// First SDRAM Addressing
    FirstSDRAMAddressing = 0x005,
    /// First SDRAM I/O Width
    FirstSDRAMIoWidth = 0x006,
    /// First SDRAM Bank Groups and Banks Per Bank Group
    FirstSDRAMBankGroups = 0x007,
    /// Second SDRAM Density and Package
    SecondSDRAMDensityAndPackage = 0x008,
    /// Second SDRAM Addressing
    SecondSDRAMAddressing = 0x009,
    /// Second SRAM I/O Width
    SecondSDRAMIoWidth = 0x00a,
    /// Second SDRAM Bank Groups and Banks Per Bank Group
    SecondSDRAMBankGroups = 0x00b,
    /// SDRAM BL32 and Post Package Repair
    BL32AndPostPackageRepair = 0x00c,
    /// SDRAM Duty Cycle Adjuster and Partial Array Self Refresh
    DutyCycleAdjuster = 0x00d,
    /// SDRAM Per Row Activation Counting, Fault Handling, and Temperature Sense
    PerRowActivationCounting = 0x00e,
    // 0x00f is reserved
    /// SDRAM Nominal Voltage, VDD
    NominalVoltageVdd = 0x010,
    /// SDRAM Nominal Voltage, VDDQ
    NominalVoltageVddq = 0x011,
    /// SDRAM Nominal Voltage, VPP
    NominalVoltageVpp = 0x012,
    /// SDRAM Timing
    Timing = 0x013,
    /// SDRAM Mininum Cycle Time (t_CKAVG^min), Least Significant Byte
    MinimumCycleTimeLsb = 0x014,
    /// SDRAM Mininum Cycle Time (t_CKAVG^min), Most Significant Byte
    MinimumCycleTimeMsb = 0x015,
    /// SDRAM Maximum Cycle Time (t_CKAVG^max), Least Significant Byte
    MaximumCycleTimeLsb = 0x016,
    /// SDRAM Maximum Cycle Time (t_CKAVG^max), Most Significant Byte
    MaximumCycleTimeMsb = 0x017,
    /// CAS Latencies Supported, First Byte
    CASLatencies0 = 0x018,
    /// CAS Latencies Supported, Second Byte
    CASLatencies1 = 0x019,
    /// CAS Latencies Supported, Third Byte
    CASLatencies2 = 0x01a,
    /// CAS Latencies Supported, Fourth Byte
    CASLatencies3 = 0x01b,
    /// CAS Latencies Supported, Fifth Byte
    CASLatencies4 = 0x01c,
    // 0x01d is reserved
    /// SDRAM Read Command to First Data (t_AA), Least Significant Byte
    TAALsb = 0x1e,
    /// SDRAM Read Command to First Data (t_AA), Most Significant Byte
    TAAMsb = 0x1f,
    /// SDRAM Activate to Read or Write Command Delay (t_RCD), Least Significant Byte
    TRCDLsb = 0x020,
    /// SDRAM Activate to Read or Write Command Delay (t_RCD), Most Significant Byte
    TRCDMsb = 0x021,
    /// SDRAM Row Precharge Time (t_RP), Least Significant Byte
    TRPLsb = 0x022,
    /// SDRAM Row Precharge Time (t_RP), Most Significant Byte
    TRPMsb = 0x023,
    /// SDRAM Activate to Precharge Command Period (t_RAS), Least Significant Byte
    TRASLsb = 0x24,
    /// SDRAM Activate to Precharge Command Period (t_RAS), Most Significant Byte
    TRASMsb = 0x25,
    /// SDRAM Activate to Activate or Refresh Command Period (t_RC), Least Significant Byte
    TRCLsb = 0x26,
    /// SDRAM Activate to Activate or Refresh Command Period (t_RC), Most Significant Byte
    TRCMsb = 0x27,
    /// SDRAM Write Recovery Time (t_WR), Least Significant Byte
    TWRLsb = 0x28,
    /// SDRAM Write Recovery Time (t_WR), Most Significant Byte
    TWRMsb = 0x29,
    /// SDRAM Normal Refresh Recovery Time (t_RFC1, tRFC1_^slr), Least Significant Byte
    TRFC1Lsb = 0x2a,
    /// SDRAM Normal Refresh Recovery Time (t_RFC1, tRFC1_^slr), Most Significant Byte
    TRFC1Msb = 0x2b,
    /// SDRAM Fine Granularity Refresh Recovery Time (t_RFC2, tRFC2_^slr), Least Significant Byte
    TRFC2Lsb = 0x2c,
    /// SDRAM Fine Granularity Refresh Recovery Time (t_RFc2, tRFC2_^slr), Most Significant Byte
    TRFC2Msb = 0x2d,
    /// SDRAM Same Bank Refresh Recovery Time (t_RFCsb, tRFCsb_^slr), Least Significant Byte
    TRFCsbLsb = 0x2e,
    /// SDRAM Same Bank Refresh Recovery Time (t_RFCsb_^dlr), Most Significant Byte
    TRFCsbMsb = 0x2f,
    /// SDRAM Normal Refresh Recovery Time, 3DS Different Logical Rank (t_RFC1_^dlr), Least Significant Byte
    TRFC1dlrLsb = 0x30,
    /// SDRAM Normal Refresh Recovery Time, 3DS Different Logical Rank (t_RFC1_^dlr), Most Significant Byte
    TRFC1dlrMsb = 0x31,
    /// SDRAM Fine Granularity Refresh Recovery Time, 3DS Different Logical Rank (t_RFC2_^dlr), Least Significant Byte
    TRFC2dlrLsb = 0x32,
    /// SDRAM Fine Granularity Refresh Recovery Time, 3DS Different Logical Rank (t_RFC2_^dlr), Most Significant Byte
    TRFC2dlrMsb = 0x33,
    /// SDRAM Same Bank Refresh Recovery Time, 3DS Different Logical Rank (t_RFCsb_^dlr), Least Significant Byte
    TRFCsbdlrLsb = 0x034,
    /// SDRAM Same Bank Refresh Recovery Time, 3DS Different Logical Rank (tRFCsb_^dlr), Most Significant Byte
    TRFCsbdlrMsb = 0x035,
    /// SDRAM Refresh Management, First Byte, First SDRAM
    FirstSDRAMRefreshManagement0 = 0x036,
    /// SDRAM Refresh Management, Second Byte, First SDRAM
    FirstSDRAMRefreshManagement1 = 0x037,
    /// SDRAM Refresh Management, First Byte, Second SDRAM
    SecondSDRAMRefreshManagement0 = 0x038,
    /// SDRAM Refresh Management, Second Byte, Second SDRAM
    SecondSDRAMRefreshManagement1 = 0x039,
    /// SDRAM Adaptive Refresh Management Level A, First Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtA0 = 0x03a,
    /// SDRAM Adaptive Refresh Management Level A, Second Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtA1 = 0x03B,
    /// SDRAM Adaptive Refresh Management Level A, First Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtA0 = 0x03C,
    /// SDRAM Adaptive Refresh Management Level A, Second Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtA1 = 0x03D,
    /// SDRAM Adaptive Refresh Management Level B, First Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtB0 = 0x03e,
    /// SDRAM Adaptive Refresh Management Level B, Second Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtB1 = 0x03F,
    /// SDRAM Adaptive Refresh Management Level B, First Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtB0 = 0x040,
    /// SDRAM Adaptive Refresh Management Level B, Second Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtB1 = 0x041,
    /// SDRAM Adaptive Refresh Management Level C, First Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtC0 = 0x042,
    /// SDRAM Adaptive Refresh Management Level C, Second Byte, First SDRAM
    FirstSDRAMAdaptiveRefreshMgmtC1 = 0x043,
    /// SDRAM Adaptive Refresh Management Level C, First Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtC0 = 0x044,
    /// SDRAM Adaptive Refresh Management Level C, Second Byte, Second SDRAM
    SecondSDRAMAdaptiveRefreshMgmtC1 = 0x045,
    /// SDRAM Activate to Activate Command Delay for Same Bank Group (t_RRD_L), Least Significant Byte
    TRDDLLsb = 0x046,
    /// SDRAM Activate to Activate Command Delay for Same Bank Group (t_RRD_L), Most Significant Byte
    TRDDLMsb = 0x047,
    /// SDRAM Activate to Activate Command Delay for Same Bank Group (t_RRD_L), Lower Clock Limit
    TRDDLLcl = 0x048,
    /// SDRAM Read to Read Command Delay for Same Bank Group (t_CCD_L), Least Significant Byte
    TCCDLLsb = 0x049,
    /// SDRAM Read to Read Command Delay for Same Bank Group (t_CCD_L), Most Significant Byte
    TCCDLMsb = 0x04A,
    /// SDRAM Read to Read Command Delay for Same Bank Group (t_CCD_L), Lower Clock Limit
    TCCDLLcl = 0x04B,
    /// SDRAM Write to Write Command Delay for Same Bank Group (t_CCD_L_WR), Least Significant Byte
    TCCDLWRLsb = 0x04C,
    /// SDRAM Write to Write Command Delay for Same Bank Group (t_CCD_L_WR), Most Significant Byte
    TCCDLWRMsb = 0x04D,
    /// SDRAM Write to Write Command Delay for Same Bank Group (t_CCD_L_WR), Lower Clock Limit
    TCCDLWRLcl = 0x04E,
    /// SDRAM Write to Write Command Delay for Same Bank Group, Second Write not RMW (t_CCD_L_WR2), Least Significant Byte
    TCCDLWR2Lsb = 0x04F,
    /// SDRAM Write to Write Command Delay for Same Bank Group, Second Write not RMW (t_CCD_L_WR2), Most Significant Byte
    TCCDLWR2Msb = 0x050,
    /// SDRAM Write to Write Command Delay for Same Bank Group, Second Write not RMW (t_CCD_L_WR2), Lower Clock Limit
    TCCDLWR2Lcl = 0x051,
    /// SDRAM Four Activate Window (t_raw), Least Significant Byte
    TrawLsb = 0x052,
    /// SDRAM Four Activate Window (t_rAw), Most Significant Byte
    TrawMsb = 0x053,
    /// SDRAM Four Activate Window (t_rAw), Lower Clock Limit
    TrawLcl = 0x054,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_L_WTR), Least Significant Byte
    TCCDLWTRLsb = 0x055,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_L_WTR), (CCD_L_ WTR), Most Significant Byte
    TCCDLWTRMsb = 0x056,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_L_WTR), Lower Clock Limit
    TCCDLWTRLcl = 0x057,
    /// SDRAM Write to Read Command Delay for Different Bank Group (t_CCD_S_WTR), Least Significant Byte
    TCCDSWTRLsb = 0x058,
    /// SDRAM Write to Read Command Delay for Different Bank Group (t_CCD_S_WTR), Most Significant Byte
    TCCDSWTRMsb = 0x059,
    /// SDRAM Write to Read Command Delay for Different Bank Group, (t_CCD_S_WTR), Lower Clock Limit
    TCCDSWTRLcl = 0x05A,
    /// SDRAM Read to Precharge Command Delay (t_RTP, t_RTP_^slr), Least Significant Byte
    TRTPLsb = 0x05B,
    /// SDRAM Read to Precharge Command Delay (t_RTP, t_RTP_^slr), Most Significant Byte
    TRTPMsb = 0x05C,
    /// SDRAM Read to Precharge Command Delay (t_RTP, t_RTP_^slr), Lower Clock Limit
    TRTPLcl = 0x05D,
    /// SDRAM Read to Read Command Delay for Different Bank in Same Bank Group (t_CCD_M), Least Significant Byte
    TCCDMLsb = 0x05E,
    /// SDRAM Read to Read Command Delay for Different Bank in Same Bank Group (t_CCD_M), Most Significant Byte
    TCCDMMsb = 0x05F,
    /// SDRAM Read to Read Command Delay for Different Bank in Same Bank Group (t_CCD_M), Lower Clock Limit
    TCCDMLcl = 0x060,
    /// SDRAM Write to Write Command Delay for Different Bank in Same Bank Group (t_CCD_M_WR), Least Significant Byte
    TCCDMWRLsb = 0x061,
    /// SDRAM Write to Write Command Delay for Different Bank in Same Bank Group (t_CCD_M_WR), Most Significant Byte
    TCCDMWRMsb = 0x062,
    /// SDRAM Write to Write Command Delay for Different Bank in Same Bank Group (t_CCD M WR), Lower Clock Limit
    TCCDMWRLcl = 0x063,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_M_WTR), Least Significant Byte
    TCCDMWTRLsb = 0x064,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_M_WTR), Most Significant Byte
    TCCDMWTRMsb = 0x065,
    /// SDRAM Write to Read Command Delay for Same Bank Group (t_CCD_M_WTR), Lower Clock Limit
    TCCDMWTRLcl = 0x066,

    /// Module Manufacturer ID Code, First Byte
    ModuleManufacturerIDCode0 = 0x200,
    /// Module Manufacturer ID Code, Second Byte
    ModuleManufacturerIDCode1 = 0x201,
    /// Module Manufacturing Location
    ModuleManufacturingLocation = 0x202,
    /// Module Manufacturing Year
    ModuleManufacturingDateYear = 0x203,
    /// Module Manufacturing Week
    ModuleManufacturingDateWeek = 0x204,
    /// Module Serial Number, First Byte
    ModuleSerialNumber0 = 0x205,
    /// Module Serial Number, Second Byte
    ModuleSerialNumber1 = 0x206,
    /// Module Serial Number, Third Byte
    ModuleSerialNumber2 = 0x207,
    /// Module Serial Number, Fourth Byte
    ModuleSerialNumber3 = 0x208,
    /// First byte of part number
    PartNumberBase = 0x209,
    /// End of part number (inclusive)
    PartNumberLimit = 0x226,
    /// DRAM Manufacturer ID Code, First Byte
    DRAMManufacturerIDCode0 = 0x228,
    /// DRAM Manufacturer ID Code, Second Byte
    DRAMManufacturerIDCode1 = 0x229,
    /// DRAM Stepping
    DRAMStepping = 0x22a,
}

impl Offset {
    pub fn to_usize(self) -> usize {
        self as usize
    }

    pub fn within(self, buf: &[u8]) -> u8 {
        buf[self as usize]
    }
}
