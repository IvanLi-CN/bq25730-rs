#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// ChargeOption0 LSB Register
    ChargeOption0 = 0x00,
    /// ChargeOption0 MSB Register
    ChargeOption0Msb = 0x01,
    /// ChargeCurrent LSB Register
    ChargeCurrent = 0x02,
    /// ChargeCurrent MSB Register
    ChargeCurrentMsb = 0x03,
    /// ChargeVoltage LSB Register
    ChargeVoltage = 0x04,
    /// ChargeVoltage MSB Register
    ChargeVoltageMsb = 0x05,
    /// OTGVoltage LSB Register
    OTGVoltage = 0x06,
    /// OTGVoltage MSB Register
    OTGVoltageMsb = 0x07,
    /// OTGCurrent LSB Register
    OTGCurrent = 0x08,
    /// OTGCurrent MSB Register
    OTGCurrentMsb = 0x09,
    /// InputVoltage LSB Register
    InputVoltage = 0x0A,
    /// InputVoltage MSB Register
    InputVoltageMsb = 0x0B,
    /// VSYS_MIN LSB Register
    VsysMin = 0x0C,
    /// VSYS_MIN MSB Register
    VsysMinMsb = 0x0D,
    /// IIN_HOST LSB Register
    IinHost = 0x0E,
    /// IIN_HOST MSB Register
    IinHostMsb = 0x0F,
    /// ChargerStatus LSB Register
    ChargerStatus = 0x20,
    /// ChargerStatus MSB Register
    ChargerStatusMsb = 0x21,
    /// ProchotStatus LSB Register
    ProchotStatus = 0x22,
    /// ProchotStatus MSB Register
    ProchotStatusMsb = 0x23,
    /// IIN_DPM LSB Register
    IinDpm = 0x24,
    /// IIN_DPM MSB Register
    IinDpmMsb = 0x25,
    /// ADCPSYS Register
    ADCPSYS = 0x26,
    /// ADCVBUS Register
    ADCVBUS = 0x27,
    /// ADCIDCHG Register
    ADCIDCHG = 0x28,
    /// ADCICHG Register
    ADCICHG = 0x29,
    /// ADCCMPIN Register
    ADCCMPIN = 0x2A,
    /// ADCIIN Register
    ADCIIN = 0x2B,
    /// ADCVBAT Register
    ADCVBAT = 0x2C,
    /// ADCVSYS Register
    ADCVSYS = 0x2D,
    /// ManufacturerID Register
    ManufacturerID = 0x2E,
    /// DeviceID Register
    DeviceID = 0x2F,
    /// ChargeOption1 LSB Register
    ChargeOption1 = 0x30,
    /// ChargeOption1 MSB Register
    ChargeOption1Msb = 0x31,
    /// ChargeOption2 LSB Register
    ChargeOption2 = 0x32,
    /// ChargeOption2 MSB Register
    ChargeOption2Msb = 0x33,
    /// ChargeOption3 LSB Register
    ChargeOption3 = 0x34,
    /// ChargeOption3 MSB Register
    ChargeOption3Msb = 0x35,
    /// ProchotOption0 LSB Register
    ProchotOption0 = 0x36,
    /// ProchotOption0 MSB Register
    ProchotOption0Msb = 0x37,
    /// ProchotOption1 LSB Register
    ProchotOption1 = 0x38,
    /// ProchotOption1 MSB Register
    ProchotOption1Msb = 0x39,
    /// ADCOption LSB Register
    ADCOption = 0x3A,
    /// ADCOption MSB Register
    ADCOptionMsb = 0x3B,
    /// ChargeOption4 LSB Register
    ChargeOption4 = 0x3C,
    /// ChargeOption4 MSB Register
    ChargeOption4Msb = 0x3D,
    /// VMIN Active Protection LSB Register
    VMINActiveProtection = 0x3E,
    /// VMIN Active Protection MSB Register
    VMINActiveProtectionMsb = 0x3F,
}

use bitflags::bitflags;

/// Watchdog Timer Adjust settings (ChargeOption0 MSB bits 6:5)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WatchdogTimerAdjust {
    /// Disable Watchdog Timer
    Disabled = 0b00,
    /// Enabled, 5 sec timeout
    Sec5 = 0b01,
    /// Enabled, 88 sec timeout
    Sec88 = 0b10,
    /// Enabled, 175 sec timeout (Default)
    Sec175 = 0b11,
}

impl WatchdogTimerAdjust {
    /// Returns the bit pattern for the register, shifted to the correct position.
    pub const fn bits(self) -> u8 {
        (self as u8) << 5
    }

    /// Creates a WatchdogTimerAdjust from the raw register bits (shifted).
    pub const fn from_bits(bits: u8) -> Option<Self> {
        match (bits >> 5) & 0b11 {
            0b00 => Some(Self::Disabled),
            0b01 => Some(Self::Sec5),
            0b10 => Some(Self::Sec88),
            0b11 => Some(Self::Sec175),
            _ => None, // Should not happen with mask
        }
    }
}

bitflags! {
    /// ChargeOption0 (01h) MSB bit masks
    /// Note: WDTMR_ADJ (bits 6:5) are handled separately using the WatchdogTimerAdjust enum.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption0MsbFlags: u8 {
        const EN_LWPWR = 1 << 7;
        // WDTMR_ADJ bits (6:5) are handled by the WatchdogTimerAdjust enum
        const IIN_DPM_AUTO_DISABLE = 1 << 4;
        const OTG_ON_CHRGOK = 1 << 3;
        const EN_OOA = 1 << 2;
        const PWM_FREQ = 1 << 1;
        const LOW_PTM_RIPPLE = 1 << 0;

        /// Mask for the WDTMR_ADJ bits.
        const WDTMR_ADJ_MASK = 0b11 << 5;
    }
}

impl ChargeOption0MsbFlags {
    /// Sets the watchdog timer adjustment value.
    pub fn set_watchdog_timer(&mut self, setting: WatchdogTimerAdjust) {
        // Remove the current watchdog timer bits
        self.remove(Self::WDTMR_ADJ_MASK);
        // Create flags representing only the new setting
        let new_setting_flags = Self::from_bits_retain(setting.bits());
        // Insert the new setting bits
        self.insert(new_setting_flags);
    }

    /// Gets the watchdog timer adjustment setting.
    pub fn get_watchdog_timer(&self) -> Option<WatchdogTimerAdjust> {
        WatchdogTimerAdjust::from_bits(self.bits())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption0MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        // Format flags and watchdog setting separately for clarity
        let flags_part = self.bits() & !ChargeOption0MsbFlags::WDTMR_ADJ_MASK.bits();
        let wdt_part = self.get_watchdog_timer();
        defmt::write!(fmt, "Flags({=u8:b}) WDT({:?})", flags_part, wdt_part);
    }
}

bitflags! {
    /// ChargeOption0 (00h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption0Flags: u8 {
        const EN_CMP_LATCH = 1 << 7;
        const VSYS_UVP_ENZ = 1 << 6;
        const EN_LEARN = 1 << 5;
        const IADPT_GAIN = 1 << 4;
        const IBAT_GAIN = 1 << 3;
        const EN_LDO = 1 << 2;
        const EN_IIN_DPM = 1 << 1;
        const CHRG_INHIBIT = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption0Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargerStatus (20h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargerStatusFlags: u8 {
        const STAT_AC = 1 << 7;
        const ICO_DONE = 1 << 6;
        const IN_VAP = 1 << 5;
        const IN_VINDPM = 1 << 4;
        const IN_IIN_DPM = 1 << 3;
        const IN_FCHRG = 1 << 2;
        const IN_PCHRG = 1 << 1;
        const IN_OTG = 1 << 0;
    }
}

bitflags! {
    /// ChargerStatus (21h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargerStatusFaultFlags: u8 {
        const FAULT_ACOV = 1 << 7;
        const FAULT_BATOC = 1 << 6;
        const FAULT_ACOC = 1 << 5;
        const FAULT_SYSOVP = 1 << 4;
        const FAULT_VSYS_UVP = 1 << 3;
        const FAULT_FORCE_CONVERTER_OFF = 1 << 2;
        const FAULT_OTG_OVP = 1 << 1;
        const FAULT_OTG_UVP = 1 << 0;
    }
}

bitflags! {
    /// ProchotStatus (23h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotStatusMsbFlags: u8 {
        const EN_PROCHOT_EXT = 1 << 6;
        const PROCHOT_WIDTH = 0b11 << 4;
        const PROCHOT_CLEAR = 1 << 3;
        const STAT_VAP_FAIL = 1 << 1;
        const STAT_EXIT_VAP = 1 << 0;
    }
}

bitflags! {
    /// ProchotStatus (22h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotStatusFlags: u8 {
        const STAT_VINDPM = 1 << 7;
        const STAT_COMP = 1 << 6;
        const STAT_ICRIT = 1 << 5;
        const STAT_INOM = 1 << 4;
        const STAT_IDCHG1 = 1 << 3;
        const STAT_VSYS = 1 << 2;
        const STAT_BAT_REMOVAL = 1 << 1;
        const STAT_ADPT_REMOVAL = 1 << 0;
    }
}

bitflags! {
    /// ChargeOption1 (31h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption1MsbFlags: u8 {
        const EN_IBAT = 1 << 7;
        const EN_PROCHOT_LPWR = 1 << 6;
        const PSYS_CONFIG = 0b11 << 4;
        const RSNS_RAC = 1 << 3; // Input sense resistor RAC (0b: 10mOhm, 1b: 5mOhm)
        const RSNS_RSR = 1 << 2; // Charge sense resistor RSR (0b: 10mOhm, 1b: 5mOhm)
        const PSYS_RATIO = 1 << 1;
        // Bit 0 is EN_VAP_MODE (EN_FRS in some contexts, but datasheet for 0x31 bit 0 is "RESERVED" or related to VAP/FRS control logic not directly a simple enable)
        // Let's assume bit 0 is not directly part of Rsns config here.
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption1MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargeOption1 (30h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption1Flags: u8 {
        const CMP_REF = 1 << 7;
        const CMP_POL = 1 << 6;
        const CMP_DEG = 0b11 << 4;
        const FORCE_CONV_OFF = 1 << 3;
        const EN_PTM = 1 << 2;
        const EN_SHIP_DCHG = 1 << 1;
        const AUTO_WAKEUP_EN = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption1Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargeOption2 (33h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption2MsbFlags: u8 {
        const PKPWR_TOVLD_DEG = 0b11 << 6;
        const EN_PKPWR_IIN_DPM = 1 << 5;
        const EN_PKPWR_VSYS = 1 << 4;
        const STAT_PKPWR_OVLD = 1 << 3;
        const STAT_PKPWR_RELAX = 1 << 2;
        const PKPWR_TMAX = 0b11;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption2MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargeOption2 (32h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption2Flags: u8 {
        const EN_EXTILIM = 1 << 7;
        const EN_ICHG_IDCHG = 1 << 6;
        const Q2_OCP = 1 << 5;
        const ACX_OCP = 1 << 4;
        const EN_ACOC = 1 << 3;
        const ACOC_VTH = 1 << 2;
        const EN_BATOC = 1 << 1;
        const BATOC_VTH = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption2Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargeOption3 (35h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption3MsbFlags: u8 {
        const EN_HIZ = 1 << 7;
        const RESET_REG = 1 << 6;
        const RESET_VINDPM = 1 << 5;
        const EN_OTG = 1 << 4;
        const EN_ICO_MODE = 1 << 3;
        const EN_PORT_CTRL = 1 << 2;
        const EN_VSYS_MIN_SOFT_SR = 1 << 1;
        const EN_OTG_BIGCAP = 1 << 0;
    }
}

impl ChargeOption3MsbFlags {
    /// Sets the EN_ICO_MODE bit.
    pub fn set_en_ico_mode(&mut self, enable: bool) {
        if enable {
            self.insert(ChargeOption3MsbFlags::EN_ICO_MODE);
        } else {
            self.remove(ChargeOption3MsbFlags::EN_ICO_MODE);
        }
    }

    /// Gets the status of the EN_ICO_MODE bit.
    pub fn get_en_ico_mode(&self) -> bool {
        self.contains(ChargeOption3MsbFlags::EN_ICO_MODE)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption3MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ChargeOption3 (34h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption3Flags: u8 {
        const BATFET_ENZ = 1 << 7;
        const EN_VBUS_VAP = 1 << 6;
        const OTG_VAP_MODE = 1 << 5;
        const IL_AVG = 0b11 << 3;
        const CMP_EN = 1 << 2;
        const BATFETOFF_HIZ = 1 << 1;
        const PSYS_OTG_IDCHG = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption3Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ProchotOption0 (37h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotOption0MsbFlags: u8 {
        const ILIM2_VTH = 0b11111 << 3;
        const ICRIT_DEG = 0b11 << 1;
        const PROCHOT_VINDPM_80_90 = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption0MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ProchotOption0 (36h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotOption0Flags: u8 {
        const VSYS_TH1 = 0b111111 << 2;
        const INOM_DEG = 1 << 1;
        const LOWER_PROCHOT_VINDPM = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption0Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ProchotOption1 (39h) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotOption1MsbFlags: u8 {
        const IDCHG_TH1 = 0b111111 << 2;
        const IDCHG_DEG1 = 0b11;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption1MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ProchotOption1 (38h) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProchotOption1Flags: u8 {
        const PP_VINDPM = 1 << 7;
        const PP_COMP = 1 << 6;
        const PP_ICRIT = 1 << 5;
        const PP_INOM = 1 << 4;
        const PP_IDCHG1 = 1 << 3;
        const PP_VSYS = 1 << 2;
        const PP_BATPRES = 1 << 1;
        const PP_ACOK = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProchotOption1Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ADCOption (3Bh) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AdcOptionMsbFlags: u8 {
        const ADC_CONV = 1 << 7;
        const ADC_START = 1 << 6;
        const ADC_FULLSCALE = 1 << 5;
        // Bit D0 of ADCOption MSB (0x3B) is RAC_RSR according to some datasheets.
        // However, typical TI datasheets show ADCOption MSB (0x3B) as D7:ADC_CONV, D6:ADC_START, D5:ADC_FULLSCALE, D4-D0: Reserved or other functions.
        // Let's assume RAC_RSR is in ADCOption LSB (0x3A) for now if not found here, or needs clarification.
        // For BQ25730, RAC_RSR is indeed bit 0 of ADCOption MSB (register 0x3B).
        const RAC_RSR = 1 << 0; // Sense resistor selection for input current path (RAC)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcOptionMsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// ADCOption (3Ah) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AdcOptionFlags: u8 {
        const EN_ADC_CMPIN = 1 << 7;
        const EN_ADC_VBUS = 1 << 6;
        const EN_ADC_PSYS = 1 << 5;
        const EN_ADC_IIN = 1 << 4;
        const EN_ADC_IDCHG = 1 << 3;
        const EN_ADC_ICHG = 1 << 2;
        const EN_ADC_VSYS = 1 << 1;
        const EN_ADC_VBAT = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for AdcOptionFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

/// Dither Setting for ChargeOption4 (REG0x3D[4:3])
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DitherSetting {
    Disabled = 0b00,
    Dither1X = 0b01, // +/- 2%
    Dither2X = 0b10, // +/- 4%
    Dither3X = 0b11, // +/- 6%
}

impl DitherSetting {
    pub fn from_bits(bits: u8) -> Self {
        match (bits >> 3) & 0b11 {
            // EN_DITHER is bits 4:3
            0b01 => DitherSetting::Dither1X,
            0b10 => DitherSetting::Dither2X,
            0b11 => DitherSetting::Dither3X,
            _ => DitherSetting::Disabled, // Default or 00b
        }
    }
    pub fn bits(self) -> u8 {
        (self as u8) << 3
    }
}

bitflags! {
    /// ChargeOption4 (3Dh) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption4MsbFlags: u8 {
        const VSYS_UVP = 0b111 << 5;
        // EN_DITHER (bits 4:3) are handled by DitherSetting enum
        const VSYS_UVP_NO_HICCUP = 1 << 2;
        const PP_VBUS_VAP = 1 << 1;
        const STAT_VBUS_VAP = 1 << 0;

        /// Mask for the EN_DITHER bits.
        const EN_DITHER_MASK = 0b11 << 3;
    }
}

impl ChargeOption4MsbFlags {
    /// Sets the EN_DITHER bits.
    pub fn set_en_dither(&mut self, setting: DitherSetting) {
        self.remove(Self::EN_DITHER_MASK);
        let new_setting_flags = Self::from_bits_retain(setting.bits());
        self.insert(new_setting_flags);
    }

    /// Gets the EN_DITHER setting.
    pub fn get_en_dither(&self) -> DitherSetting {
        DitherSetting::from_bits(self.bits())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption4MsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        let flags_part = self.bits() & !ChargeOption4MsbFlags::EN_DITHER_MASK.bits();
        let dither_part = self.get_en_dither();
        defmt::write!(
            fmt,
            "Flags({=u8:b}) EN_DITHER({:?})",
            flags_part,
            dither_part
        );
    }
}

bitflags! {
    /// ChargeOption4 (3Ch) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ChargeOption4Flags: u8 {
        const IDCHG_DEG2 = 0b11 << 6;
        const IDCHG_TH2 = 0b111 << 3;
        const PP_IDCHG2 = 1 << 2;
        const STAT_IDCHG2 = 1 << 1;
        const STAT_PTM = 1 << 0;
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChargeOption4Flags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{=u8:b}", self.bits());
    }
}

bitflags! {
    /// VminActiveProtection (3Fh) MSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VminActiveProtectionMsbFlags: u8 {
        // VBUS_VAP_TH bits 7:1 (0xFE)
        const VBUS_VAP_TH_MASK = 0b11111110;
        // Bit 0 is reserved
    }
}

impl VminActiveProtectionMsbFlags {
    /// Sets the VBUS_VAP_TH value (7 bits).
    /// The raw_value should be the 7-bit value for VBUS_VAP_TH.
    pub fn set_vbus_vap_th(&mut self, raw_value: u8) {
        let mut new_bits = self.bits();
        new_bits &= !VminActiveProtectionMsbFlags::VBUS_VAP_TH_MASK.bits(); // Clear current VBUS_VAP_TH bits
        new_bits |= (raw_value << 1) & VminActiveProtectionMsbFlags::VBUS_VAP_TH_MASK.bits(); // Set new bits (shifted to D7-D1)
        *self = VminActiveProtectionMsbFlags::from_bits_truncate(new_bits);
    }

    /// Gets the VBUS_VAP_TH value (7 bits).
    pub fn get_vbus_vap_th(&self) -> u8 {
        (self.bits() & Self::VBUS_VAP_TH_MASK.bits()) >> 1
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VminActiveProtectionMsbFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "VBUS_VAP_TH: {=u8}", self.get_vbus_vap_th());
    }
}

bitflags! {
    /// VminActiveProtection (3Eh) LSB bit masks
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VminActiveProtectionFlags: u8 {
        // VSYS_TH2 bits 7:2 (0xFC)
        const VSYS_TH2_MASK = 0b11111100;
        const EN_VSYSTH2_FOLLOW_VSYSTH1 = 1 << 1;
        const EN_FRS = 1 << 0;
    }
}

impl VminActiveProtectionFlags {
    /// Sets the VSYS_TH2 value (6 bits).
    /// The raw_value should be the 6-bit value for VSYS_TH2.
    pub fn set_vsys_th2(&mut self, raw_value: u8) {
        let mut new_bits = self.bits();
        new_bits &= !VminActiveProtectionFlags::VSYS_TH2_MASK.bits(); // Clear current VSYS_TH2 bits
        new_bits |= (raw_value << 2) & VminActiveProtectionFlags::VSYS_TH2_MASK.bits(); // Set new bits (shifted to D7-D2)
        *self = VminActiveProtectionFlags::from_bits_truncate(new_bits);
    }

    /// Gets the VSYS_TH2 value (6 bits).
    pub fn get_vsys_th2(&self) -> u8 {
        (self.bits() & Self::VSYS_TH2_MASK.bits()) >> 2
    }

    pub fn set_en_vsysth2_follow_vsysth1(&mut self, enable: bool) {
        if enable {
            self.insert(VminActiveProtectionFlags::EN_VSYSTH2_FOLLOW_VSYSTH1);
        } else {
            self.remove(VminActiveProtectionFlags::EN_VSYSTH2_FOLLOW_VSYSTH1);
        }
    }

    pub fn get_en_vsysth2_follow_vsysth1(&self) -> bool {
        self.contains(VminActiveProtectionFlags::EN_VSYSTH2_FOLLOW_VSYSTH1)
    }

    pub fn set_en_frs(&mut self, enable: bool) {
        if enable {
            self.insert(VminActiveProtectionFlags::EN_FRS);
        } else {
            self.remove(VminActiveProtectionFlags::EN_FRS);
        }
    }

    pub fn get_en_frs(&self) -> bool {
        self.contains(VminActiveProtectionFlags::EN_FRS)
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for VminActiveProtectionFlags {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "VSYS_TH2: {=u8}, EN_VSYSTH2_FOLLOW_VSYSTH1: {}, EN_FRS: {}",
            self.get_vsys_th2(),
            self.get_en_vsysth2_follow_vsysth1(),
            self.get_en_frs()
        );
    }
}
