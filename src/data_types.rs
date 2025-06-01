#![allow(clippy::approx_constant)]

#[cfg(feature = "defmt")]
use defmt::Format;

use crate::registers::{
    AdcOptionFlags, AdcOptionMsbFlags, ChargeOption0Flags, ChargeOption0MsbFlags,
    ChargeOption1Flags, ChargeOption1MsbFlags, ChargeOption2Flags, ChargeOption2MsbFlags,
    ChargeOption3Flags, ChargeOption3MsbFlags, ChargerStatusFaultFlags, ChargerStatusFlags,
    ProchotOption0Flags, ProchotOption0MsbFlags, ProchotOption1Flags, ProchotOption1MsbFlags,
    ProchotStatusFlags, ProchotStatusMsbFlags,
};
#[cfg(feature = "binrw")]
use binrw::{BinRead, BinWrite};

/// Enum to represent the sense resistor value.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(repr = u8))] // Assuming u8 representation for the enum
#[cfg_attr(feature = "binrw", bw(repr = u8))] // Assuming u8 representation for the enum
#[repr(u8)]
pub enum SenseResistorValue {
    R5mOhm,  // 5mΩ sense resistor
    R10mOhm, // 10mΩ sense resistor
}

impl Default for SenseResistorValue {
    fn default() -> Self {
        SenseResistorValue::R5mOhm // Default to 5mOhm
    }
}

/// Represents the status of the BQ25730 charger.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargerStatus::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargerStatus {
    pub status_flags: ChargerStatusFlags,
    pub fault_flags: ChargerStatusFaultFlags,
}

impl Default for ChargerStatus {
    fn default() -> Self {
        Self {
            status_flags: ChargerStatusFlags::empty(),
            fault_flags: ChargerStatusFaultFlags::empty(),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargerStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargerStatus {{ status_flags: {}, fault_flags: {} }}",
            self.status_flags,
            self.fault_flags
        );
    }
}

impl ChargerStatus {
    #[cfg(feature = "binrw")]
    fn from_u16(value: u16) -> Self {
        Self {
            status_flags: ChargerStatusFlags::from_bits_truncate((value >> 8) as u8),
            fault_flags: ChargerStatusFaultFlags::from_bits_truncate(value as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        ((self.status_flags.bits() as u16) << 8) | (self.fault_flags.bits() as u16)
    }
}

impl ChargerStatus {
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargerStatusFlags {
    fn format(&self, fmt: defmt::Formatter) {
        if self.is_empty() {
            defmt::write!(fmt, "(empty)");
            return;
        }
        let mut first = true;
        if self.contains(ChargerStatusFlags::STAT_AC) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "STAT_AC");
            first = false;
        }
        if self.contains(ChargerStatusFlags::ICO_DONE) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "ICO_DONE");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_VAP) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_VAP");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_VINDPM) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_VINDPM");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_IIN_DPM) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_IIN_DPM");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_FCHRG) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_FCHRG");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_PCHRG) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_PCHRG");
            first = false;
        }
        if self.contains(ChargerStatusFlags::IN_OTG) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "IN_OTG");
            // first = false; // Not needed for the last one
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargerStatusFaultFlags {
    fn format(&self, fmt: defmt::Formatter) {
        if self.is_empty() {
            defmt::write!(fmt, "(empty)");
            return;
        }
        let mut first = true;
        if self.contains(ChargerStatusFaultFlags::FAULT_ACOV) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_ACOV");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_BATOC) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_BATOC");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_ACOC) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_ACOC");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_SYSOVP) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_SYSOVP");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_VSYS_UVP) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_VSYS_UVP");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_FORCE_CONVERTER_OFF) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_FORCE_CONVERTER_OFF");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_OTG_OVP) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_OTG_OVP");
            first = false;
        }
        if self.contains(ChargerStatusFaultFlags::FAULT_OTG_UVP) {
            if !first {
                defmt::write!(fmt, " | ");
            }
            defmt::write!(fmt, "FAULT_OTG_UVP");
            // first = false; // Not needed for the last one
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotStatusMsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotStatusFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

/// Represents the PROCHOT status of the BQ25730.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ProchotStatus::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ProchotStatus {
    pub msb_flags: ProchotStatusMsbFlags,
    pub lsb_flags: ProchotStatusFlags,
    pub prochot_width: u8,
}

impl Default for ProchotStatus {
    fn default() -> Self {
        Self {
            msb_flags: ProchotStatusMsbFlags::empty(),
            lsb_flags: ProchotStatusFlags::empty(),
            prochot_width: 0,
        }
    }
}

impl ProchotStatus {
    #[cfg(feature = "binrw")]
    fn from_u16(value: u16) -> Self {
        Self {
            // Mask out bits 13:12 (prochot_width) from the MSB when extracting msb_flags
            msb_flags: ProchotStatusMsbFlags::from_bits_truncate(((value >> 8) & !0x30) as u8),
            lsb_flags: ProchotStatusFlags::from_bits_truncate((value & 0xFF) as u8),
            prochot_width: ((value >> 12) & 0x03) as u8, // Bits 13:12
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value |= ((self.prochot_width & 0x03) as u16) << 12;
        value
    }
}

impl ProchotStatus {
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotStatus {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ProchotStatus {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b}, prochot_width: {} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits(),
            self.prochot_width
        );
    }
}

/// Represents the Charge Current setting in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct ChargeCurrent {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for ChargeCurrent {
    fn default() -> Self {
        Self {
            milliamps: 0,
            rsns_bat: SenseResistorValue::default(),
        }
    }
}

impl ChargeCurrent {
    /// Creates a new ChargeCurrent from a raw 7-bit register value and RSNS setting.
    /// The 7-bit value represents the charge current setting.
    pub fn from_raw(raw_7bit: u8, rsns_bat: SenseResistorValue) -> Self {
        let lsb_ma = match rsns_bat {
            SenseResistorValue::R5mOhm => 128, // 128mA/LSB for 5mΩ
            SenseResistorValue::R10mOhm => 64, // 64mA/LSB for 10mΩ
        };
        // Raw value is 7-bit (0-127)
        Self {
            milliamps: (raw_7bit as u16) * lsb_ma,
            rsns_bat,
        }
    }

    /// Converts the ChargeCurrent to a raw 7-bit register value.
    pub fn to_raw(&self) -> u8 {
        let lsb_ma = match self.rsns_bat {
            SenseResistorValue::R5mOhm => 128,
            SenseResistorValue::R10mOhm => 64,
        };
        // Ensure the result fits in 7 bits (0-127)
        let raw_value = self.milliamps / lsb_ma;
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }

    /// Converts the ChargeCurrent to milliamps.
    pub fn to_milliamps(&self) -> u16 {
        self.milliamps
    }
}

/// Represents the Charge Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeVoltage::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeVoltage(pub u16);

impl ChargeVoltage {
    /// Creates a new ChargeVoltage from a 16-bit raw register value.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x05): D11-D5 in bits 6:0
    /// LSB (0x04): D4-D0 in bits 7:3
    pub fn from_u16(value: u16) -> Self {
        ChargeVoltage(value & 0x7FF8)
    }

    /// Converts the ChargeVoltage to a 16-bit raw register value.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x05): D11-D5 in bits 6:0
    /// LSB (0x04): D4-D0 in bits 4:0
    pub fn to_u16(&self) -> u16 {
        // raw_value is a 12-bit value (D11-D0)
        // msb (0x05) bits 6:0 should be D11-D5
        // lsb (0x04) bits 7:3 should be D4-D0
        self.0 & 0x7FF8
    }

    /// Converts the ChargeVoltage to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0;
        let msb = ((raw_value & 0x7F) >> 8) as u8; // D11-D5
        let lsb = (raw_value & 0x07) as u8; // D4-D0, shifted to bits 7:3 for 04h register
        (lsb, msb) // Returns (04h_value, 05h_value)
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
#[cfg_attr(feature = "binrw", br(map = OtgVoltage::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct OtgVoltage(pub u16);

impl OtgVoltage {
    /// LSB value for OTG Voltage in mV.
    pub const LSB_MV: u16 = 8; // 8mV/LSB based on datasheet 7.5

    /// Creates a new OtgVoltage from a 16-bit raw register value.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x07): D11-D5 in bits 6:0
    /// LSB (0x06): D4-D0 in bits 7:3
    pub fn from_u16(value: u16) -> Self {
        let msb = (value >> 8) as u8;
        let lsb = value as u8;
        // D11-D5 are in msb bits 6:0
        // D4-D0 are in lsb bits 7:3
        let d11_d5 = (msb & 0x7F) as u16; // Extract bits 6:0 from msb
        let d4_d0 = ((lsb >> 3) & 0x1F) as u16; // Extract bits 7:3 from lsb

        // Combine them to form a 12-bit raw_value (D11 D10 D9 D8 D7 D6 D5 D4 D3 D2 D1 D0)
        let raw_value = (d11_d5 << 5) | d4_d0;
        OtgVoltage(raw_value * Self::LSB_MV)
    }

    /// Converts the OtgVoltage to a 16-bit raw register value.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x07): D11-D5 in bits 6:0
    /// LSB (0x06): D4-D0 in bits 7:3
    pub fn to_u16(&self) -> u16 {
        let raw_value = self.0 / Self::LSB_MV;
        // raw_value is a 12-bit value (D11-D0)
        // msb (0x07) bits 6:0 should be D11-D5
        // lsb (0x06) bits 7:3 should be D4-D0
        let msb = ((raw_value >> 5) & 0x7F) as u8; // D11-D5
        let lsb = ((raw_value & 0x1F) << 3) as u8; // D4-D0 shifted to bits 7:3
        (lsb as u16) | ((msb as u16) << 8)
    }

    /// Converts the OtgVoltage to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MV;
        let msb = ((raw_value >> 5) & 0x7F) as u8; // D11-D5
        let lsb = ((raw_value & 0x1F) << 3) as u8; // D4-D0 shifted to bits 7:3
        (lsb, msb)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct OtgCurrent {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for OtgCurrent {
    fn default() -> Self {
        Self {
            milliamps: 0,
            rsns_bat: SenseResistorValue::default(),
        }
    }
}

impl OtgCurrent {
    /// Creates a new OtgCurrent from a raw 7-bit register value and RSNS setting.
    pub fn from_raw(raw_7bit: u8, rsns_bat: SenseResistorValue) -> Self {
        let lsb_ma = match rsns_bat {
            SenseResistorValue::R5mOhm => 100, // 100mA/LSB for 5mΩ
            SenseResistorValue::R10mOhm => 50, // 50mA/LSB for 10mΩ
        };
        // Raw value is 7-bit (0-127)
        Self {
            milliamps: (raw_7bit as u16) * lsb_ma,
            rsns_bat,
        }
    }

    /// Converts the OtgCurrent to a raw 7-bit register value.
    pub fn to_raw(&self) -> u8 {
        let lsb_ma = match self.rsns_bat {
            SenseResistorValue::R5mOhm => 100,
            SenseResistorValue::R10mOhm => 50,
        };
        // Ensure the result fits in 7 bits (0-127)
        let raw_value = self.milliamps / lsb_ma;
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the Input Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = InputVoltage::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct InputVoltage(pub u16);

impl InputVoltage {
    /// LSB value for Input Voltage in mV.
    pub const LSB_MV: u16 = 64;
    /// Offset value for Input Voltage in mV.
    pub const OFFSET_MV: u16 = 3200; // 3200mV offset (from 3.2V)

    /// Creates a new InputVoltage from a 16-bit raw register value.
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn from_u16(value: u16) -> Self {
        let msb = (value >> 8) as u8;
        let lsb = value as u8;
        // D8 is in bit 5 of MSB (0x0B)
        let raw_value = ((((msb >> 5) & 0x01) as u16) << 8) | (lsb as u16); // D8-D0 (D8 is bit 5 of MSB)
        InputVoltage(raw_value * Self::LSB_MV + Self::OFFSET_MV)
    }

    /// Converts the InputVoltage to a 16-bit raw register value.
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn to_u16(&self) -> u16 {
        // Ensure the value is not less than the offset to prevent underflow
        let raw_value_9bit = if self.0 >= Self::OFFSET_MV {
            (self.0 - Self::OFFSET_MV) / Self::LSB_MV
        } else {
            0 // Clamp to the minimum register value (corresponding to OFFSET_MV)
        };
        // raw_value_9bit is a 9-bit value (D8-D0)
        let d8 = (raw_value_9bit >> 8) & 0x01; // D8 is bit 8
        let d7_d0 = raw_value_9bit & 0xFF; // D7-D0 are bits 7:0

        // MSB (0x0B): D8 in bit 5
        let msb = (d8 as u8) << 5;

        // LSB (0x0A): D7-D0 in bits 7:0
        let lsb = d7_d0 as u8;

        (lsb as u16) | ((msb as u16) << 8)
    }

    /// Converts the InputVoltage to raw MSB and LSB register values.
    /// Since InputVoltage is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let _raw_value = (self.0 - Self::OFFSET_MV) / Self::LSB_MV;
        // Ensure the value is not less than the offset to prevent underflow
        let raw_value = if self.0 >= Self::OFFSET_MV {
            (self.0 - Self::OFFSET_MV) / Self::LSB_MV
        } else {
            0 // Clamp to the minimum register value (corresponding to OFFSET_MV)
        };
        let msb = (((raw_value >> 8) & 0x01) << 5) as u8; // D8 in bit 5 of MSB (0x0B)
        let lsb = (raw_value & 0xFF) as u8; // D7-D0 in bits 7:0 of LSB (0x0A)
        (lsb, msb) // LSB, MSB
    }

    /// Converts the InputVoltage to a raw 8-bit register value.
    /// This function is likely not used for InputVoltage as it's a 2-byte register.
    /// However, if it were to return the LSB part, it would be:
    pub fn to_register_value(&self) -> u8 {
        let _raw_value = (self.0 - Self::OFFSET_MV) / Self::LSB_MV;
        // Ensure the value is not less than the offset to prevent underflow
        let raw_value = if self.0 >= Self::OFFSET_MV {
            (self.0 - Self::OFFSET_MV) / Self::LSB_MV
        } else {
            0 // Clamp to the minimum register value (corresponding to OFFSET_MV)
        };
        (raw_value & 0xFF) as u8 // Return LSB part
    }
}

/// Represents the Minimum System Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = VsysMin::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct VsysMin(pub u16);

impl VsysMin {
    /// LSB value for Minimum System Voltage in mV.
    pub const LSB_MV: u16 = 100;

    /// Creates a new VsysMin from a 16-bit raw register value.
    pub fn from_u16(value: u16) -> Self {
        let msb = (value >> 8) as u8;
        VsysMin((msb as u16) * Self::LSB_MV)
    }

    /// Converts the VsysMin to a 16-bit raw register value.
    pub fn to_u16(&self) -> u16 {
        let msb = (self.0 / Self::LSB_MV) as u8;
        (msb as u16) << 8
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
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct IinHost {
    pub milliamps: u16,
    pub rsns_ac: SenseResistorValue,
}

impl Default for IinHost {
    fn default() -> Self {
        // Default to minimum possible value based on default Rsns
        let rsns_ac = SenseResistorValue::default();
        let (_lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100),
            SenseResistorValue::R10mOhm => (50, 50),
        };
        Self {
            milliamps: offset_ma,
            rsns_ac,
        }
    }
}

impl IinHost {
    /// Creates a new IinHost from a raw 7-bit register value and RSNS setting.
    pub fn from_raw(raw_7bit: u8, rsns_ac: SenseResistorValue) -> Self {
        let (lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100), // LSB 100mA, Offset 100mA for 5mΩ
            SenseResistorValue::R10mOhm => (50, 50),  // LSB 50mA, Offset 50mA for 10mΩ
        };
        // Raw value is 7-bit (0-127)
        Self {
            milliamps: (raw_7bit as u16) * lsb_ma + offset_ma,
            rsns_ac,
        }
    }

    /// Converts the IinHost to a raw 7-bit register value.
    pub fn to_raw(&self) -> u8 {
        let (lsb_ma, offset_ma) = match self.rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100),
            SenseResistorValue::R10mOhm => (50, 50),
        };
        let raw_value = if self.milliamps >= offset_ma {
            (self.milliamps - offset_ma) / lsb_ma
        } else {
            0 // Clamp to 0 if milliamps is less than offset
        };
        // Ensure the result fits in 7 bits (0-127)
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the Input Current Limit in Use (IIN_DPM) in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct IinDpm {
    pub milliamps: u16,
    pub rsns_ac: SenseResistorValue,
}

impl Default for IinDpm {
    fn default() -> Self {
        let rsns_ac = SenseResistorValue::default();
        let (_lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100),
            SenseResistorValue::R10mOhm => (50, 50),
        };
        Self {
            milliamps: offset_ma,
            rsns_ac,
        }
    }
}

impl IinDpm {
    /// Creates a new IinDpm from a raw 7-bit register value and RSNS setting.
    pub fn from_raw(raw_7bit: u8, rsns_ac: SenseResistorValue) -> Self {
        let (lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100), // LSB 100mA, Offset 100mA for 5mΩ
            SenseResistorValue::R10mOhm => (50, 50),  // LSB 50mA, Offset 50mA for 10mΩ
        };
        // Raw value is 7-bit (0-127)
        Self {
            milliamps: (raw_7bit as u16) * lsb_ma + offset_ma,
            rsns_ac,
        }
    }

    /// Converts the IinDpm to a raw 7-bit register value.
    pub fn to_raw(&self) -> u8 {
        let (lsb_ma, offset_ma) = match self.rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100),
            SenseResistorValue::R10mOhm => (50, 50),
        };
        let raw_value = if self.milliamps >= offset_ma {
            (self.milliamps - offset_ma) / lsb_ma
        } else {
            0 // Clamp to 0 if milliamps is less than offset
        };
        // Ensure the result fits in 7 bits (0-127)
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the ADC measurements.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))] // Specify Little Endian for reading
#[cfg_attr(feature = "binrw", bw(little))] // Specify Little Endian for writing
pub struct AdcMeasurements {
    pub vbat: AdcVbat,
    pub vsys: AdcVsys,
    pub ichg: AdcIchg,
    pub idchg: AdcIdchg,
    pub iin: AdcIin,
    pub psys: AdcPsys,
    pub vbus: AdcVbus,
    pub cmpin: AdcCmpin,
}

impl Default for AdcMeasurements {
    fn default() -> Self {
        Self {
            vbat: AdcVbat::default(),
            vsys: AdcVsys::default(),
            ichg: AdcIchg::default(),
            idchg: AdcIdchg::default(),
            iin: AdcIin::default(),
            psys: AdcPsys::default(),
            vbus: AdcVbus::default(),
            cmpin: AdcCmpin::default(),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcMeasurements {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "AdcMeasurements {{ vbat: {}, vsys: {}, ichg: {}, idchg: {}, iin: {}, psys: {}, vbus: {}, cmpin: {} }}",
            self.vbat,
            self.vsys,
            self.ichg,
            self.idchg,
            self.iin,
            self.psys,
            self.vbus,
            self.cmpin
        );
    }
}

/// Represents the ADCCMPIN register value.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcCmpin::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct AdcCmpin(pub u16);

impl Default for AdcCmpin {
    fn default() -> Self {
        Self(0)
    }
}

impl AdcCmpin {
    /// LSB value for ADCCMPIN in mV (with ADC_FULLSCALE=1b).
    pub const LSB_MV: u16 = 12;

    /// Creates a new AdcCmpin from a 16-bit raw register value.
    pub fn from_u16(value: u16) -> Self {
        // Extract the 8-bit raw value from the MSB before scaling
        AdcCmpin((value >> 8) * Self::LSB_MV)
    }

    /// Creates a new AdcCmpin from an 8-bit raw register value.
    /// Converts the 8-bit raw value to a scaled value based on LSB.
    pub fn from_u8(raw_value: u8) -> Self {
        // ADCCMPIN is an 8-bit value (0-255)
        AdcCmpin((raw_value as u16) * Self::LSB_MV) // Scale by 12mV LSB
    }

    /// Converts the AdcCmpin to a 16-bit raw register value.
    pub fn to_u16(&self) -> u16 {
        (self.0 / Self::LSB_MV) << 8 // Convert mV back to raw 8-bit value in MSB
    }

    /// Converts the AdcCmpin to raw MSB and LSB register values.
    /// Since ADCCMPIN is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, (self.0 / Self::LSB_MV) as u8) // Convert mV back to raw 8-bit value
    }
}

/// Represents the ADCICHG register value in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct AdcIchg {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for AdcIchg {
    fn default() -> Self {
        Self {
            milliamps: 0,
            rsns_bat: SenseResistorValue::default(),
        }
    }
}

impl AdcIchg {
    /// Creates a new AdcIchg from a raw 7-bit ADC value and RSNS setting.
    pub fn from_raw(raw_7bit_adc: u8, rsns_bat: SenseResistorValue) -> Self {
        let lsb_ma = match rsns_bat {
            SenseResistorValue::R5mOhm => 128, // 128mA/LSB for 5mΩ
            SenseResistorValue::R10mOhm => 64, // 64mA/LSB for 10mΩ
        };
        // ADCICHG is a 7-bit value (0-127)
        Self {
            milliamps: (raw_7bit_adc as u16) * lsb_ma,
            rsns_bat,
        }
    }

    /// Converts the AdcIchg to a raw 7-bit ADC value.
    pub fn to_raw(&self) -> u8 {
        let lsb_ma = match self.rsns_bat {
            SenseResistorValue::R5mOhm => 128,
            SenseResistorValue::R10mOhm => 64,
        };
        // Ensure the result fits in 7 bits (0-127)
        let raw_value = self.milliamps / lsb_ma;
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the ADCIDCHG register value in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcIdchg {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for AdcIdchg {
    fn default() -> Self {
        Self {
            milliamps: 0,
            rsns_bat: SenseResistorValue::default(),
        }
    }
}

impl AdcIdchg {
    /// Creates a new AdcIdchg from a raw 7-bit ADC value and RSNS setting.
    pub fn from_raw(raw_7bit_adc: u8, rsns_bat: SenseResistorValue) -> Self {
        let lsb_ma = match rsns_bat {
            SenseResistorValue::R5mOhm => 512,  // 512mA/LSB for 5mΩ
            SenseResistorValue::R10mOhm => 256, // 256mA/LSB for 10mΩ
        };
        // ADCIDCHG is a 7-bit value (0-127)
        Self {
            milliamps: (raw_7bit_adc as u16) * lsb_ma,
            rsns_bat,
        }
    }

    /// Converts the AdcIdchg to a raw 7-bit ADC value.
    pub fn to_raw(&self) -> u8 {
        let lsb_ma = match self.rsns_bat {
            SenseResistorValue::R5mOhm => 512,
            SenseResistorValue::R10mOhm => 256,
        };
        // Ensure the result fits in 7 bits (0-127)
        let raw_value = self.milliamps / lsb_ma;
        if raw_value > 0x7F {
            0x7F // Clamp to max 7-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the ADCIIN register value in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))]
#[cfg_attr(feature = "binrw", bw(little))]
pub struct AdcIin {
    pub milliamps: u16,
    pub rsns_ac: SenseResistorValue,
}

impl Default for AdcIin {
    fn default() -> Self {
        Self {
            milliamps: 0,
            rsns_ac: SenseResistorValue::default(),
        }
    }
}

impl AdcIin {
    /// Creates a new AdcIin from an 8-bit raw ADC value and RSNS setting.
    pub fn from_raw(raw_8bit_adc: u8, rsns_ac: SenseResistorValue) -> Self {
        let lsb_ma = match rsns_ac {
            SenseResistorValue::R5mOhm => 100, // 100mA/LSB for 5mΩ
            SenseResistorValue::R10mOhm => 50, // 50mA/LSB for 10mΩ
        };
        // ADCIIN is an 8-bit value (0-255)
        Self {
            milliamps: (raw_8bit_adc as u16) * lsb_ma,
            rsns_ac,
        }
    }

    /// Converts the AdcIin to a raw 8-bit ADC value.
    pub fn to_raw(&self) -> u8 {
        let lsb_ma = match self.rsns_ac {
            SenseResistorValue::R5mOhm => 100,
            SenseResistorValue::R10mOhm => 50,
        };
        // Ensure the result fits in 8 bits (0-255)
        let raw_value = self.milliamps / lsb_ma;
        if raw_value > 0xFF {
            0xFF // Clamp to max 8-bit value
        } else {
            raw_value as u8
        }
    }
}

/// Represents the ADCPSYS register value (related to system power).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcPsys::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct AdcPsys(pub u16);

impl Default for AdcPsys {
    fn default() -> Self {
        Self(0)
    }
}

impl AdcPsys {
    /// LSB value for ADCPSYS (assuming 12mV/LSB when ADC_FULLSCALE=1b).
    /// This might represent a voltage proportional to power.
    pub const LSB_MV: u16 = 12;

    /// Creates a new AdcPsys from a 16-bit raw register value.
    pub fn from_u16(value: u16) -> Self {
        AdcPsys(value) // Assuming value is already scaled
    }

    /// Creates a new AdcPsys from an 8-bit raw register value.
    /// Converts the 8-bit raw value to a scaled value based on LSB.
    pub fn from_u8(raw_value: u8) -> Self {
        // ADCPSYS is an 8-bit value (0-255)
        AdcPsys((raw_value as u16) * Self::LSB_MV) // Scale by 12mV LSB
    }

    /// Converts the AdcPsys to a 16-bit raw register value.
    pub fn to_u16(&self) -> u16 {
        self.0 // Assuming self.0 is already scaled
    }

    /// Converts the AdcPsys to raw MSB and LSB register values.
    /// Since ADCPSYS is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, (self.0 / Self::LSB_MV) as u8) // Convert scaled value back to raw 8-bit value
    }
}

/// Represents the ADCVBUS register value in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcVbus::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct AdcVbus(pub u16);

impl Default for AdcVbus {
    fn default() -> Self {
        Self(0)
    }
}

impl AdcVbus {
    /// LSB value for ADCVBUS in mV.
    pub const LSB_MV: u16 = 96;

    /// Creates a new AdcVbus from a 16-bit raw register value.
    pub fn from_u16(value: u16) -> Self {
        let raw_value = (value >> 8) as u8;
        AdcVbus((raw_value as u16) * Self::LSB_MV)
    }

    /// Creates a new AdcVbus from an 8-bit raw register value.
    /// Converts the 8-bit raw value to mV based on LSB.
    pub fn from_u8(raw_value: u8) -> Self {
        // ADCVBUS is an 8-bit value (0-255)
        AdcVbus((raw_value as u16) * Self::LSB_MV)
    }

    /// Converts the AdcVbus to a 16-bit raw register value.
    pub fn to_u16(&self) -> u16 {
        let raw_value = self.0 / Self::LSB_MV;
        raw_value << 8
    }

    /// Converts the AdcVbus to raw MSB and LSB register values.
    /// Since ADCVBUS is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, self.to_u16() as u8)
    }
}

/// Represents the ADCVBAT register value in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AdcVbat(pub u16);

impl Default for AdcVbat {
    fn default() -> Self {
        Self(0)
    }
}

impl AdcVbat {
    /// LSB value for ADCVBAT in mV.
    pub const LSB_MV: u16 = 64;
    /// Offset value for ADCVBAT in mV.
    pub const OFFSET_MV: u16 = 0; // Assuming 0mV offset for ADC measurements

    /// Creates a new AdcVbat from a 16-bit raw register value.
    /// The 8-bit value is in the MSB (0x2D).
    pub fn from_u16(value: u16, offset_mv: u16) -> Self {
        let raw_value = (value >> 8) as u8;
        AdcVbat((raw_value as u16) * Self::LSB_MV + offset_mv)
    }

    /// Creates a new AdcVbat from raw LSB and MSB register values.
    pub fn from_register_value(_lsb: u8, msb: u8, offset_mv: u16) -> Self {
        AdcVbat((msb as u16) * Self::LSB_MV + offset_mv)
    }

    /// Converts the AdcVbat to a 16-bit raw register value.
    /// The 8-bit value is in the MSB (0x2D).
    pub fn to_u16(&self) -> u16 {
        let raw_value = (self.0 - self.0 % Self::LSB_MV) / Self::LSB_MV; // Convert mV back to raw 8-bit value
        raw_value << 8
    }

    /// Converts the AdcVbat to raw MSB and LSB register values.
    /// Since ADCVBAT is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, (self.0 / Self::LSB_MV) as u8) // Convert mV back to raw 8-bit value
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcVbat {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "AdcVbat({})", self.0);
    }
}

#[cfg(feature = "binrw")]
impl BinRead for AdcVbat {
    type Args<'a> = ();

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let value = u16::read_options(reader, endian, ())?;
        // Use the constant offset for conversion
        Ok(AdcVbat::from_u16(value, AdcVbat::OFFSET_MV))
    }
}

#[cfg(feature = "binrw")]
impl BinWrite for AdcVbat {
    type Args<'a> = ();

    fn write_options<W: binrw::io::Write + binrw::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let value = self.to_u16();
        value.write_options(writer, endian, ())?;
        Ok(())
    }
}

/// Represents the ADCVSYS register value in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AdcVsys(pub u16);

impl Default for AdcVsys {
    fn default() -> Self {
        Self(0)
    }
}

impl AdcVsys {
    /// LSB value for ADCVSYS in mV.
    pub const LSB_MV: u16 = 64;
    /// Offset value for ADCVSYS in mV.
    pub const OFFSET_MV: u16 = 0; // Assuming 0mV offset for ADC measurements

    /// Creates a new AdcVsys from a 16-bit raw register value.
    /// The 8-bit value is in the MSB (0x2D).
    pub fn from_u16(value: u16, offset_mv: u16) -> Self {
        let raw_value = (value >> 8) as u8;
        AdcVsys((raw_value as u16) * Self::LSB_MV + offset_mv)
    }

    /// Creates a new AdcVsys from raw LSB and MSB register values.
    pub fn from_register_value(_lsb: u8, msb: u8, offset_mv: u16) -> Self {
        AdcVsys((msb as u16) * Self::LSB_MV + offset_mv)
    }

    /// Converts the AdcVsys to a 16-bit raw register value.
    /// The 8-bit value is in the MSB (0x2D).
    pub fn to_u16(&self) -> u16 {
        let raw_value = (self.0 - self.0 % Self::LSB_MV) / Self::LSB_MV; // Convert mV back to raw 8-bit value
        raw_value << 8
    }

    /// Converts the AdcVsys to raw MSB and LSB register values.
    /// Since ADCVSYS is an 8-bit register, LSB will be 0.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, (self.0 / Self::LSB_MV) as u8) // Convert mV back to raw 8-bit value
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcVsys {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "AdcVsys({})", self.0);
    }
}

#[cfg(feature = "binrw")]
impl BinRead for AdcVsys {
    type Args<'a> = ();

    fn read_options<R: binrw::io::Read + binrw::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let value = u16::read_options(reader, endian, ())?;
        // Use the constant offset for conversion
        Ok(AdcVsys::from_u16(value, AdcVsys::OFFSET_MV))
    }
}

#[cfg(feature = "binrw")]
impl BinWrite for AdcVsys {
    type Args<'a> = ();

    fn write_options<W: binrw::io::Write + binrw::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let value = self.to_u16();
        value.write_options(writer, endian, ())?;
        Ok(())
    }
}

/// Represents the ADCOption register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = AdcOption::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct AdcOption {
    pub msb_flags: AdcOptionMsbFlags,
    pub lsb_flags: AdcOptionFlags,
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcOption {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "AdcOption {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl AdcOption {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: AdcOptionMsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: AdcOptionFlags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ChargeOption0 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption0::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeOption0 {
    pub msb_flags: ChargeOption0MsbFlags,
    pub lsb_flags: ChargeOption0Flags,
}

impl Default for ChargeOption0 {
    fn default() -> Self {
        // Datasheet reset E70Eh for REG0x01/00h
        // MSB (01h) = E7h (11100111b)
        // LSB (00h) = 0Eh (00001110b)
        Self {
            msb_flags: ChargeOption0MsbFlags::from_bits_truncate(0xE7),
            lsb_flags: ChargeOption0Flags::from_bits_truncate(0x0E),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption0 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargeOption0 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ChargeOption0 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ChargeOption0MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ChargeOption0Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ChargeOption1 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption1::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeOption1 {
    pub msb_flags: ChargeOption1MsbFlags,
    pub lsb_flags: ChargeOption1Flags,
}

impl Default for ChargeOption1 {
    fn default() -> Self {
        // Datasheet reset 3300h for REG0x31/30h (Figure 8-26)
        // MSB (31h) = 33h (00110011b)
        // LSB (30h) = 00h (00000000b)
        Self {
            msb_flags: ChargeOption1MsbFlags::from_bits_truncate(0x33),
            lsb_flags: ChargeOption1Flags::from_bits_truncate(0x00),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption1 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargeOption1 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ChargeOption1 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ChargeOption1MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ChargeOption1Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ChargeOption2 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption2::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeOption2 {
    pub msb_flags: ChargeOption2MsbFlags,
    pub lsb_flags: ChargeOption2Flags,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption2 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargeOption2 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ChargeOption2 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ChargeOption2MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ChargeOption2Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ChargeOption3 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption3::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeOption3 {
    pub msb_flags: ChargeOption3MsbFlags,
    pub lsb_flags: ChargeOption3Flags,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption3 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargeOption3 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ChargeOption3 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ChargeOption3MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ChargeOption3Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ProchotOption0 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ProchotOption0::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ProchotOption0 {
    pub msb_flags: ProchotOption0MsbFlags,
    pub lsb_flags: ProchotOption0Flags,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption0 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ProchotOption0 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ProchotOption0 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ProchotOption0MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ProchotOption0Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Represents the ProchotOption1 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ProchotOption1::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ProchotOption1 {
    pub msb_flags: ProchotOption1MsbFlags,
    pub lsb_flags: ProchotOption1Flags,
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption1 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ProchotOption1 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ProchotOption1 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ProchotOption1MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ProchotOption1Flags::from_bits_truncate((value & 0xFF) as u8),
        }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value = 0;
        value |= (self.msb_flags.bits() as u16) << 8;
        value |= self.lsb_flags.bits() as u16;
        value
    }

    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.to_u16();
        (raw_value as u8, (raw_value >> 8) as u8)
    }
}

/// Configuration for the BQ25730 charger.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub rsns_bat: SenseResistorValue,
    pub rsns_ac: SenseResistorValue,
    pub charge_option0: ChargeOption0,
    pub charge_option1: ChargeOption1,
    /// Raw register value for ChargeCurrent (REG0x03/02h).
    /// LSB (02h): bits 7:6 are D1:D0. MSB (03h): bits 4:0 are D6:D2.
    pub charge_current: u16,
    /// Raw register value for ChargeVoltage (REG0x05/04h).
    /// LSB (04h): bits 7:3 are D4:D0. MSB (05h): bits 6:0 are D11:D5.
    pub charge_voltage: u16,
    /// Raw register value for InputVoltage (VINDPM) (REG0x0B/0Ah).
    /// LSB (0Ah): bits 7:0 are D7:D0. MSB (0Bh): bit 5 is D8.
    pub input_voltage: u16,
    /// Raw register value for VSYS_MIN (REG0x0D/0Ch).
    /// MSB (0Dh): bits 7:0 are D7:D0. LSB (0Ch) is reserved (should be 0x00).
    pub vsys_min: u16,
    /// Raw register value for IIN_HOST (REG0x0F/0Eh).
    /// MSB (0Fh): bits 6:0 are D6:D0. LSB (0Eh) is reserved (should be 0x00).
    pub iin_host: u16,
    // TODO: Add other configurable registers as needed
}

impl Config {
    /// Creates a new Config with datasheet-recommended default values
    /// for a given cell count, battery sense resistor, and AC path sense resistor.
    pub fn new(cell_count: u8, rsns_bat: SenseResistorValue, rsns_ac: SenseResistorValue) -> Self {
        let mut co1_msb = ChargeOption1MsbFlags::from_bits_truncate(0x33); // Start with datasheet reset for MSB of ChargeOption1 (0x31h)
        // Default for 0x31h is 0x33. Bit 2 (RSNS_RSR) = 1 (5mOhm), Bit 3 (RSNS_RAC) = 1 (5mOhm).
        // If resistor is 10mOhm, clear the bit. If 5mOhm, set the bit.

        if rsns_bat == SenseResistorValue::R5mOhm {
            co1_msb.insert(ChargeOption1MsbFlags::RSNS_RSR);
        } else {
            // R10mOhm
            co1_msb.remove(ChargeOption1MsbFlags::RSNS_RSR);
        }

        if rsns_ac == SenseResistorValue::R5mOhm {
            co1_msb.insert(ChargeOption1MsbFlags::RSNS_RAC);
        } else {
            // R10mOhm
            co1_msb.remove(ChargeOption1MsbFlags::RSNS_RAC);
        }

        Self {
            rsns_bat,
            rsns_ac,
            charge_option0: ChargeOption0::default(), // Uses datasheet reset E70Eh
            charge_option1: ChargeOption1 {
                msb_flags: co1_msb,
                lsb_flags: ChargeOption1Flags::from_bits_truncate(0x00), // Datasheet reset for LSB of ChargeOption1 (0x30h) is 0x00
            },
            charge_current: 0x0000, // Default 0A.
            charge_voltage: match cell_count {
                1 => 0x1068, // 4.2V.
                2 => 0x20D0, // 8.4V.
                3 => 0x3138, // 12.6V.
                4 => 0x41A0, // 16.8V.
                5 => 0x5208, // 21.0V.
                _ => 0x41A0, // Default to 4S
            },
            input_voltage: 0x00C8, // Default VINDPM 16V (raw 0x00C8 based on (16000-3200)/64 = 200 = 0xC8 for LSB, MSB D8=0)
            // The register value is LSB=0xC8, MSB=0x00 (bit 5 for D8) -> 0x00C8
            vsys_min: match cell_count {
                // VSYS_MIN is 8 bits in MSB (0x0D), LSB (0x0C) is 0x00.
                1 => 0x2400, // 3.6V. Raw MSB 0x24.
                2 => 0x4200, // 6.6V. Raw MSB 0x42.
                3 => 0x5C00, // 9.2V. Raw MSB 0x5C.
                4 => 0x7B00, // 12.3V. Raw MSB 0x7B.
                5 => 0x9A00, // 15.4V. Raw MSB 0x9A.
                _ => 0x7B00, // Default to 4S
            },
            iin_host: match rsns_ac {
                // IIN_HOST is 7 bits in MSB (0x0F), LSB (0x0E) is 0x00.
                // For 5mOhm, 3.2A. (3200-100)/100 = 31 = 0x1F. So MSB=0x1F. Value = 0x1F00
                // For 10mOhm, 3.25A. (3250-50)/50 = 64 = 0x40. So MSB=0x40. Value = 0x4000
                SenseResistorValue::R5mOhm => 0x1F00,
                SenseResistorValue::R10mOhm => 0x4000,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            4,
            SenseResistorValue::default(),
            SenseResistorValue::default(),
        )
    }
}
