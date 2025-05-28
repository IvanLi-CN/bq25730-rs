#![allow(clippy::approx_constant)]

#[cfg(feature = "defmt")]
use defmt::Format;

/// Represents the status of the BQ25730 charger.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
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

/// Represents the PROCHOT status of the BQ25730.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
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

/// Represents the Charge Current setting in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeCurrent(pub u16);

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
pub struct ChargeVoltage(pub u16);

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
pub struct OtgVoltage(pub u16);

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
pub struct OtgCurrent(pub u16);

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
pub struct InputVoltage(pub u16);

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
        let raw_value = (((msb >> 5) & 0x01) as u16) << 8 | (lsb as u16); // D8-D0 (D8 is bit 5 of MSB)
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
pub struct VsysMin(pub u16);

impl VsysMin {
    /// LSB value for Minimum System Voltage in mV.
    pub const LSB_MV: u16 = 100;

    /// Creates a new VsysMin from a raw 8-bit register value (LSB at 0x0C).
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
pub struct IinHost(pub u16);

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
pub struct IinDpm(pub u16);

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
pub struct AdcPsys(pub u16);

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
pub struct AdcVbus(pub u16);

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
pub struct AdcIdchg(pub u16);

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
pub struct AdcIchg(pub u16);

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
pub struct AdcCmpin(pub u16);

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
pub struct AdcIin(pub u16);

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
pub struct AdcVbat(pub u16);

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
pub struct AdcVsys(pub u16);

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

impl AdcMeasurements {
    // This function is no longer needed as AdcMeasurements is constructed directly in Bq25730::read_adc_measurements
}

/// Represents the ChargeOption0 register settings.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeOption0 {
    pub en_low_power: bool,
    pub wdtmr_adj: u8, // 00b: Disable, 01b: 5s, 10b: 88s, 11b: 175s
    pub iin_dpm_auto_disable: bool,
    pub otg_on_chrgok: bool,
    pub en_ooa: bool,
    pub pwm_freq: bool, // 0b: 800kHz, 1b: 400kHz
    pub low_ptm_ripple: bool,
    pub en_cmp_latch: bool,
    pub vsys_uvp_enz: bool,
    pub en_learn: bool,
    pub iadpt_gain: bool, // 0b: 20x, 1b: 40x
    pub ibat_gain: bool,  // 0b: 8x, 1b: 16x
    pub en_ldo: bool,
    pub en_iin_dpm: bool,
    pub chrg_inhibit: bool,
}

impl ChargeOption0 {
    /// Creates a new ChargeOption0 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_low_power: (msb & 0x80) != 0,         // Bit 7 of MSB (0x01)
            wdtmr_adj: (msb >> 5) & 0x03,            // Bits 6:5 of MSB (0x01)
            iin_dpm_auto_disable: (msb & 0x10) != 0, // Bit 4 of MSB (0x01)
            otg_on_chrgok: (msb & 0x08) != 0,        // Bit 3 of MSB (0x01)
            en_ooa: (msb & 0x04) != 0,               // Bit 2 of MSB (0x01)
            pwm_freq: (msb & 0x02) != 0,             // Bit 1 of MSB (0x01)
            low_ptm_ripple: (msb & 0x01) != 0,       // Bit 0 of MSB (0x01)
            en_cmp_latch: (lsb & 0x80) != 0,         // Bit 7 of LSB (0x00)
            vsys_uvp_enz: (lsb & 0x40) != 0,         // Bit 6 of LSB (0x00)
            en_learn: (lsb & 0x20) != 0,             // Bit 5 of LSB (0x00)
            iadpt_gain: (lsb & 0x10) != 0,           // Bit 4 of LSB (0x00)
            ibat_gain: (lsb & 0x08) != 0,            // Bit 3 of LSB (0x00)
            en_ldo: (lsb & 0x04) != 0,               // Bit 2 of LSB (0x00)
            en_iin_dpm: (lsb & 0x02) != 0,           // Bit 1 of LSB (0x00)
            chrg_inhibit: (lsb & 0x01) != 0,         // Bit 0 of LSB (0x00)
        }
    }

    /// Converts the ChargeOption0 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;

        if self.en_low_power {
            msb |= 0x80;
        }
        msb |= (self.wdtmr_adj & 0x03) << 5;
        if self.iin_dpm_auto_disable {
            msb |= 0x10;
        }
        if self.otg_on_chrgok {
            msb |= 0x08;
        }
        if self.en_ooa {
            msb |= 0x04;
        }
        if self.pwm_freq {
            msb |= 0x02;
        }
        if self.low_ptm_ripple {
            msb |= 0x01;
        }
        if self.en_cmp_latch {
            lsb |= 0x80;
        }
        if self.vsys_uvp_enz {
            lsb |= 0x40;
        }
        if self.en_learn {
            lsb |= 0x20;
        }
        if self.iadpt_gain {
            lsb |= 0x10;
        }
        if self.ibat_gain {
            lsb |= 0x08;
        }
        if self.en_ldo {
            lsb |= 0x04;
        }
        if self.en_iin_dpm {
            lsb |= 0x02;
        }
        if self.chrg_inhibit {
            lsb |= 0x01;
        }
        (lsb, msb)
    }
}

/// Represents the ChargeOption1 register settings.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeOption1 {
    pub en_ibat: bool,
    pub en_prochot_lpwr: bool,
    pub psys_config: u8, // 00b: PBUS+PBAT, 01b: PBUS, 10b: Reserved, 11b: Disabled
    pub rsns_rac: bool,  // 0b: 10mOhm, 1b: 5mOhm
    pub rsns_rsr: bool,  // 0b: 10mOhm, 1b: 5mOhm
    pub psys_ratio: bool, // 0b: 0.25A/W, 1b: 1A/W
    pub cmp_ref: bool,   // 0b: 2.3V, 1b: 1.2V
    pub cmp_pol: bool, // 0b: CMPOUT LOW when CMPIN above threshold, 1b: CMPOUT LOW when CMPIN below threshold
    pub cmp_deg: u8,   // 00b: 5us, 01b: 2ms, 10b: 20ms, 11b: 5s
    pub force_conv_off: bool,
    pub en_ptm: bool,
    pub en_ship_dchg: bool,
    pub auto_wakeup_en: bool,
}

impl ChargeOption1 {
    /// Creates a new ChargeOption1 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_ibat: (msb & 0x80) != 0,
            en_prochot_lpwr: (msb & 0x40) != 0,
            psys_config: (msb >> 4) & 0x03,
            rsns_rac: (msb & 0x08) != 0,
            rsns_rsr: (msb & 0x04) != 0,
            psys_ratio: (msb & 0x02) != 0,
            cmp_ref: (lsb & 0x80) != 0,
            cmp_pol: (lsb & 0x40) != 0,
            cmp_deg: (lsb >> 4) & 0x03,
            force_conv_off: (lsb & 0x08) != 0,
            en_ptm: (lsb & 0x04) != 0,
            en_ship_dchg: (lsb & 0x02) != 0,
            auto_wakeup_en: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption1 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;

        if self.en_ibat {
            msb |= 0x80;
        }
        if self.en_prochot_lpwr {
            msb |= 0x40;
        }
        msb |= (self.psys_config & 0x03) << 4;
        if self.rsns_rac {
            msb |= 0x08;
        }
        if self.rsns_rsr {
            msb |= 0x04;
        }
        if self.psys_ratio {
            msb |= 0x02;
        }
        if self.cmp_ref {
            lsb |= 0x80;
        }
        if self.cmp_pol {
            lsb |= 0x40;
        }
        lsb |= (self.cmp_deg & 0x03) << 4;
        if self.force_conv_off {
            lsb |= 0x08;
        }
        if self.en_ptm {
            lsb |= 0x04;
        }
        if self.en_ship_dchg {
            lsb |= 0x02;
        }
        if self.auto_wakeup_en {
            lsb |= 0x01;
        }
        (lsb, msb)
    }
}

/// Represents the ChargeOption2 register settings.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeOption2 {
    pub pkpwr_tovld_deg: u8, // 00b: 1ms, 01b: 2ms, 10b: 5ms, 11b: 10ms
    pub en_pkpwr_iin_dpm: bool,
    pub en_pkpwr_vsys: bool,
    pub stat_pkpwr_ovld: bool,
    pub stat_pkpwr_relax: bool,
    pub pkpwr_tmax: u8, // 00b: 20ms, 01b: 40ms, 10b: 80ms, 11b: 1s
    pub en_extilim: bool,
    pub en_ichg_idchg: bool,
    pub q2_ocp: bool,  // 0b: 210mV, 1b: 150mV
    pub acx_ocp: bool, // 0b: 280mV/200mV, 1b: 150mV/100mV
    pub en_acoc: bool,
    pub acoc_vth: bool, // 0b: 133%, 1b: 200%
    pub en_batoc: bool,
    pub batoc_vth: bool, // 0b: 133%, 1b: 200%
}

impl ChargeOption2 {
    /// Creates a new ChargeOption2 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            pkpwr_tovld_deg: (msb >> 6) & 0x03,
            en_pkpwr_iin_dpm: (msb & 0x20) != 0,
            en_pkpwr_vsys: (msb & 0x10) != 0,
            stat_pkpwr_ovld: (msb & 0x08) != 0,
            stat_pkpwr_relax: (msb & 0x04) != 0,
            pkpwr_tmax: msb & 0x03,
            en_extilim: (lsb & 0x80) != 0,
            en_ichg_idchg: (lsb & 0x40) != 0,
            q2_ocp: (lsb & 0x20) != 0,
            acx_ocp: (lsb & 0x10) != 0,
            en_acoc: (lsb & 0x08) != 0,
            acoc_vth: (lsb & 0x04) != 0,
            en_batoc: (lsb & 0x02) != 0,
            batoc_vth: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption2 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;

        msb |= (self.pkpwr_tovld_deg & 0x03) << 6;
        if self.en_pkpwr_iin_dpm {
            msb |= 0x20;
        }
        if self.en_pkpwr_vsys {
            msb |= 0x10;
        }
        if self.stat_pkpwr_ovld {
            msb |= 0x08;
        }
        if self.stat_pkpwr_relax {
            msb |= 0x04;
        }
        msb |= self.pkpwr_tmax & 0x03;
        if self.en_extilim {
            lsb |= 0x80;
        }
        if self.en_ichg_idchg {
            lsb |= 0x40;
        }
        if self.q2_ocp {
            lsb |= 0x20;
        }
        if self.acx_ocp {
            lsb |= 0x10;
        }
        if self.en_acoc {
            lsb |= 0x08;
        }
        if self.acoc_vth {
            lsb |= 0x04;
        }
        if self.en_batoc {
            lsb |= 0x02;
        }
        if self.batoc_vth {
            lsb |= 0x01;
        }
        (lsb, msb)
    }
}

/// Represents the ChargeOption3 register settings.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeOption3 {
    pub en_hiz: bool,
    pub reset_reg: bool,
    pub reset_vindpm: bool,
    pub en_otg: bool,
    pub en_ico_mode: bool,
    pub en_port_ctrl: bool,
    pub en_vsys_min_soft_sr: bool,
    pub en_otg_bigcap: bool,
    pub batfet_enz: bool,
    pub en_vbus_vap: bool,
    pub otg_vap_mode: bool, // 0b: EN/DIS VAP, 1b: EN/DIS OTG
    pub il_avg: u8,         // 00b: 6A, 01b: 10A, 10b: 15A, 11b: Disabled
    pub cmp_en: bool,
    pub batfetoff_hiz: bool,
    pub psys_otg_idchg: bool,
}

impl ChargeOption3 {
    /// Creates a new ChargeOption3 from raw LSB and MSB register values.
    pub fn from_register_value(lsb: u8, msb: u8) -> Self {
        Self {
            en_hiz: (msb & 0x80) != 0,
            reset_reg: (msb & 0x40) != 0,
            reset_vindpm: (msb & 0x20) != 0,
            en_otg: (msb & 0x10) != 0,
            en_ico_mode: (msb & 0x08) != 0,
            en_port_ctrl: (msb & 0x04) != 0,
            en_vsys_min_soft_sr: (msb & 0x02) != 0,
            en_otg_bigcap: (msb & 0x01) != 0,
            batfet_enz: (lsb & 0x80) != 0,
            en_vbus_vap: (lsb & 0x40) != 0,
            otg_vap_mode: (lsb & 0x20) != 0,
            il_avg: (lsb >> 3) & 0x03,
            cmp_en: (lsb & 0x04) != 0,
            batfetoff_hiz: (lsb & 0x02) != 0,
            psys_otg_idchg: (lsb & 0x01) != 0,
        }
    }

    /// Converts the ChargeOption3 to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let mut lsb: u8 = 0;
        let mut msb: u8 = 0;

        if self.en_hiz {
            msb |= 0x80;
        }
        if self.reset_reg {
            msb |= 0x40;
        }
        if self.reset_vindpm {
            msb |= 0x20;
        }
        if self.en_otg {
            msb |= 0x10;
        }
        if self.en_ico_mode {
            msb |= 0x08;
        }
        if self.en_port_ctrl {
            msb |= 0x04;
        }
        if self.en_vsys_min_soft_sr {
            msb |= 0x02;
        }
        if self.en_otg_bigcap {
            msb |= 0x01;
        }
        if self.batfet_enz {
            lsb |= 0x80;
        }
        if self.en_vbus_vap {
            lsb |= 0x40;
        }
        if self.otg_vap_mode {
            lsb |= 0x20;
        }
        lsb |= (self.il_avg & 0x03) << 3;
        if self.cmp_en {
            lsb |= 0x04;
        }
        if self.batfetoff_hiz {
            lsb |= 0x02;
        }
        if self.psys_otg_idchg {
            lsb |= 0x01;
        }
        (lsb, msb)
    }
}
