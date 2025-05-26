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
    pub const LSB_MA: u16 = 64;

    /// Creates a new ChargeCurrent from a raw 13-bit register value.
    /// The value is stored across two bytes (MSB at 0x03, LSB at 0x02).
    /// The 13-bit value is formed by `((msb & 0x1F) << 8) | lsb`.
    /// Creates a new ChargeCurrent from raw MSB and LSB register values.
    /// The 13-bit value (D12-D0) is formed by:
    /// MSB (0x03): D12-D8 in bits 4:0
    /// LSB (0x02): D4-D0 in bits 7:3
    pub fn from_register_value(msb: u8, lsb: u8) -> Self {
        let raw_value = (((msb & 0x1F) as u16) << 8) | (lsb as u16);
        ChargeCurrent(raw_value * Self::LSB_MA)
    }

    /// Converts the ChargeCurrent to raw MSB and LSB register values.
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MA;
        // ChargeCurrent is a 13-bit value (D12-D0)
        // MSB (0x03): D12-D8 in bits 4:0
        // LSB (0x02): D7-D0 in bits 7:0
        let msb = ((raw_value >> 8) & 0x1F) as u8; // D12-D8
        let lsb = (raw_value & 0xFF) as u8; // D7-D0
        (msb, lsb)
    }
}

/// Represents the Charge Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct ChargeVoltage(pub u16);

impl ChargeVoltage {
    /// LSB value for Charge Voltage in mV.
    pub const LSB_MV: u16 = 8;

    /// Creates a new ChargeVoltage from a raw 12-bit register value.
    /// The value is stored across two bytes (MSB at 0x05, LSB at 0x04).
    /// The 12-bit value is formed by `((msb & 0x0F) << 8) | lsb`.
    pub fn from_register_value(msb: u8, lsb: u8) -> Self {
        // ChargeVoltage is a 12-bit value (D11-D0)
        // MSB (0x05) contains D11-D4
        // LSB (0x04) contains D3-D0
        // ChargeVoltage is a 12-bit value (D11-D0)
        // MSB (0x05) contains D11-D4
        // LSB (0x04) contains D3-D0
        let raw_value = ((msb as u16) << 4) | ((lsb >> 4) as u16);
        ChargeVoltage(1024 + raw_value * Self::LSB_MV) // Add 1024mV offset
    }

    /// Converts the ChargeVoltage to raw MSB and LSB register values.
    /// The 12-bit value (D11-D0) is formed by:
    /// MSB (0x05): D11-D4 in bits 7:0
    /// LSB (0x04): D3-D0 in bits 7:4
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = (self.0 - 1024) / Self::LSB_MV; // Subtract 1024mV offset
        let msb = (raw_value >> 4) as u8; // D11-D4 in bits 7:0
        let lsb = ((raw_value & 0x0F) << 4) as u8; // D3-D0 in bits 7:4
        (msb, lsb)
    }
}

/// Represents the OTG Voltage setting in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct OtgVoltage(pub u16);

impl OtgVoltage {
    /// LSB value for OTG Voltage in mV.
    pub const LSB_MV: u16 = 8;

    /// Creates a new OtgVoltage from a raw 11-bit register value.
    /// The value is stored across two bytes (MSB at 0x07, LSB at 0x06).
    /// The 11-bit value is formed by `((msb & 0x07) << 8) | lsb`.
    /// Creates a new OtgVoltage from raw MSB and LSB register values.
    /// The 11-bit value (D10-D0) is formed by:
    /// MSB (0x07): D10-D8 in bits 2:0
    /// LSB (0x06): D7-D0 in bits 7:0
    pub fn from_register_value(msb: u8, lsb: u8) -> Self {
        let raw_value = (((msb & 0x07) as u16) << 8) | (lsb as u16);
        OtgVoltage(raw_value * Self::LSB_MV + 3000) // Add offset
    }

    /// Converts the OtgVoltage to raw MSB and LSB register values.
    /// The 11-bit value (D10-D0) is formed by:
    /// MSB (0x07): D10-D3 in bits 7:0
    /// LSB (0x06): D2-D0 in bits 7:5
    /// Converts the OtgVoltage to raw MSB and LSB register values.
    /// The 11-bit value (D10-D0) is formed by:
    /// MSB (0x07): D10-D8 in bits 2:0
    /// LSB (0x06): D7-D0 in bits 7:0
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = (self.0 - 3000) / Self::LSB_MV; // Subtract offset
        let msb = ((raw_value >> 8) & 0x07) as u8; // D10-D8 in bits 2:0
        let lsb = (raw_value & 0xFF) as u8; // D7-D0 in bits 7:0
        (msb, lsb)
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
    /// The 10-bit value (D9-D0) is formed by:
    /// MSB (0x09): D9-D8 in bits 1:0
    /// LSB (0x08): D7-D0 in bits 7:0
    pub fn from_register_value(msb: u8, lsb: u8) -> Self {
        // D9 is in bit 1 of MSB (0x09), D8 is in bit 0 of MSB (0x09)
        let raw_value = (((msb & 0x03) as u16) << 8) | (lsb as u16);
        OtgCurrent(raw_value * Self::LSB_MA)
    }

    /// Converts the OtgCurrent to raw MSB and LSB register values.
    /// The 10-bit value (D9-D0) is formed by:
    /// MSB (0x09): D9-D2 in bits 7:0
    /// LSB (0x08): D1-D0 in bits 7:6
    /// Converts the OtgCurrent to raw MSB and LSB register values.
    /// The 10-bit value (D9-D0) is formed by:
    /// MSB (0x09): D9-D8 in bits 1:0
    /// LSB (0x08): D7-D0 in bits 7:0
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = self.0 / Self::LSB_MA;
        // OTGCurrent is a 10-bit value (D9-D0)
        // MSB (0x09): D9-D8 in bits 1:0
        // LSB (0x08): D7-D0 in bits 7:0
        let msb = ((raw_value >> 8) & 0x03) as u8; // D9-D8
        let lsb = (raw_value & 0xFF) as u8; // D7-D0
        (msb, lsb)
    }
    /// Converts the OtgCurrent to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 { // This function is likely not used for OtgCurrent as it's a 2-byte register.
        let raw_value = self.0 / Self::LSB_MA;
        (raw_value & 0xFF) as u8 // Return LSB part
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
    pub const OFFSET_MV: u16 = 3200;

    /// Creates a new InputVoltage from a raw 9-bit register value.
    /// The value is stored across two bytes (MSB at 0x0B, LSB at 0x0A).
    /// The 9-bit value is formed by `((msb & 0x01) << 8) | lsb`.
    pub fn from_register_value(msb: u8, lsb: u8) -> Self {
        // D8 is in bit 7 of MSB (0x0B)
        let raw_value = (((msb >> 7) as u16) << 8) | (lsb as u16);
        InputVoltage(raw_value * Self::LSB_MV + Self::OFFSET_MV)
    }

    /// Converts the InputVoltage to raw MSB and LSB register values.
    /// The 9-bit value (D8-D0) is formed by:
    /// MSB (0x0B): D8 in bit 7
    /// LSB (0x0A): D7-D0 in bits 7:0
    pub fn to_msb_lsb_bytes(&self) -> (u8, u8) {
        let raw_value = (self.0 - Self::OFFSET_MV) / Self::LSB_MV;
        let msb = ((raw_value >> 8) & 0x01) as u8; // D8
        let lsb = (raw_value & 0xFF) as u8; // D7-D0
        (msb << 7, lsb) // Shift D8 to bit 7 of MSB byte
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
    pub fn from_register_value(lsb: u8) -> Self {
        VsysMin((lsb as u16) * Self::LSB_MV)
    }

    /// Converts the VsysMin to a raw 8-bit register value.
    pub fn to_register_value(&self) -> u8 {
        (self.0 / Self::LSB_MV) as u8
    }
}

/// Represents the Input Current Limit Set by Host in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct IinHost(pub u16);

impl IinHost {
    /// LSB value for Input Current Limit Set by Host in mA.
    pub const LSB_MA: u16 = 50;
    /// Offset value for Input Current Limit Set by Host in mA.
    pub const OFFSET_MA: u16 = 50;

    /// Creates a new IinHost from a raw 8-bit register value (LSB at 0x0E).
    pub fn from_register_value(lsb: u8) -> Self {
        IinHost((lsb as u16) * Self::LSB_MA + Self::OFFSET_MA)
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
        (0, self.to_register_value())
    }
}

/// Represents the Input Current Limit in Use (IIN_DPM) in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct IinDpm(pub u16);

impl IinDpm {
    /// LSB value for Input Current Limit in Use in mA.
    pub const LSB_MA: u16 = 50;
    /// Offset value for Input Current Limit in Use in mA.
    pub const OFFSET_MA: u16 = 50;

    /// Creates a new IinDpm from a raw 8-bit register value (LSB at 0x24).
    pub fn from_register_value(lsb: u8) -> Self {
        IinDpm((lsb as u16) * Self::LSB_MA + Self::OFFSET_MA)
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
        (0, self.to_register_value())
    }
}

/// Represents the ADC PSYS measurement in mW.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcPsys(pub u16);

impl AdcPsys {
    /// LSB value for ADC PSYS in mW (assuming 256mW/LSB, PSYS_RATIO=0).
    pub const LSB_MW: u16 = 320; // Corrected from 256 to 320 (0.32W)

    /// Creates a new AdcPsys from a raw 8-bit register value (0x26).
    pub fn from_register_value(value: u8) -> Self {
        AdcPsys((value as u16) * Self::LSB_MW)
    }
}

/// Represents the ADC VBUS measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcVbus(pub u16);

impl AdcVbus {
    /// LSB value for ADC VBUS in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcVbus from a raw 8-bit register value (0x27).
    pub fn from_register_value(value: u8) -> Self {
        AdcVbus((value as u16) * Self::LSB_MV)
    }
}

/// Represents the ADC IDCHG measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcIdchg(pub u16);

impl AdcIdchg {
    /// LSB value for ADC IDCHG in mA.
    pub const LSB_MA: u16 = 64;

    /// Creates a new AdcIdchg from a raw 8-bit register value (0x28).
    pub fn from_register_value(value: u8) -> Self {
        AdcIdchg((value as u16) * Self::LSB_MA)
    }
}

/// Represents the ADC ICHG measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcIchg(pub u16);

impl AdcIchg {
    /// LSB value for ADC ICHG in mA.
    pub const LSB_MA: u16 = 64;

    /// Creates a new AdcIchg from a raw 8-bit register value (0x29).
    pub fn from_register_value(value: u8) -> Self {
        AdcIchg((value as u16) * Self::LSB_MA)
    }
}

/// Represents the ADC CMPIN measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcCmpin(pub u16);

impl AdcCmpin {
    /// LSB value for ADC CMPIN in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcCmpin from a raw 8-bit register value (0x2A).
    pub fn from_register_value(value: u8) -> Self {
        AdcCmpin((value as u16) * Self::LSB_MV)
    }
}

/// Represents the ADC IIN measurement in mA.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcIin(pub u16);

impl AdcIin {
    /// LSB value for ADC IIN in mA.
    pub const LSB_MA: u16 = 50; // Corrected from 64 to 50

    /// Creates a new AdcIin from a raw 8-bit register value (0x2B).
    pub fn from_register_value(value: u8) -> Self {
        AdcIin((value as u16) * Self::LSB_MA)
    }
}

/// Represents the ADC VBAT measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcVbat(pub u16);

impl AdcVbat {
    /// LSB value for ADC VBAT in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcVbat from a raw 8-bit register value (0x2C).
    pub fn from_register_value(value: u8) -> Self {
        AdcVbat((value as u16) * Self::LSB_MV)
    }
}

/// Represents the ADC VSYS measurement in mV.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub struct AdcVsys(pub u16);

impl AdcVsys {
    /// LSB value for ADC VSYS in mV.
    pub const LSB_MV: u16 = 64;

    /// Creates a new AdcVsys from a raw 8-bit register value (0x2D).
    pub fn from_register_value(value: u8) -> Self {
        AdcVsys((value as u16) * Self::LSB_MV)
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
    /// Creates a new AdcMeasurements struct from raw register values.
    pub fn from_register_values(
        psys: u8,
        vbus: u8,
        idchg: u8,
        ichg: u8,
        cmpin: u8,
        iin: u8,
        vbat: u8,
        vsys: u8,
    ) -> Self {
        Self {
            psys: AdcPsys::from_register_value(psys),
            vbus: AdcVbus::from_register_value(vbus),
            idchg: AdcIdchg::from_register_value(idchg),
            ichg: AdcIchg::from_register_value(ichg),
            cmpin: AdcCmpin::from_register_value(cmpin),
            iin: AdcIin::from_register_value(iin),
            vbat: AdcVbat::from_register_value(vbat),
            vsys: AdcVsys::from_register_value(vsys),
        }
    }
}