#![allow(clippy::approx_constant)]

#[cfg(feature = "defmt")]
use defmt::Format;

use crate::registers::{
    AdcOptionFlags, AdcOptionMsbFlags, ChargeOption0Flags, ChargeOption0MsbFlags,
    ChargeOption1Flags, ChargeOption1MsbFlags, ChargeOption2Flags, ChargeOption2MsbFlags,
    ChargeOption3Flags, ChargeOption3MsbFlags, ChargeOption4Flags, ChargeOption4MsbFlags,
    ChargerStatusFaultFlags, ChargerStatusFlags, ProchotOption0Flags, ProchotOption0MsbFlags,
    ProchotOption1Flags, ProchotOption1MsbFlags, ProchotStatusFlags, ProchotStatusMsbFlags,
    VminActiveProtectionFlags, VminActiveProtectionMsbFlags,
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
        if self.bits() == 0 {
            defmt::write!(fmt, "(empty)");
            return;
        }
        let mut first = true;
        if self.contains(ProchotStatusMsbFlags::EN_PROCHOT_EXT) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "EN_PROCHOT_EXT");
            first = false;
        }

        // Extract and display PROCHOT_WIDTH value (bits 5:4 of MSB)
        let width_val = (self.bits() & ProchotStatusMsbFlags::PROCHOT_WIDTH.bits()) >> 4;
        // PROCHOT_WIDTH is a field, not a simple flag. We'll show its value.
        // The ProchotStatus struct itself also decodes this, but showing it here makes the MSB flags more complete.
        if self.intersects(ProchotStatusMsbFlags::PROCHOT_WIDTH) { // If any of the width bits are set
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "PROCHOT_WIDTH_VAL={}", width_val);
            first = false;
        }

        if self.contains(ProchotStatusMsbFlags::PROCHOT_CLEAR) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "PROCHOT_CLEAR");
            first = false;
        }
        if self.contains(ProchotStatusMsbFlags::STAT_VAP_FAIL) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_VAP_FAIL");
            first = false;
        }
        if self.contains(ProchotStatusMsbFlags::STAT_EXIT_VAP) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_EXIT_VAP");
            // first = false; // last one
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotStatusFlags {
    fn format(&self, fmt: defmt::Formatter) {
        if self.is_empty() {
            defmt::write!(fmt, "(empty)");
            return;
        }
        let mut first = true;
        if self.contains(ProchotStatusFlags::STAT_VINDPM) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_VINDPM");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_COMP) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_COMP");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_ICRIT) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_ICRIT");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_INOM) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_INOM");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_IDCHG1) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_IDCHG1");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_VSYS) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_VSYS");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_BAT_REMOVAL) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_BAT_REMOVAL");
            first = false;
        }
        if self.contains(ProchotStatusFlags::STAT_ADPT_REMOVAL) {
            if !first { defmt::write!(fmt, " | "); }
            defmt::write!(fmt, "STAT_ADPT_REMOVAL");
            // first = false; // last one
        }
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

/// Represents the Charge Current setting.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(little))] // Assuming LSB first for raw u16
#[cfg_attr(feature = "binrw", bw(little))] // Assuming LSB first for raw u16
pub struct ChargeCurrentSetting {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for ChargeCurrentSetting {
    fn default() -> Self {
        // Datasheet reset for REG0x03/02h is 0000h, which corresponds to 0mA.
        Self {
            milliamps: 0,
            rsns_bat: SenseResistorValue::default(),
        }
    }
}

impl ChargeCurrentSetting {
    pub fn from_milliamps(milliamps: u16, rsns_bat: SenseResistorValue) -> Self {
        Self { milliamps, rsns_bat }
    }

    pub fn to_milliamps(&self) -> u16 {
        self.milliamps
    }

    /// Creates a new ChargeCurrentSetting from a raw 16-bit register value (LSB first).
    /// LSB (02h): bits 7:6 are D1:D0. MSB (03h): bits 4:0 are D6:D2.
    pub fn from_raw(raw_value: u16, rsns_bat: SenseResistorValue) -> Self {
        let lsb_byte = (raw_value & 0xFF) as u8;
        let msb_byte = ((raw_value >> 8) & 0xFF) as u8;
        let d1_d0 = (lsb_byte >> 6) & 0x03; // Extract D1:D0 from LSB bits 7:6
        let d6_d2 = msb_byte & 0x1F;      // Extract D6:D2 from MSB bits 4:0
        let raw_7bit = (d6_d2 << 2) | d1_d0;

        let lsb_ma_val = match rsns_bat {
            SenseResistorValue::R5mOhm => 128,
            SenseResistorValue::R10mOhm => 64,
        };
        Self {
            milliamps: (raw_7bit as u16) * lsb_ma_val,
            rsns_bat,
        }
    }

    /// Converts the ChargeCurrentSetting to a raw 16-bit register value (LSB first).
    /// LSB (02h): bits 7:6 are D1:D0. MSB (03h): bits 4:0 are D6:D2.
    pub fn to_raw(&self) -> u16 {
        let lsb_ma_val = match self.rsns_bat {
            SenseResistorValue::R5mOhm => 128,
            SenseResistorValue::R10mOhm => 64,
        };
        let mut raw_7bit_val = self.milliamps / lsb_ma_val;
        if raw_7bit_val > 0x7F {
            raw_7bit_val = 0x7F; // Clamp to max 7-bit value
        }
        let raw_7bit = raw_7bit_val as u8;

        let d1_d0 = raw_7bit & 0x03;
        let d6_d2 = (raw_7bit >> 2) & 0x1F;

        // LSB (02h): bits 7:6 are D1:D0. Other bits are reserved (assume 0).
        let lsb_byte = d1_d0 << 6;
        // MSB (03h): bits 4:0 are D6:D2. Other bits are reserved (assume 0).
        let msb_byte = d6_d2;

        (msb_byte as u16) << 8 | (lsb_byte as u16)
    }

    /// Converts to MSB and LSB bytes for register writing.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_val = self.to_raw();
        ((raw_val & 0xFF) as u8, ((raw_val >> 8) & 0xFF) as u8)
    }
}

/// Represents the Charge Voltage setting.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| ChargeVoltageSetting::from_raw(x, None) ))]
#[cfg_attr(feature = "binrw", bw(map = |s: &ChargeVoltageSetting| s.to_raw() ))]
pub struct ChargeVoltageSetting {
    pub millivolts: u16,
}

impl Default for ChargeVoltageSetting {
    fn default() -> Self {
        // Default for 4S battery: 16.8V (raw 0x41A0)
        // This will be overridden by Config::new based on cell_count
        Self::from_millivolts(16800)
    }
}

impl ChargeVoltageSetting {
    pub fn from_millivolts(millivolts: u16) -> Self {
        Self { millivolts }
    }

    pub fn to_millivolts(&self) -> u16 {
        self.millivolts
    }

    /// Creates a new ChargeVoltageSetting from a 16-bit raw register value (LSB first).
    /// LSB (04h): D4-D0 in bits 7:3. MSB (05h): D11-D5 in bits 6:0.
    /// Raw value has LSB 8mV.
    pub fn from_raw(raw_value: u16, _rsns: Option<SenseResistorValue>) -> Self {
        let lsb_byte = (raw_value & 0xFF) as u8;
        let msb_byte = ((raw_value >> 8) & 0xFF) as u8;

        let d4_d0 = (lsb_byte >> 3) & 0x1F; // Extract D4-D0 from LSB bits 7:3
        let d11_d5 = msb_byte & 0x7F;      // Extract D11-D5 from MSB bits 6:0

        let combined_12bit = ((d11_d5 as u16) << 5) | (d4_d0 as u16);
        Self { millivolts: combined_12bit * 8 }
    }

    /// Converts the ChargeVoltageSetting to a raw 16-bit register value (LSB first).
    /// LSB (04h): D4-D0 in bits 7:3. MSB (05h): D11-D5 in bits 6:0.
    pub fn to_raw(&self) -> u16 {
        let mut combined_12bit = self.millivolts / 8;
        if combined_12bit > 0xFFF { // Clamp to max 12-bit value
            combined_12bit = 0xFFF;
        }


        let d4_d0 = (combined_12bit & 0x1F) as u8;      // Lower 5 bits
        let d11_d5 = ((combined_12bit >> 5) & 0x7F) as u8; // Upper 7 bits

        // LSB (04h): D4-D0 in bits 7:3. Other bits are reserved (assume 0).
        let lsb_byte = d4_d0 << 3;
        // MSB (05h): D11-D5 in bits 6:0. Other bits are reserved (assume 0).
        let msb_byte = d11_d5;

        (msb_byte as u16) << 8 | (lsb_byte as u16)
    }

    /// Converts to MSB and LSB bytes for register writing.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_val = self.to_raw();
        ((raw_val & 0xFF) as u8, ((raw_val >> 8) & 0xFF) as u8)
    }
}

/// Represents the OTG Voltage setting.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| OtgVoltageSetting::from_raw(x) ))]
#[cfg_attr(feature = "binrw", bw(map = |s: &OtgVoltageSetting| s.to_raw() ))]
pub struct OtgVoltageSetting {
    pub millivolts: u16,
}

impl Default for OtgVoltageSetting {
    fn default() -> Self {
        // Datasheet reset for REG0x07/06h is 0x09C4.
        // This corresponds to 2496mV.
        Self::from_raw(0x09C4)
    }
}

impl OtgVoltageSetting {
    /// LSB value for OTG Voltage in mV.
    pub const LSB_MV: u16 = 8;

    pub fn from_millivolts(millivolts: u16) -> Self {
        Self { millivolts }
    }

    pub fn to_millivolts(&self) -> u16 {
        self.millivolts
    }

    /// Creates a new OtgVoltageSetting from a 16-bit raw register value.
    /// According to datasheet (e.g., SLUSE65, Table 8-44, 8-45):
    /// MSB (REG0x07): D11-D6 in bits 5:0. Bit 7 is reserved, Bit 6 is unused by D11-D6.
    /// LSB (REG0x06): D5 in bit 7, D4-D0 in bits 6:2. Bits 1:0 are reserved.
    pub fn from_raw(raw_value: u16) -> Self {
        let msb_u8 = (raw_value >> 8) as u8;
        let lsb_u8 = (raw_value & 0xFF) as u8;

        // Extract D11-D6 from MSB bits 5:0 (msb_u8[5] is D11, msb_u8[0] is D6)
        let d11_d6 = (msb_u8 & 0x3F) as u16;
        // Extract D5 from LSB bit 7
        let d5 = ((lsb_u8 >> 7) & 0x01) as u16;
        // Extract D4-D0 from LSB bits 6:2 (lsb_u8[6] is D4, lsb_u8[2] is D0)
        let d4_d0 = ((lsb_u8 >> 2) & 0x1F) as u16;

        // Combine D11..D6, D5, D4..D0 to form the 12-bit value
        let combined_12bit = (d11_d6 << 6) | (d5 << 5) | d4_d0;
        Self {
            millivolts: combined_12bit * Self::LSB_MV,
        }
    }

    /// Converts the OtgVoltageSetting to a raw 16-bit register value.
    /// According to datasheet (e.g., SLUSE65, Table 8-44, 8-45):
    /// MSB (REG0x07): D11-D6 in bits 5:0. Bit 7 is reserved, Bit 6 is unused by D11-D6.
    /// LSB (REG0x06): D5 in bit 7, D4-D0 in bits 6:2. Bits 1:0 are reserved.
    pub fn to_raw(&self) -> u16 {
        let mut raw_12bit = self.millivolts / Self::LSB_MV;
        if raw_12bit > 0xFFF { // Clamp to max 12-bit value (4095)
            raw_12bit = 0xFFF;
        }

        // D11-D6 are bits 11:6 of raw_12bit
        let val_d11_d6 = ((raw_12bit >> 6) & 0x3F) as u8;
        // D5 is bit 5 of raw_12bit
        let val_d5 = ((raw_12bit >> 5) & 0x01) as u8;
        // D4-D0 are bits 4:0 of raw_12bit
        let val_d4_d0 = (raw_12bit & 0x1F) as u8;

        // MSB (REG0x07): D11-D6 in bits 5:0. Bit 7 reserved (0), Bit 6 unused (0).
        // So, msb_u8 becomes 00<D11..D6>
        let msb_u8 = val_d11_d6;

        // LSB (REG0x06): D5 in bit 7, D4-D0 in bits 6:2. Bits 1:0 reserved (0).
        // So, lsb_u8 becomes <D5><D4..D0>00
        let lsb_u8 = (val_d5 << 7) | (val_d4_d0 << 2);

        (msb_u8 as u16) << 8 | (lsb_u8 as u16)
    }

    /// Converts to MSB and LSB bytes for register writing.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_val = self.to_raw();
        // LSB is first byte, MSB is second byte in the pair for register map
        ((raw_val & 0xFF) as u8, ((raw_val >> 8) & 0xFF) as u8)
    }
}

/// Represents the OTG Current setting.
/// REG0x09/08h: MSB (09h) D6-D0, LSB (08h) is reserved (0x00).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| OtgCurrentSetting::from_raw((x >> 8) as u8, SenseResistorValue::default()) ))] // Reads MSB for raw value
#[cfg_attr(feature = "binrw", bw(map = |s: &OtgCurrentSetting| (s.to_raw() as u16) << 8 ))] // Writes raw value to MSB
pub struct OtgCurrentSetting {
    pub milliamps: u16,
    pub rsns_bat: SenseResistorValue,
}

impl Default for OtgCurrentSetting {
    fn default() -> Self {
        // Datasheet reset for REG0x09/08h is 0x3C00.
        // MSB (0x09h) is 0x3C (raw 7-bit value). LSB (0x08h) is 0x00.
        Self::from_raw(0x3C, SenseResistorValue::default())
    }
}

impl OtgCurrentSetting {
    pub fn from_milliamps(milliamps: u16, rsns_bat: SenseResistorValue) -> Self {
        Self { milliamps, rsns_bat }
    }

    pub fn to_milliamps(&self) -> u16 {
        self.milliamps
    }

    /// Creates a new OtgCurrentSetting from a raw 7-bit register value (from MSB REG0x09h) and RSNS setting.
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

    /// Converts the OtgCurrentSetting to a raw 7-bit register value (for MSB REG0x09h).
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

    /// Converts to MSB and LSB bytes for register writing.
    /// LSB (REG0x08h) is reserved and should be 0x00.
    /// MSB (REG0x09h) contains the 7-bit raw current value.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        (0x00, self.to_raw())
    }
}

/// Represents the Input Voltage (VINDPM) setting.
/// REG0x0B/0Ah: LSB (0Ah) D7-D0, MSB (0Bh) D8 in bit 5.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| InputVoltageSetting::from_raw(x) ))]
#[cfg_attr(feature = "binrw", bw(map = |s: &InputVoltageSetting| s.to_raw() ))]
pub struct InputVoltageSetting {
    pub millivolts: u16,
}

impl Default for InputVoltageSetting {
    fn default() -> Self {
        // Datasheet reset for REG0x0B/0Ah is 0x00C8.
        // LSB (0Ah) = 0xC8, MSB (0Bh) = 0x00.
        // This corresponds to (200 * 64) + 3200 = 12800 + 3200 = 16000 mV = 16V.
        Self::from_raw(0x00C8)
    }
}

impl InputVoltageSetting {
    /// LSB value for Input Voltage in mV.
    pub const LSB_MV: u16 = 64;
    /// Offset value for Input Voltage in mV.
    pub const OFFSET_MV: u16 = 3200; // 3200mV offset

    pub fn from_millivolts(millivolts: u16) -> Self {
        Self { millivolts }
    }

    pub fn to_millivolts(&self) -> u16 {
        self.millivolts
    }

    /// Creates a new InputVoltageSetting from a 16-bit raw register value (LSB first).
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn from_raw(raw_value: u16) -> Self {
        let lsb_byte = (raw_value & 0xFF) as u8;
        let msb_byte = ((raw_value >> 8) & 0xFF) as u8;

        // D8 is in bit 5 of MSB (0x0B)
        let d8 = (msb_byte >> 5) & 0x01;
        // D7-D0 are in LSB (0x0A)
        let d7_d0 = lsb_byte;

        let combined_9bit = ((d8 as u16) << 8) | (d7_d0 as u16);
        Self {
            millivolts: combined_9bit * Self::LSB_MV + Self::OFFSET_MV,
        }
    }

    /// Converts the InputVoltageSetting to a raw 16-bit register value (LSB first).
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 5
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn to_raw(&self) -> u16 {
        let mut raw_9bit = if self.millivolts >= Self::OFFSET_MV {
            (self.millivolts - Self::OFFSET_MV) / Self::LSB_MV
        } else {
            0 // Clamp to the minimum register value
        };

        if raw_9bit > 0x1FF { // Clamp to max 9-bit value
            raw_9bit = 0x1FF;
        }

        let d8 = ((raw_9bit >> 8) & 0x01) as u8; // D8 is bit 8 of the 9-bit value
        let d7_d0 = (raw_9bit & 0xFF) as u8;   // D7-D0 are bits 7:0

        // MSB (0x0B): D8 in bit 5. Other bits are reserved (assume 0).
        let msb_byte = d8 << 5;
        // LSB (0x0A): D7-D0 in bits 7:0.
        let lsb_byte = d7_d0;

        (msb_byte as u16) << 8 | (lsb_byte as u16)
    }

    /// Converts to MSB and LSB bytes for register writing.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_val = self.to_raw();
        // LSB is first byte, MSB is second byte in the pair for register map
        ((raw_val & 0xFF) as u8, ((raw_val >> 8) & 0xFF) as u8)
    }
}

/// Represents the Minimum System Voltage (VSYS_MIN) setting.
/// REG0x0D/0Ch: MSB (0Dh) D7-D0, LSB (0Ch) is reserved (0x00).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| VsysMinSetting::from_raw(x) ))]
#[cfg_attr(feature = "binrw", bw(map = |s: &VsysMinSetting| s.to_raw() ))]
pub struct VsysMinSetting {
    pub millivolts: u16,
}

impl Default for VsysMinSetting {
    fn default() -> Self {
        // Datasheet default for 4S (from Config::new logic): 12.3V
        // 12300mV / 100mV/LSB = 123 (0x7B)
        // Raw value: MSB=0x7B, LSB=0x00 => 0x7B00
        Self::from_raw(0x7B00)
    }
}

impl VsysMinSetting {
    /// LSB value for Minimum System Voltage in mV.
    pub const LSB_MV: u16 = 100;

    pub fn from_millivolts(millivolts: u16) -> Self {
        Self { millivolts }
    }

    pub fn to_millivolts(&self) -> u16 {
        self.millivolts
    }

    /// Creates a new VsysMinSetting from a 16-bit raw register value (LSB first).
    /// MSB (0x0D) contains the 8-bit value. LSB (0x0C) is reserved (0x00).
    pub fn from_raw(raw_value: u16) -> Self {
        let msb_byte = ((raw_value >> 8) & 0xFF) as u8; // Extract MSB
        Self {
            millivolts: (msb_byte as u16) * Self::LSB_MV,
        }
    }

    /// Converts the VsysMinSetting to a raw 16-bit register value (LSB first).
    /// MSB (0x0D) contains the 8-bit value. LSB (0x0C) is 0x00.
    pub fn to_raw(&self) -> u16 {
        let mut msb_val = self.millivolts / Self::LSB_MV;
        if msb_val > 0xFF { // Clamp to max 8-bit value
            msb_val = 0xFF;
        }
        (msb_val as u16) << 8 // LSB is 0x00
    }

    /// Converts to MSB and LSB bytes for register writing.
    /// LSB (REG0x0Ch) is 0x00. MSB (REG0x0Dh) contains the 8-bit value.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_val = self.to_raw();
        // LSB is first byte, MSB is second byte
        ((raw_val & 0xFF) as u8, ((raw_val >> 8) & 0xFF) as u8)
    }
}

/// Represents the Input Current Limit Set by Host (IIN_HOST) setting.
/// REG0x0F/0Eh: MSB (0Fh) D6-D0, LSB (0Eh) is reserved (0x00).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = |x: u16| IinHostSetting::from_milliamps(x) ))]
#[cfg_attr(feature = "binrw", bw(map = |s: &IinHostSetting| s.to_milliamps() ))]
pub struct IinHostSetting {
    pub milliamps: u16,
}

impl Default for IinHostSetting {
    fn default() -> Self {
        Self { milliamps: 0 } // Default to 0mA, specific value set in Config::new
    }
}

impl IinHostSetting {
    pub fn from_milliamps(milliamps: u16) -> Self {
        Self { milliamps }
    }

    pub fn to_milliamps(&self) -> u16 {
        self.milliamps
    }

    /// Creates a new IinHostSetting from a raw 16-bit register value and RSNS_AC setting.
    /// `raw_reg_value` is the 16-bit content (e.g., 0x1F00). MSB (0x0F) has current, LSB (0x0E) is 0x00.
    pub fn from_raw(raw_reg_value: u16, rsns_ac: SenseResistorValue) -> Self {
        let raw_7bit = (raw_reg_value >> 8) as u8; // Extract 7-bit current code from MSB
        let (lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100), // LSB 100mA, Offset 100mA
            SenseResistorValue::R10mOhm => (50, 50),   // LSB 50mA, Offset 50mA
        };
        let milliamps = (raw_7bit as u16) * lsb_ma + offset_ma;
        Self { milliamps }
    }

    /// Converts the IinHostSetting to a raw 16-bit register value using RSNS_AC.
    /// Returns a 16-bit value where MSB is the 7-bit current code and LSB is 0x00.
    pub fn to_raw(&self, rsns_ac: SenseResistorValue) -> u16 {
        let (lsb_ma, offset_ma) = match rsns_ac {
            SenseResistorValue::R5mOhm => (100, 100),
            SenseResistorValue::R10mOhm => (50, 50),
        };
        let mut raw_7bit_val = 0;
        if self.milliamps >= offset_ma {
            raw_7bit_val = (self.milliamps - offset_ma) / lsb_ma;
        }
        if raw_7bit_val > 0x7F { // Clamp to max 7-bit value (127)
            raw_7bit_val = 0x7F;
        }
        (raw_7bit_val as u16) << 8 // Place 7-bit code in MSB, LSB is 0x00
    }

    /// Converts to MSB and LSB bytes for register writing, using RSNS_AC.
    /// LSB (REG0x0Eh) is 0x00. MSB (REG0x0Fh) contains the 7-bit current code.
    pub fn to_msb_lsb_bytes(&self, rsns_ac: SenseResistorValue) -> (u8, u8) {
        let raw_val = self.to_raw(rsns_ac);
        // LSB is first byte, MSB is second byte
        ((raw_val & 0xFF) as u8, (raw_val >> 8) as u8)
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

impl Default for ChargeOption3 {
    fn default() -> Self {
        Self {
            msb_flags: ChargeOption3MsbFlags::from_bits_truncate(0x04), // Reset MSB 0x04
            lsb_flags: ChargeOption3Flags::from_bits_truncate(0x34),   // Reset LSB 0x34
        }
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

/// Represents the ChargeOption4 register.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = ChargeOption4::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct ChargeOption4 {
    pub msb_flags: ChargeOption4MsbFlags,
    pub lsb_flags: ChargeOption4Flags,
}

impl Default for ChargeOption4 {
    fn default() -> Self {
        Self {
            msb_flags: ChargeOption4MsbFlags::from_bits_truncate(0x00), // Reset MSB 0x00
            lsb_flags: ChargeOption4Flags::from_bits_truncate(0x48),   // Reset LSB 0x48
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption4 {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ChargeOption4 {{ msb_flags: {=u8:b}, lsb_flags: {=u8:b} }}",
            self.msb_flags.bits(),
            self.lsb_flags.bits()
        );
    }
}

impl ChargeOption4 {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: ChargeOption4MsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: ChargeOption4Flags::from_bits_truncate((value & 0xFF) as u8),
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

/// Represents the VminActiveProtection register (REG0x3F/3Eh).
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
#[cfg_attr(feature = "binrw", br(map = VminActiveProtection::from_u16))]
#[cfg_attr(feature = "binrw", bw(map = |&s: &Self| s.to_u16()))]
pub struct VminActiveProtection {
    pub msb_flags: VminActiveProtectionMsbFlags,
    pub lsb_flags: VminActiveProtectionFlags,
}

impl Default for VminActiveProtection {
    fn default() -> Self {
        // Default for 2s-5s cell count: 0x006C
        // MSB (0x3F) = 0x00
        // LSB (0x3E) = 0x6C
        // (VSYS_TH2 = 0b011011 (27 -> 5.9V for 2s-5s), EN_VSYSTH2_FOLLOW_VSYSTH1 = 0, EN_FRS = 0)
        // For 1S, LSB is 0x04 (VSYS_TH2 = 0b000001 (1 -> 3.2V for 1S), EN_VSYSTH2_FOLLOW_VSYSTH1 = 0, EN_FRS = 0)
        // We'll use the 2s-5s default here, user can adjust for 1S if needed via specific methods.
        Self {
            msb_flags: VminActiveProtectionMsbFlags::from_bits_truncate(0x00),
            lsb_flags: VminActiveProtectionFlags::from_bits_truncate(0x6C),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VminActiveProtection {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "VminActiveProtection {{ msb_flags: {}, lsb_flags: {} }}",
            self.msb_flags, // Uses the custom Format impl for VminActiveProtectionMsbFlags
            self.lsb_flags  // Uses the custom Format impl for VminActiveProtectionFlags
        );
    }
}

impl VminActiveProtection {
    pub fn from_u16(value: u16) -> Self {
        Self {
            msb_flags: VminActiveProtectionMsbFlags::from_bits_truncate(((value >> 8) & 0xFF) as u8),
            lsb_flags: VminActiveProtectionFlags::from_bits_truncate((value & 0xFF) as u8),
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

    /// Converts the raw VBUS_VAP_TH value to voltage in mV.
    /// Formula: 3200mV + raw_value * 100mV
    pub fn vbus_vap_th_mv(&self) -> u16 {
        3200 + (self.msb_flags.get_vbus_vap_th() as u16) * 100
    }

    /// Sets the VBUS_VAP_TH value from voltage in mV.
    /// Clamps the value to the valid range (3200mV to 15900mV).
    pub fn set_vbus_vap_th_mv(&mut self, voltage_mv: u16) {
        let clamped_voltage = voltage_mv.clamp(3200, 15900);
        let raw_value = ((clamped_voltage - 3200) / 100) as u8;
        self.msb_flags.set_vbus_vap_th(raw_value);
    }

    /// Converts the raw VSYS_TH2 value to voltage in mV (assuming 2s-5s mode).
    /// Formula: 3200mV + raw_value * 100mV
    /// Note: This conversion might differ for 1S mode. Refer to datasheet for details.
    pub fn vsys_th2_mv(&self) -> u16 {
        3200 + (self.lsb_flags.get_vsys_th2() as u16) * 100
    }

    /// Sets the VSYS_TH2 value from voltage in mV (assuming 2s-5s mode).
    /// Clamps the value to the valid range (3200mV to 9500mV for 2s-5s).
    /// Note: This conversion might differ for 1S mode. Refer to datasheet for details.
    pub fn set_vsys_th2_mv(&mut self, voltage_mv: u16) {
        let clamped_voltage = voltage_mv.clamp(3200, 9500); // Assuming 2s-5s range
        let raw_value = ((clamped_voltage - 3200) / 100) as u8;
        self.lsb_flags.set_vsys_th2(raw_value);
    }

    /// Gets the state of the EN_VSYSTH2_FOLLOW_VSYSTH1 bit.
    pub fn en_vsysth2_follow_vsysth1(&self) -> bool {
        self.lsb_flags.get_en_vsysth2_follow_vsysth1()
    }

    /// Sets the state of the EN_VSYSTH2_FOLLOW_VSYSTH1 bit.
    pub fn set_en_vsysth2_follow_vsysth1(&mut self, enable: bool) {
        self.lsb_flags.set_en_vsysth2_follow_vsysth1(enable);
    }

    /// Gets the state of the EN_FRS bit.
    pub fn en_frs(&self) -> bool {
        self.lsb_flags.get_en_frs()
    }

    /// Sets the state of the EN_FRS bit.
    pub fn set_en_frs(&mut self, enable: bool) {
        self.lsb_flags.set_en_frs(enable);
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
    pub charge_option3: ChargeOption3,
    pub charge_option4: ChargeOption4,
    pub charge_current: ChargeCurrentSetting,
    pub charge_voltage: ChargeVoltageSetting,
    pub otg_voltage: OtgVoltageSetting,
    pub otg_current: OtgCurrentSetting,
    pub input_voltage: InputVoltageSetting,
    pub vsys_min: VsysMinSetting,
    pub iin_host: IinHostSetting,
    pub vmin_active_protection: VminActiveProtection,
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
            charge_option3: ChargeOption3::default(), // Uses datasheet reset 0434h
            charge_option4: ChargeOption4::default(), // Uses datasheet reset 0048h
            charge_current: ChargeCurrentSetting::from_raw(0x0000, rsns_bat), // Default 0A.
            charge_voltage: ChargeVoltageSetting::from_raw(
                match cell_count {
                    1 => 0x1068, // 4.2V.
                    2 => 0x20D0, // 8.4V.
                    3 => 0x3138, // 12.6V.
                    4 => 0x41A0, // 16.8V.
                    5 => 0x5208, // 21.0V.
                    _ => 0x41A0, // Default to 4S
                },
                None, // rsns not needed for voltage
            ),
            otg_voltage: OtgVoltageSetting::from_raw(0x09C4), // Datasheet reset for REG0x07/06h is 0x09C4 (2496mV)
            otg_current: OtgCurrentSetting::from_raw(0x3C, rsns_bat), // Datasheet reset for REG0x09/08h is 0x3C00 (MSB 0x3C)
            input_voltage: InputVoltageSetting::from_raw(0x00C8), // Default VINDPM 16V (raw 0x00C8)
            vsys_min: VsysMinSetting::from_raw(match cell_count {
                1 => 0x2400, // 3.6V. Raw MSB 0x24.
                2 => 0x4200, // 6.6V. Raw MSB 0x42.
                3 => 0x5C00, // 9.2V. Raw MSB 0x5C.
                4 => 0x7B00, // 12.3V. Raw MSB 0x7B.
                5 => 0x9A00, // 15.4V. Raw MSB 0x9A.
                _ => 0x7B00, // Default to 4S
            }),
            iin_host: IinHostSetting::from_raw(
                match rsns_ac {
                    // IIN_HOST is 7 bits in MSB (0x0F), LSB (0x0E) is 0x00.
                    // For 5mOhm, 3.2A. Raw MSB=0x1F. Value = 0x1F00
                    // For 10mOhm, 3.25A. Raw MSB=0x40. Value = 0x4000
                    SenseResistorValue::R5mOhm => 0x1F00,
                    SenseResistorValue::R10mOhm => 0x4000,
                },
                rsns_ac,
            ),
            vmin_active_protection: VminActiveProtection::default(), // Uses datasheet reset (e.g. 0x006C for 2s-5s)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            4, // Default to 4-cell
            SenseResistorValue::default(), // Default RsnsBat
            SenseResistorValue::default(), // Default RsnsAc
        )
    }
}
