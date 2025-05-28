#![allow(clippy::approx_constant)]

#[cfg(feature = "defmt")]
use defmt::Format;

#[cfg(feature = "binrw")]
use binrw::meta::EndianKind;
#[cfg(feature = "binrw")]
use binrw::{BinRead, BinWrite, Endian}; // Added Endian // Added EndianKind

/// Represents the status of the BQ25730 charger.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargerStatus::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargerStatus {
    /// Input source status (STAT_AC)
    pub stat_ac: bool,
    /// ICO routine complete (ICO_DONE)
    pub ico_done: bool,
    /// Charger is operated in VAP mode (IN_VAP)
    pub in_vap: bool,
    /// Charger is in VINDPM or OTG voltage regulation (IN_VINDPM)
    pub in_vindpm: bool,
    /// Charger is in IIN_DPM (IN_IIN_DPM)
    pub in_iin_dpm: bool,
    /// Charger is in fast charge (IN_FCHRG)
    pub in_fchrg: bool,
    /// Charger is in pre-charge (IN_PCHRG)
    pub in_pchrg: bool,
    /// Charger is in OTG (IN_OTG)
    pub in_otg: bool,
    /// ACOV fault (Fault ACOV)
    pub fault_acov: bool,
    /// BATOC fault (Fault BATOC)
    pub fault_batoc: bool,
    /// ACOC fault (Fault ACOC)
    pub fault_acoc: bool,
    /// SYSOVP fault (Fault SYSOVP)
    pub fault_sysovp: bool,
    /// VSYS_UVP fault (Fault VSYS_UVP)
    pub fault_vsys_uvp: bool,
    /// Force converter off fault (Fault Force_Converter_Off)
    pub fault_force_converter_off: bool,
    /// OTG OVP fault (Fault_OTG_OVP)
    pub fault_otg_ovp: bool,
    /// OTG UVP fault (Fault_OTG_UVP)
    pub fault_otg_uvp: bool,
}

#[cfg(feature = "binrw")]
impl ChargerStatus {
    fn from_u16(value: u16) -> Self {
        Self {
            stat_ac: (value & (1 << 0)) != 0,
            ico_done: (value & (1 << 1)) != 0,
            in_vap: (value & (1 << 2)) != 0,
            in_vindpm: (value & (1 << 3)) != 0,
            in_iin_dpm: (value & (1 << 4)) != 0,
            in_fchrg: (value & (1 << 5)) != 0,
            in_pchrg: (value & (1 << 6)) != 0,
            in_otg: (value & (1 << 7)) != 0,
            fault_acov: (value & (1 << 8)) != 0,
            fault_batoc: (value & (1 << 9)) != 0,
            fault_acoc: (value & (1 << 10)) != 0,
            fault_sysovp: (value & (1 << 11)) != 0,
            fault_vsys_uvp: (value & (1 << 12)) != 0,
            fault_force_converter_off: (value & (1 << 13)) != 0,
            fault_otg_ovp: (value & (1 << 14)) != 0,
            fault_otg_uvp: (value & (1 << 15)) != 0,
        }
    }

    fn to_u16(&self) -> u16 {
        let mut value = 0;
        if self.stat_ac {
            value |= 1 << 0;
        }
        if self.ico_done {
            value |= 1 << 1;
        }
        if self.in_vap {
            value |= 1 << 2;
        }
        if self.in_vindpm {
            value |= 1 << 3;
        }
        if self.in_iin_dpm {
            value |= 1 << 4;
        }
        if self.in_fchrg {
            value |= 1 << 5;
        }
        if self.in_pchrg {
            value |= 1 << 6;
        }
        if self.in_otg {
            value |= 1 << 7;
        }
        if self.fault_acov {
            value |= 1 << 8;
        }
        if self.fault_batoc {
            value |= 1 << 9;
        }
        if self.fault_acoc {
            value |= 1 << 10;
        }
        if self.fault_sysovp {
            value |= 1 << 11;
        }
        if self.fault_vsys_uvp {
            value |= 1 << 12;
        }
        if self.fault_force_converter_off {
            value |= 1 << 13;
        }
        if self.fault_otg_ovp {
            value |= 1 << 14;
        }
        if self.fault_otg_uvp {
            value |= 1 << 15;
        }
        value
    }
}

/// Represents the PROCHOT status of the BQ25730.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ProchotStatus::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ProchotStatus {
    /// PROCHOT Pulse Extension Enable (EN_PROCHOT_EXT)
    pub en_prochot_ext: bool,
    /// PROCHOT Pulse Width (PROCHOT_WIDTH)
    pub prochot_width: u8, // Represents 00b, 01b, 10b, 11b
    /// PROCHOT Pulse Clear (PROCHOT_CLEAR)
    pub prochot_clear: bool,
    /// VAP failure status (STAT_VAP_FAIL)
    pub stat_vap_fail: bool,
    /// Exit VAP status (STAT_EXIT_VAP)
    pub stat_exit_vap: bool,
    /// VINDPM PROCHOT Profile status (STAT_VINDPM)
    pub stat_vindpm: bool,
    /// CMPOUT PROCHOT Profile status (STAT_COMP)
    pub stat_comp: bool,
    /// ICRIT PROCHOT Profile status (STAT_ICRIT)
    pub stat_icrit: bool,
    /// INOM PROCHOT Profile status (STAT_INOM)
    pub stat_inom: bool,
    /// IDCHG1 PROCHOT Profile status (STAT_IDCHG1)
    pub stat_idchg1: bool,
    /// VSYS PROCHOT Profile status (STAT_VSYS)
    pub stat_vsys: bool,
    /// Battery removal PROCHOT Profile status (STAT_BAT_REMOVAL)
    pub stat_bat_removal: bool,
    /// Adapter removal PROCHOT Profile status (STAT_ADPT_REMOVAL)
    pub stat_adpt_removal: bool,
    /// IDCHG2 PROCHOT Profile status (STAT_IDCHG2)
    pub stat_idchg2: bool,
    /// PTM PROCHOT Profile status (STAT_PTM)
    pub stat_ptm: bool,
}

#[cfg(feature = "binrw")]
impl ProchotStatus {
    fn from_u16(value: u16) -> Self {
        Self {
            en_prochot_ext: (value & (1 << 0)) != 0,
            prochot_width: ((value >> 1) & 0x03) as u8, // Bits 2:1
            prochot_clear: (value & (1 << 3)) != 0,
            stat_vap_fail: (value & (1 << 4)) != 0,
            stat_exit_vap: (value & (1 << 5)) != 0,
            stat_vindpm: (value & (1 << 6)) != 0,
            stat_comp: (value & (1 << 7)) != 0,
            stat_icrit: (value & (1 << 8)) != 0,
            stat_inom: (value & (1 << 9)) != 0,
            stat_idchg1: (value & (1 << 10)) != 0,
            stat_vsys: (value & (1 << 11)) != 0,
            stat_bat_removal: (value & (1 << 12)) != 0,
            stat_adpt_removal: (value & (1 << 13)) != 0,
            stat_idchg2: (value & (1 << 14)) != 0,
            stat_ptm: (value & (1 << 15)) != 0,
        }
    }

    fn to_u16(&self) -> u16 {
        let mut value = 0;
        if self.en_prochot_ext {
            value |= 1 << 0;
        }
        value |= ((self.prochot_width & 0x03) as u16) << 1;
        if self.prochot_clear {
            value |= 1 << 3;
        }
        if self.stat_vap_fail {
            value |= 1 << 4;
        }
        if self.stat_exit_vap {
            value |= 1 << 5;
        }
        if self.stat_vindpm {
            value |= 1 << 6;
        }
        if self.stat_comp {
            value |= 1 << 7;
        }
        if self.stat_icrit {
            value |= 1 << 8;
        }
        if self.stat_inom {
            value |= 1 << 9;
        }
        if self.stat_idchg1 {
            value |= 1 << 10;
        }
        if self.stat_vsys {
            value |= 1 << 11;
        }
        if self.stat_bat_removal {
            value |= 1 << 12;
        }
        if self.stat_adpt_removal {
            value |= 1 << 13;
        }
        if self.stat_idchg2 {
            value |= 1 << 14;
        }
        if self.stat_ptm {
            value |= 1 << 15;
        }
        value
    }
}

/// Represents the Charge Current setting in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeCurrent::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeCurrent(pub u16);

#[cfg(feature = "binrw")]
impl ChargeCurrent {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeCurrent::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeCurrent {
    /// LSB value for Charge Current in mA (with 5mΩ sense resistor).
    pub const LSB_MA: u16 = 128; // 128mA/LSB for 5mΩ sense resistor

    /// Creates a new ChargeCurrent from raw LSB and MSB register values.
    /// The 7-bit value (D6-D0) is formed by:
    /// MSB (0x03): D6-D2 in bits 4:0
    /// LSB (0x02): D1-D0 in bits 7:6
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        // D6-D2 are in msb bits 4:0
        // D1-D0 are in lsb bits 7:6
        let d6_d2 = (msb & 0x1F) as u16; // Extract bits 4:0 from msb
        let d1_d0 = ((lsb >> 6) & 0x03) as u16; // Extract bits 7:6 from lsb

        // Combine them to form a 7-bit raw_value (D6 D5 D4 D3 D2 D1 D0)
        let raw_value = (d6_d2 << 2) | d1_d0;
        ChargeCurrent(raw_value * Self::LSB_MA)
    }

    /// Converts the ChargeCurrent to raw MSB and LSB register values.
    /// The 7-bit value (D6-D0) is formed by:
    /// MSB (0x03): D6-D2 in bits 4:0
    /// LSB (0x02): D1-D0 in bits 7:6
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MA;
        // raw_value is a 7-bit value (D6-D0)
        // msb (0x03) bits 4:0 should be D6-D2
        // lsb (0x02) bits 7:6 should be D1-D0
        let msb = ((raw_value >> 2) & 0x1F) as u8; // D6-D2
        let lsb = ((raw_value & 0x03) << 6) as u8; // D1-D0 shifted to bits 7:6
        (lsb, msb)
    }
    /// Converts the ChargeCurrent to milliamps.
    pub fn to_milliamps(&self) -> u16 {
        self.0
    }
}

/// Represents the Charge Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeVoltage::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeVoltage(pub u16);

#[cfg(feature = "binrw")]
impl ChargeVoltage {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeVoltage::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeVoltage {
    /// LSB value for Charge Voltage in mV.
    pub const LSB_MV: u16 = 8; // 8mV/LSB

    /// Creates a new ChargeVoltage from raw LSB and MSB register values.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x05): D11-D5 in bits 6:0
    /// LSB (0x04): D4-D0 in bits 4:0
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        // D11-D5 are in msb bits 6:0
        // D4-D0 are in lsb bits 4:0
        let d11_d5 = (msb & 0x7F) as u16; // Extract bits 6:0 from msb
        let d4_d0 = (lsb & 0x1F) as u16; // Extract bits 4:0 from lsb

        // Combine them to form a 12-bit raw_value (D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0)
        let raw_value = (d11_d5 << 5) | d4_d0;
        ChargeVoltage(raw_value * Self::LSB_MV)
    }

    /// Converts the ChargeVoltage to raw MSB and LSB register values.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x05): D11-D5 in bits 6:0
    /// LSB (0x04): D4-D0 in bits 4:0
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MV;
        // raw_value is a 12-bit value (D11-D0)
        // msb (0x05) bits 6:0 should be D11-D5
        // lsb (0x04) bits 4:0 should be D4-D0
        let msb = ((raw_value >> 5) & 0x7F) as u8; // D11-D5
        let lsb = (raw_value & 0x1F) as u8; // D4-D0
        (lsb, msb)
    }
    /// Converts the ChargeVoltage to millivolts.
    pub fn to_millivolts(&self) -> u16 {
        self.0
    }
}

/// Represents the OTG Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = OtgVoltage::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct OtgVoltage(pub u16);

#[cfg(feature = "binrw")]
impl OtgVoltage {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        OtgVoltage::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl OtgVoltage {
    /// LSB value for OTG Voltage in mV.
    pub const LSB_MV: u16 = 8; // 8mV/LSB based on datasheet 7.5

    /// Creates a new OtgVoltage from raw LSB and MSB register values.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x07): D11-D5 in bits 6:0
    /// LSB (0x06): D4-D0 in bits 7:3
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        // D11-D5 are in msb bits 6:0
        // D4-D0 are in lsb bits 7:3
        let d11_d5 = (msb & 0x7F) as u16; // Extract bits 6:0 from msb
        let d4_d0 = ((lsb >> 3) & 0x1F) as u16; // Extract bits 7:3 from lsb

        // Combine them to form a 12-bit raw_value (D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0)
        let raw_value = (d11_d5 << 5) | d4_d0;
        OtgVoltage(raw_value * Self::LSB_MV)
    }

    /// Converts the OtgVoltage to raw MSB and LSB register values.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x07): D11-D5 in bits 6:0
    /// LSB (0x06): D4-D0 in bits 7:3
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MV;
        // raw_value is a 12-bit value (D11-D0)
        // msb (0x07) bits 6:0 should be D11-D5
        // lsb (0x06) bits 7:3 should be D4-D0
        let msb = ((raw_value >> 5) & 0x7F) as u8; // D11-D5
        let lsb = ((raw_value & 0x1F) << 3) as u8; // D4-D0 shifted to bits 7:3
        (lsb, msb)
    }
}

/// Represents the OTG Current setting in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = OtgCurrent::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct OtgCurrent(pub u16);

#[cfg(feature = "binrw")]
impl OtgCurrent {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        OtgCurrent::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl OtgCurrent {
    /// LSB value for OTG Current in mA (with 5mΩ sense resistor).
    pub const LSB_MA: u16 = 100;

    /// Creates a new OtgCurrent from raw MSB and LSB register values.
    /// The 7-bit value (D6-D0) is formed by:
    /// MSB (0x09): D6-D0 in bits 6:0
    /// LSB (0x08): Reserved
    pub fn from_register_value(_lsb: u8, msb: u8) -> Self {
        let raw_value = (msb & 0x7F) as u16; // D6-D0
        OtgCurrent(raw_value * Self::LSB_MA)
    }

    /// Converts the OtgCurrent to raw MSB and LSB register values.
    /// The 7-bit value (D6-D0) is formed by:
    /// MSB (0x09): D6-D0 in bits 6:0
    /// LSB (0x08): Reserved
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MA;
        // OTGCurrent is a 7-bit value (D6-D0)
        // MSB (0x09): D6-D0 in bits 6:0
        // LSB (0x08): Reserved
        let msb = (raw_value & 0x7F) as u8; // D6-D0 in bits 6:0 of MSB (0x09)
        let lsb = 0x00; // LSB (0x08) is reserved, write 0
        (lsb, msb)
    }
    /// Converts the OtgCurrent to a raw 8-bit register value (MSB part).
    pub fn to_register_value(&self) -> u8 {
        let raw_value = self.0 / Self::LSB_MA;
        (raw_value & 0x7F) as u8 // Return MSB part (D6-D0)
    }
}

/// Represents the Input Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = InputVoltage::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct InputVoltage(pub u16);

#[cfg(feature = "binrw")]
impl InputVoltage {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        InputVoltage::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl InputVoltage {
    /// LSB value for Input Voltage in mV.
    pub const LSB_MV: u16 = 64;
    /// Offset value for Input Voltage in mV.
    pub const OFFSET_MV: u16 = 3200; // 3200mV offset (from 3.2V)

    /// Creates a new InputVoltage from a raw 9-bit register value.
    /// The value is stored across two bytes (MSB at 0x0B, LSB at 0x0A).
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        // D8 is in bit 5 of MSB (0x0B)
        let raw_value = ((((msb >> 5) & 0x01) as u16) << 8) | (lsb as u16); // D8-D0 (D8 is bit 5 of MSB)
        InputVoltage(raw_value * Self::LSB_MV + Self::OFFSET_MV)
    }

    /// Converts the InputVoltage to raw MSB and LSB register values.
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = (self.0 - Self::OFFSET_MV) / Self::LSB_MV;
        let msb = (((raw_value >> 8) & 0x01) << 5) as u8; // D8 in bit 5 of MSB (0x0B)
        let lsb = (raw_value & 0xFF) as u8; // D7-D0 in bits 7:0 of LSB (0x0A)
        (lsb, msb) // LSB, MSB
    }
    /// Converts the InputVoltage to a raw 8-bit register value.
    /// This function is likely not used for InputVoltage as it's a 2-byte register.
    /// However, if it were to return the LSB part, it would be:
    pub fn to_register_value(&self) -> u8 {
        let raw_value = (self.0 - Self::OFFSET_MV) / Self::LSB_MV;
        (raw_value & 0xFF) as u8 // Return LSB part
    }
}

/// Represents the Minimum System Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = VsysMin::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct VsysMin(pub u16);

#[cfg(feature = "binrw")]
impl VsysMin {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        // VsysMin only uses the MSB for its value, LSB is 0x00
        VsysMin::from_register_value(bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl VsysMin {
    /// LSB value for Minimum System Voltage in mV.
    pub const LSB_MV: u16 = 100;

    /// Creates a new VsysMin from a raw 8-bit register value (0x0C).
    pub fn from_register_value(msb: u8) -> Self {
        VsysMin((msb as u16) * Self::LSB_MV)
    }

    /// Converts the VsysMin to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }

    /// Converts the VsysMin to raw MSB and LSB register values.
    /// Since VsysMin is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, self.to_register_value())
    }
}

/// Represents the Input Current Limit Set by Host in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = IinHost::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct IinHost(pub u16);

#[cfg(feature = "binrw")]
impl IinHost {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        // IinHost only uses the MSB for its value, LSB is 0x00
        IinHost::from_register_value(bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl IinHost {
    /// LSB value for Input Current Limit Set by Host in mA.
    pub const LSB_MA: u16 = 100; // 100mA/LSB for 5mΩ sense resistor
    /// Offset value for Input Current Limit Set by Host in mA.
    pub const OFFSET_MA: u16 = 100; // 100mA offset at code 0

    /// Creates a new IinHost from a raw 8-bit register value (LSB at 0x0E).
    pub fn from_register_value(msb: u8) -> Self {
        IinHost(((msb & 0x7F) as u16) * Self::LSB_MA + Self::OFFSET_MA)
    }

    /// Converts the IinHost to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        // Ensure the value is not less than the offset to prevent overflow
        let raw_value = if self.0 >= Self::OFFSET_MA {
            (self.0 - Self::OFFSET_MA) / Self::LSB_MA
        } else {
            0 // Clamp to the minimum register value (corresponding to OFFSET_MA)
        };
        raw_value as u8
    }
}

impl IinHost {
    /// Converts the IinHost to raw MSB and LSB register values.
    /// Since IinHost is an 8-bit register, MSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, self.to_register_value())
    }
}

/// Represents the Input Current Limit in Use (IIN_DPM) in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = IinDpm::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct IinDpm(pub u16);

#[cfg(feature = "binrw")]
impl IinDpm {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        // IinDpm only uses the MSB for its value, LSB is 0x00
        IinDpm::from_register_value(bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl IinDpm {
    /// LSB value for Input Current Limit in Use in mA.
    pub const LSB_MA: u16 = 100; // 100mA/LSB for 5mΩ sense resistor
    /// Offset value for Input Current Limit in Use in mA.
    pub const OFFSET_MA: u16 = 100; // 100mA offset at code 0

    /// Creates a new IinDpm from a raw 8-bit register value (LSB at 0x24).
    pub fn from_register_value(msb: u8) -> Self {
        IinDpm(((msb & 0x7F) as u16) * Self::LSB_MA + Self::OFFSET_MA)
    }

    /// Converts the IinDpm to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        // Ensure the value is not less than the offset to prevent overflow
        let raw_value = if self.0 >= Self::OFFSET_MA {
            (self.0 - Self::OFFSET_MA) / Self::LSB_MA
        } else {
            0 // Clamp to the minimum register value (corresponding to OFFSET_MA)
        };
        raw_value as u8
    }
}

impl IinDpm {
    /// Converts the IinDpm to raw MSB and LSB register values.
    /// Since IinDpm is an 8-bit register, MSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, self.to_register_value())
    }
}

/// Represents the ADC PSYS measurement in mW.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcPsys::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcPsys(pub u16);

#[cfg(feature = "binrw")]
impl AdcPsys {
    fn from_bin_byte(byte: u8) -> Self {
        AdcPsys::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcPsys {
    /// LSB value for ADC PSYS in mV.
    pub const LSB_MV: u16 = 12; // 12mV/LSB for ADC_FULLSCALE=1b

    /// Creates a new AdcPsys from a raw 8-bit register value (0x26).
    pub fn from_register_value(value: u8) -> Self {
        AdcPsys((value as u16) * Self::LSB_MV)
    }

    /// Converts the AdcPsys to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents the ADC VBUS measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcVbus::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcVbus(pub u16);

#[cfg(feature = "binrw")]
impl AdcVbus {
    fn from_bin_byte(byte: u8) -> Self {
        AdcVbus::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcVbus {
    /// LSB value for ADC VBUS in mV.
    pub const LSB_MV: u16 = 96; // 96mV/LSB

    /// Creates a new AdcVbus from a raw 8-bit register value (0x27).
    pub fn from_register_value(value: u8) -> Self {
        AdcVbus((value as u16) * Self::LSB_MV)
    }

    /// Converts the AdcVbus to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents the ADC IDCHG measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcIdchg::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcIdchg(pub u16);

#[cfg(feature = "binrw")]
impl AdcIdchg {
    fn from_bin_byte(byte: u8) -> Self {
        AdcIdchg::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcIdchg {
    /// LSB value for ADC IDCHG in mA.
    pub const LSB_MA: u16 = 512; // 512mA/LSB for 5mΩ sense resistor

    /// Creates a new AdcIdchg from a raw 8-bit register value (0x28).
    pub fn from_register_value(value: u8) -> Self {
        AdcIdchg((value as u16) * Self::LSB_MA)
    }

    /// Converts the AdcIdchg to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MA) as u8
    }
}

/// Represents the ADC ICHG measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcIchg::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcIchg(pub u16);

#[cfg(feature = "binrw")]
impl AdcIchg {
    fn from_bin_byte(byte: u8) -> Self {
        AdcIchg::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcIchg {
    /// LSB value for ADC ICHG in mA.
    pub const LSB_MA: u16 = 128; // 128mA/LSB for 5mΩ sense resistor

    /// Creates a new AdcIchg from a raw 8-bit register value (0x29).
    pub fn from_register_value(value: u8) -> Self {
        AdcIchg((value as u16) * Self::LSB_MA)
    }

    /// Converts the AdcIchg to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MA) as u8
    }
}

/// Represents the ADC CMPIN measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcCmpin::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcCmpin(pub u16);

#[cfg(feature = "binrw")]
impl AdcCmpin {
    fn from_bin_byte(byte: u8) -> Self {
        AdcCmpin::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcCmpin {
    /// LSB value for ADC CMPIN in mV.
    pub const LSB_MV: u16 = 12; // 12mV/LSB for ADC_FULLSCALE=1b

    /// Creates a new AdcCmpin from a raw 8-bit register value (0x2A).
    pub fn from_register_value(value: u8) -> Self {
        AdcCmpin((value as u16) * Self::LSB_MV)
    }

    /// Converts the AdcCmpin to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents the ADC IIN measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcIin::from_bin_byte))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_byte()))]
pub struct AdcIin(pub u16);

#[cfg(feature = "binrw")]
impl AdcIin {
    fn from_bin_byte(byte: u8) -> Self {
        AdcIin::from_register_value(byte)
    }

    fn to_bin_byte(&self) -> u8 {
        self.to_register_value()
    }
}

impl AdcIin {
    /// LSB value for ADC IIN in mA.
    pub const LSB_MA: u16 = 100; // 100mA/LSB for 5mΩ sense resistor

    /// Creates a new AdcIin from a raw 8-bit register value (0x2B).
    pub fn from_register_value(value: u8) -> Self {
        AdcIin((value as u16) * Self::LSB_MA)
    }

    /// Converts the AdcIin to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MA) as u8
    }
}

/// Represents the ADC VBAT measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
// Removed #[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))] to avoid conflicting implementations
pub struct AdcVbat(pub u16);

#[cfg(feature = "binrw")]
impl binrw::meta::ReadEndian for AdcVbat {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
#[cfg(feature = "binrw")]
impl binrw::meta::WriteEndian for AdcVbat {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
#[cfg(feature = "binrw")]
impl BinRead for AdcVbat {
    type Args<'a> = (); // Added lifetime parameter

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>, // Used lifetime parameter
    ) -> binrw::BinResult<Self> {
        let value = u8::read_options(reader, endian, ())?;
        // For binrw, we assume offset_mv is 0 for simplicity in serialization/deserialization.
        // In a real application, this offset might be dynamic or passed as BinRead::Args.
        Ok(AdcVbat::from_register_value(value, 0))
    }
}

#[cfg(feature = "binrw")]
impl BinWrite for AdcVbat {
    type Args<'a> = (); // Added lifetime parameter

    fn write_options<W: binrw::io::Write + binrw::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>, // Used lifetime parameter
    ) -> binrw::BinResult<()> {
        self.to_register_value().write_options(writer, endian, ())
    }
}

impl AdcVbat {
    /// LSB value for ADC VBAT in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcVbat from a raw 8-bit register value (0x2C) and an offset.
    pub fn from_register_value(value: u8, offset_mv: u16) -> Self {
        AdcVbat((value as u16) * Self::LSB_MV + offset_mv)
    }

    /// Converts the AdcVbat to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents the ADC VSYS measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
// Removed #[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))] to avoid conflicting implementations
pub struct AdcVsys(pub u16);

#[cfg(feature = "binrw")]
impl binrw::meta::ReadEndian for AdcVsys {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
#[cfg(feature = "binrw")]
impl binrw::meta::WriteEndian for AdcVsys {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}
#[cfg(feature = "binrw")]
impl BinRead for AdcVsys {
    type Args<'a> = (); // Added lifetime parameter

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>, // Used lifetime parameter
    ) -> binrw::BinResult<Self> {
        let value = u8::read_options(reader, endian, ())?;
        // For binrw, we assume offset_mv is 0 for simplicity in serialization/deserialization.
        // In a real application, this offset might be dynamic or passed as BinRead::Args.
        Ok(AdcVsys::from_register_value(value, 0))
    }
}

#[cfg(feature = "binrw")]
impl BinWrite for AdcVsys {
    type Args<'a> = (); // Added lifetime parameter

    fn write_options<W: binrw::io::Write + binrw::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>, // Used lifetime parameter
    ) -> binrw::BinResult<()> {
        self.to_register_value().write_options(writer, endian, ())
    }
}

impl AdcVsys {
    /// LSB value for ADC VSYS in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcVsys from a raw 8-bit register value (0x2D) and an offset.
    pub fn from_register_value(value: u8, offset_mv: u16) -> Self {
        AdcVsys((value as u16) * Self::LSB_MV + offset_mv)
    }

    /// Converts the AdcVsys to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents all ADC measurements.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
pub struct AdcMeasurements {
    pub psys: AdcPsys,
    pub vbus: AdcVbus,
    pub idchg: AdcIdchg,
    pub ichg: AdcIchg,
    pub cmpin: AdcCmpin,
    pub iin: AdcIin,
    pub vbat: AdcVbat,
    pub vsys: AdcVsys,
}

#[cfg(feature = "binrw")]
impl binrw::meta::ReadEndian for AdcMeasurements {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

#[cfg(feature = "binrw")]
impl binrw::meta::WriteEndian for AdcMeasurements {
    const ENDIAN: EndianKind = EndianKind::Endian(Endian::Little);
}

impl AdcMeasurements {
    // This function is no longer needed as AdcMeasurements is constructed directly in Bq25730::read_adc_measurements
}

/// Represents the ADCOption register (0x3B/0x3A).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcOption::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct AdcOption {
    /// ADC conversion mode (ADC_CONV)
    pub adc_conv: bool, // 0b: One-shot, 1b: Continuous
    /// Start ADC conversion (ADC_START)
    pub adc_start: bool,
    /// ADC input voltage range adjustment for PSYS and CMPIN (ADC_FULLSCALE)
    pub adc_fullscale: bool, // 0b: 2.04V, 1b: 3.06V
    /// Enable ADC CMPIN channel (EN_ADC_CMPIN)
    pub en_adc_cmpin: bool,
    /// Enable ADC VBUS channel (EN_ADC_VBUS)
    pub en_adc_vbus: bool,
    /// Enable ADC PSYS channel (EN_ADC_PSYS)
    pub en_adc_psys: bool,
    /// Enable ADC IIN channel (EN_ADC_IIN)
    pub en_adc_iin: bool,
    /// Enable ADC IDCHG channel (EN_ADC_IDCHG)
    pub en_adc_idchg: bool,
    /// Enable ADC ICHG channel (EN_ADC_ICHG)
    pub en_adc_ichg: bool,
    /// Enable ADC VSYS channel (EN_ADC_VSYS)
    pub en_adc_vsys: bool,
    /// Enable ADC VBAT channel (EN_ADC_VBAT)
    pub en_adc_vbat: bool,
}

#[cfg(feature = "binrw")]
impl AdcOption {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        AdcOption::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl AdcOption {
    /// Creates a new AdcOption from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            adc_conv: (msb & 0x80) != 0,      // Bit 7 of MSB (0x3B)
            adc_start: (msb & 0x40) != 0,     // Bit 6 of MSB (0x3B)
            adc_fullscale: (msb & 0x20) != 0, // Bit 5 of MSB (0x3B)
            en_adc_cmpin: (lsb & 0x80) != 0,  // Bit 7 of LSB (0x3A)
            en_adc_vbus: (lsb & 0x40) != 0,   // Bit 6 of LSB (0x3A)
            en_adc_psys: (lsb & 0x20) != 0,   // Bit 5 of LSB (0x3A)
            en_adc_iin: (lsb & 0x10) != 0,    // Bit 4 of LSB (0x3A)
            en_adc_idchg: (lsb & 0x08) != 0,  // Bit 3 of LSB (0x3A)
            en_adc_ichg: (lsb & 0x04) != 0,   // Bit 2 of LSB (0x3A)
            en_adc_vsys: (lsb & 0x02) != 0,   // Bit 1 of LSB (0x3A)
            en_adc_vbat: (lsb & 0x01) != 0,   // Bit 0 of LSB (0x3A)
        }
    }

    /// Converts the AdcOption to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let msb = (if self.adc_conv { 0x80 } else { 0 })
            | (if self.adc_start { 0x40 } else { 0 })
            | (if self.adc_fullscale { 0x20 } else { 0 });

        let lsb = (if self.en_adc_cmpin { 0x80 } else { 0 })
            | (if self.en_adc_vbus { 0x40 } else { 0 })
            | (if self.en_adc_psys { 0x20 } else { 0 })
            | (if self.en_adc_iin { 0x10 } else { 0 })
            | (if self.en_adc_idchg { 0x08 } else { 0 })
            | (if self.en_adc_ichg { 0x04 } else { 0 })
            | (if self.en_adc_vsys { 0x02 } else { 0 })
            | (if self.en_adc_vbat { 0x01 } else { 0 });

        (lsb, msb)
    }
}

/// Represents the ChargeOption0 register (0x3C/0x3D).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption0::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeOption0 {
    /// Input current optimization (EN_ICO_MODE)
    pub en_ico_mode: bool,
    /// Enable PROCHOT on ACOV (EN_ACOV)
    pub en_acov: bool,
    /// Enable PROCHOT on BATOC (EN_BATOC)
    pub en_batoc: bool,
    /// Enable PROCHOT on ACOC (EN_ACOC)
    pub en_acoc: bool,
    /// Enable PROCHOT on SYSOVP (EN_SYSOVP)
    pub en_sysovp: bool,
    /// Enable PROCHOT on VSYS_UVP (EN_VSYS_UVP)
    pub en_vsys_uvp: bool,
    /// Enable PROCHOT on Force Converter Off (EN_FORCE_CONVERTER_OFF)
    pub en_force_converter_off: bool,
    /// Enable PROCHOT on OTG OVP (EN_OTG_OVP)
    pub en_otg_ovp: bool,
    /// Enable PROCHOT on OTG UVP (EN_OTG_UVP)
    pub en_otg_uvp: bool,
    /// Enable PROCHOT on VAP (EN_VAP)
    pub en_vap: bool,
    /// Enable PROCHOT on VINDPM (EN_VINDPM)
    pub en_vindpm: bool,
    /// Enable PROCHOT on CMPOUT (EN_COMP)
    pub en_comp: bool,
    /// Enable PROCHOT on ICRIT (EN_ICRIT)
    pub en_icrit: bool,
    /// Enable PROCHOT on INOM (EN_INOM)
    pub en_inom: bool,
    /// Enable PROCHOT on IDCHG1 (EN_IDCHG1)
    pub en_idchg1: bool,
    /// Enable PROCHOT on VSYS (EN_VSYS)
    pub en_vsys: bool,
}

#[cfg(feature = "binrw")]
impl ChargeOption0 {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeOption0::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeOption0 {
    /// Creates a new ChargeOption0 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_ico_mode: (msb & 0x80) != 0,
            en_acov: (msb & 0x40) != 0,
            en_batoc: (msb & 0x20) != 0,
            en_acoc: (msb & 0x10) != 0,
            en_sysovp: (msb & 0x08) != 0,
            en_vsys_uvp: (msb & 0x04) != 0,
            en_force_converter_off: (msb & 0x02) != 0,
            en_otg_ovp: (msb & 0x01) != 0,
            en_otg_uvp: (lsb & 0x80) != 0,
            en_vap: (lsb & 0x40) != 0,
            en_vindpm: (lsb & 0x20) != 0,
            en_comp: (lsb & 0x10) != 0,
            en_icrit: (lsb & 0x08) != 0,
            en_inom: (lsb & 0x04) != 0,
            en_idchg1: (lsb & 0x02) != 0,
            en_vsys: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption0 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let msb = (if self.en_ico_mode { 0x80 } else { 0 })
            | (if self.en_acov { 0x40 } else { 0 })
            | (if self.en_batoc { 0x20 } else { 0 })
            | (if self.en_acoc { 0x10 } else { 0 })
            | (if self.en_sysovp { 0x08 } else { 0 })
            | (if self.en_vsys_uvp { 0x04 } else { 0 })
            | (if self.en_force_converter_off { 0x02 } else { 0 })
            | (if self.en_otg_ovp { 0x01 } else { 0 });

        let lsb = (if self.en_otg_uvp { 0x80 } else { 0 })
            | (if self.en_vap { 0x40 } else { 0 })
            | (if self.en_vindpm { 0x20 } else { 0 })
            | (if self.en_comp { 0x10 } else { 0 })
            | (if self.en_icrit { 0x08 } else { 0 })
            | (if self.en_inom { 0x04 } else { 0 })
            | (if self.en_idchg1 { 0x02 } else { 0 })
            | (if self.en_vsys { 0x01 } else { 0 });

        (lsb, msb)
    }
}

/// Represents the ChargeOption1 register (0x3E/0x3F).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption1::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeOption1 {
    /// Enable PROCHOT on PTM (EN_PTM)
    pub en_ptm: bool,
    /// Enable PROCHOT on IDCHG2 (EN_IDCHG2)
    pub en_idchg2: bool,
    /// Enable PROCHOT on Battery Removal (EN_BAT_REMOVAL)
    pub en_bat_removal: bool,
    /// Enable PROCHOT on Adapter Removal (EN_ADPT_REMOVAL)
    pub en_adpt_removal: bool,
    /// Enable PROCHOT on VSYS (EN_VSYS_PROCHOT)
    pub en_vsys_prochot: bool,
    /// Enable PROCHOT on IDCHG1 (EN_IDCHG1_PROCHOT)
    pub en_idchg1_prochot: bool,
    /// Enable PROCHOT on INOM (EN_INOM_PROCHOT)
    pub en_inom_prochot: bool,
    /// Enable PROCHOT on ICRIT (EN_ICRIT_PROCHOT)
    pub en_icrit_prochot: bool,
    /// Enable PROCHOT on CMPOUT (EN_COMP_PROCHOT)
    pub en_comp_prochot: bool,
    /// Enable PROCHOT on VINDPM (EN_VINDPM_PROCHOT)
    pub en_vindpm_prochot: bool,
    /// Enable PROCHOT on VAP (EN_VAP_PROCHOT)
    pub en_vap_prochot: bool,
    /// Enable PROCHOT on OTG UVP (EN_OTG_UVP_PROCHOT)
    pub en_otg_uvp_prochot: bool,
    /// Enable PROCHOT on OTG OVP (EN_OTG_OVP_PROCHOT)
    pub en_otg_ovp_prochot: bool,
    /// Enable PROCHOT on Force Converter Off (EN_FORCE_CONVERTER_OFF_PROCHOT)
    pub en_force_converter_off_prochot: bool,
    /// Enable PROCHOT on VSYS_UVP (EN_VSYS_UVP_PROCHOT)
    pub en_vsys_uvp_prochot: bool,
    /// Enable PROCHOT on SYSOVP (EN_SYSOVP_PROCHOT)
    pub en_sysovp_prochot: bool,
}

#[cfg(feature = "binrw")]
impl ChargeOption1 {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeOption1::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeOption1 {
    /// Creates a new ChargeOption1 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_ptm: (msb & 0x80) != 0,
            en_idchg2: (msb & 0x40) != 0,
            en_bat_removal: (msb & 0x20) != 0,
            en_adpt_removal: (msb & 0x10) != 0,
            en_vsys_prochot: (msb & 0x08) != 0,
            en_idchg1_prochot: (msb & 0x04) != 0,
            en_inom_prochot: (msb & 0x02) != 0,
            en_icrit_prochot: (msb & 0x01) != 0,
            en_comp_prochot: (lsb & 0x80) != 0,
            en_vindpm_prochot: (lsb & 0x40) != 0,
            en_vap_prochot: (lsb & 0x20) != 0,
            en_otg_uvp_prochot: (lsb & 0x10) != 0,
            en_otg_ovp_prochot: (lsb & 0x08) != 0,
            en_force_converter_off_prochot: (lsb & 0x04) != 0,
            en_vsys_uvp_prochot: (lsb & 0x02) != 0,
            en_sysovp_prochot: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption1 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let msb = (if self.en_ptm { 0x80 } else { 0 })
            | (if self.en_idchg2 { 0x40 } else { 0 })
            | (if self.en_bat_removal { 0x20 } else { 0 })
            | (if self.en_adpt_removal { 0x10 } else { 0 })
            | (if self.en_vsys_prochot { 0x08 } else { 0 })
            | (if self.en_idchg1_prochot { 0x04 } else { 0 })
            | (if self.en_inom_prochot { 0x02 } else { 0 })
            | (if self.en_icrit_prochot { 0x01 } else { 0 });

        let lsb = (if self.en_comp_prochot { 0x80 } else { 0 })
            | (if self.en_vindpm_prochot { 0x40 } else { 0 })
            | (if self.en_vap_prochot { 0x20 } else { 0 })
            | (if self.en_otg_uvp_prochot { 0x10 } else { 0 })
            | (if self.en_otg_ovp_prochot { 0x08 } else { 0 })
            | (if self.en_force_converter_off_prochot {
                0x04
            } else {
                0
            })
            | (if self.en_vsys_uvp_prochot { 0x02 } else { 0 })
            | (if self.en_sysovp_prochot { 0x01 } else { 0 });

        (lsb, msb)
    }
}

/// Represents the ChargeOption2 register (0x40/0x41).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption2::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeOption2 {
    /// Enable PROCHOT on ACOC (EN_ACOC_PROCHOT)
    pub en_acoc_prochot: bool,
    /// Enable PROCHOT on BATOC (EN_BATOC_PROCHOT)
    pub en_batoc_prochot: bool,
    /// Enable PROCHOT on ACOV (EN_ACOV_PROCHOT)
    pub en_acov_prochot: bool,
    /// Enable PROCHOT on ICO (EN_ICO_PROCHOT)
    pub en_ico_prochot: bool,
    /// Enable PROCHOT on VINDPM (EN_VINDPM_PROCHOT_2)
    pub en_vindpm_prochot_2: bool,
    /// Enable PROCHOT on CMPOUT (EN_COMP_PROCHOT_2)
    pub en_comp_prochot_2: bool,
    /// Enable PROCHOT on ICRIT (EN_ICRIT_PROCHOT_2)
    pub en_icrit_prochot_2: bool,
    /// Enable PROCHOT on INOM (EN_INOM_PROCHOT_2)
    pub en_inom_prochot_2: bool,
    /// Enable PROCHOT on IDCHG1 (EN_IDCHG1_PROCHOT_2)
    pub en_idchg1_prochot_2: bool,
    /// Enable PROCHOT on VSYS (EN_VSYS_PROCHOT_2)
    pub en_vsys_prochot_2: bool,
    /// Enable PROCHOT on Battery Removal (EN_BAT_REMOVAL_PROCHOT_2)
    pub en_bat_removal_prochot_2: bool,
    /// Enable PROCHOT on Adapter Removal (EN_ADPT_REMOVAL_PROCHOT_2)
    pub en_adpt_removal_prochot_2: bool,
    /// Enable PROCHOT on IDCHG2 (EN_IDCHG2_PROCHOT_2)
    pub en_idchg2_prochot_2: bool,
    /// Enable PROCHOT on PTM (EN_PTM_PROCHOT_2)
    pub en_ptm_prochot_2: bool,
    /// Enable PROCHOT on OTG OVP (EN_OTG_OVP_PROCHOT_2)
    pub en_otg_ovp_prochot_2: bool,
    /// Enable PROCHOT on OTG UVP (EN_OTG_UVP_PROCHOT_2)
    pub en_otg_uvp_prochot_2: bool,
}

#[cfg(feature = "binrw")]
impl ChargeOption2 {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeOption2::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeOption2 {
    /// Creates a new ChargeOption2 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_acoc_prochot: (msb & 0x80) != 0,
            en_batoc_prochot: (msb & 0x40) != 0,
            en_acov_prochot: (msb & 0x20) != 0,
            en_ico_prochot: (msb & 0x10) != 0,
            en_vindpm_prochot_2: (msb & 0x08) != 0,
            en_comp_prochot_2: (msb & 0x04) != 0,
            en_icrit_prochot_2: (msb & 0x02) != 0,
            en_inom_prochot_2: (msb & 0x01) != 0,
            en_idchg1_prochot_2: (lsb & 0x80) != 0,
            en_vsys_prochot_2: (lsb & 0x40) != 0,
            en_bat_removal_prochot_2: (lsb & 0x20) != 0,
            en_adpt_removal_prochot_2: (lsb & 0x10) != 0,
            en_idchg2_prochot_2: (lsb & 0x08) != 0,
            en_ptm_prochot_2: (lsb & 0x04) != 0,
            en_otg_ovp_prochot_2: (lsb & 0x02) != 0,
            en_otg_uvp_prochot_2: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption2 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let msb = (if self.en_acoc_prochot { 0x80 } else { 0 })
            | (if self.en_batoc_prochot { 0x40 } else { 0 })
            | (if self.en_acov_prochot { 0x20 } else { 0 })
            | (if self.en_ico_prochot { 0x10 } else { 0 })
            | (if self.en_vindpm_prochot_2 { 0x08 } else { 0 })
            | (if self.en_comp_prochot_2 { 0x04 } else { 0 })
            | (if self.en_icrit_prochot_2 { 0x02 } else { 0 })
            | (if self.en_inom_prochot_2 { 0x01 } else { 0 });

        let lsb = (if self.en_idchg1_prochot_2 { 0x80 } else { 0 })
            | (if self.en_vsys_prochot_2 { 0x40 } else { 0 })
            | (if self.en_bat_removal_prochot_2 {
                0x20
            } else {
                0
            })
            | (if self.en_adpt_removal_prochot_2 {
                0x10
            } else {
                0
            })
            | (if self.en_idchg2_prochot_2 { 0x08 } else { 0 })
            | (if self.en_ptm_prochot_2 { 0x04 } else { 0 })
            | (if self.en_otg_ovp_prochot_2 { 0x02 } else { 0 })
            | (if self.en_otg_uvp_prochot_2 { 0x01 } else { 0 });

        (lsb, msb)
    }
}

/// Represents the ChargeOption3 register (0x42/0x43).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption3::from_bin_bytes))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_bin_bytes()))]
pub struct ChargeOption3 {
    /// Enable PROCHOT on ACOV (EN_ACOV_PROCHOT_3)
    pub en_acov_prochot_3: bool,
    /// Enable PROCHOT on BATOC (EN_BATOC_PROCHOT_3)
    pub en_batoc_prochot_3: bool,
    /// Enable PROCHOT on ACOC (EN_ACOC_PROCHOT_3)
    pub en_acoc_prochot_3: bool,
    /// Enable PROCHOT on ICO (EN_ICO_PROCHOT_3)
    pub en_ico_prochot_3: bool,
    /// Enable PROCHOT on VINDPM (EN_VINDPM_PROCHOT_3)
    pub en_vindpm_prochot_3: bool,
    /// Enable PROCHOT on CMPOUT (EN_COMP_PROCHOT_3)
    pub en_comp_prochot_3: bool,
    /// Enable PROCHOT on ICRIT (EN_ICRIT_PROCHOT_3)
    pub en_icrit_prochot_3: bool,
    /// Enable PROCHOT on INOM (EN_INOM_PROCHOT_3)
    pub en_inom_prochot_3: bool,
    /// Enable PROCHOT on IDCHG1 (EN_IDCHG1_PROCHOT_3)
    pub en_idchg1_prochot_3: bool,
    /// Enable PROCHOT on VSYS (EN_VSYS_PROCHOT_3)
    pub en_vsys_prochot_3: bool,
    /// Enable PROCHOT on Battery Removal (EN_BAT_REMOVAL_PROCHOT_3)
    pub en_bat_removal_prochot_3: bool,
    /// Enable PROCHOT on Adapter Removal (EN_ADPT_REMOVAL_PROCHOT_3)
    pub en_adpt_removal_prochot_3: bool,
    /// Enable PROCHOT on IDCHG2 (EN_IDCHG2_PROCHOT_3)
    pub en_idchg2_prochot_3: bool,
    /// Enable PROCHOT on PTM (EN_PTM_PROCHOT_3)
    pub en_ptm_prochot_3: bool,
    /// Enable PROCHOT on OTG OVP (EN_OTG_OVP_PROCHOT_3)
    pub en_otg_ovp_prochot_3: bool,
    /// Enable PROCHOT on OTG UVP (EN_OTG_UVP_PROCHOT_3)
    pub en_otg_uvp_prochot_3: bool,
}

#[cfg(feature = "binrw")]
impl ChargeOption3 {
    fn from_bin_bytes(bytes: (u8, u8)) -> Self {
        ChargeOption3::from_register_value(bytes.0, bytes.1)
    }

    fn to_bin_bytes(&self) -> (u8, u8) {
        self.to_msb_lsb_bytes()
    }
}

impl ChargeOption3 {
    /// Creates a new ChargeOption3 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_acov_prochot_3: (msb & 0x80) != 0,
            en_batoc_prochot_3: (msb & 0x40) != 0,
            en_acoc_prochot_3: (msb & 0x20) != 0,
            en_ico_prochot_3: (msb & 0x10) != 0,
            en_vindpm_prochot_3: (msb & 0x08) != 0,
            en_comp_prochot_3: (msb & 0x04) != 0,
            en_icrit_prochot_3: (msb & 0x02) != 0,
            en_inom_prochot_3: (msb & 0x01) != 0,
            en_idchg1_prochot_3: (lsb & 0x80) != 0,
            en_vsys_prochot_3: (lsb & 0x40) != 0,
            en_bat_removal_prochot_3: (lsb & 0x20) != 0,
            en_adpt_removal_prochot_3: (lsb & 0x10) != 0,
            en_idchg2_prochot_3: (lsb & 0x08) != 0,
            en_ptm_prochot_3: (lsb & 0x04) != 0,
            en_otg_ovp_prochot_3: (lsb & 0x02) != 0,
            en_otg_uvp_prochot_3: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption3 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let msb = (if self.en_acov_prochot_3 { 0x80 } else { 0 })
            | (if self.en_batoc_prochot_3 { 0x40 } else { 0 })
            | (if self.en_acoc_prochot_3 { 0x20 } else { 0 })
            | (if self.en_ico_prochot_3 { 0x10 } else { 0 })
            | (if self.en_vindpm_prochot_3 { 0x08 } else { 0 })
            | (if self.en_comp_prochot_3 { 0x04 } else { 0 })
            | (if self.en_icrit_prochot_3 { 0x02 } else { 0 })
            | (if self.en_inom_prochot_3 { 0x01 } else { 0 });

        let lsb = (if self.en_idchg1_prochot_3 { 0x80 } else { 0 })
            | (if self.en_vsys_prochot_3 { 0x40 } else { 0 })
            | (if self.en_bat_removal_prochot_3 {
                0x20
            } else {
                0
            })
            | (if self.en_adpt_removal_prochot_3 {
                0x10
            } else {
                0
            })
            | (if self.en_idchg2_prochot_3 { 0x08 } else { 0 })
            | (if self.en_ptm_prochot_3 { 0x04 } else { 0 })
            | (if self.en_otg_ovp_prochot_3 { 0x02 } else { 0 })
            | (if self.en_otg_uvp_prochot_3 { 0x01 } else { 0 });

        (lsb, msb)
    }
}
