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
    /// ADCVSYS Register
    ADCVSYS = 0x2C,
    /// ADCVBAT Register
    ADCVBAT = 0x2D,
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
    /// Ship Mode Register
    ShipMode = 0x40,
    /// Shutdown Control Register
    ShutdownControl = 0x18, // Found in section 8.3.1.14
}

// ChargeOption0 (01/00h) bit masks
pub const CHARGE_OPTION0_MSB_EN_LWPWR: u8 = 1 << 7; // EN_LWPWR is in MSB (0x01)
pub const CHARGE_OPTION0_MSB_WDTMR_ADJ: u8 = 0b11 << 5;
pub const CHARGE_OPTION0_MSB_IIN_DPM_AUTO_DISABLE: u8 = 1 << 4;
pub const CHARGE_OPTION0_MSB_OTG_ON_CHRGOK: u8 = 1 << 3;
pub const CHARGE_OPTION0_MSB_EN_OOA: u8 = 1 << 2;
pub const CHARGE_OPTION0_MSB_PWM_FREQ: u8 = 1 << 1;
pub const CHARGE_OPTION0_MSB_LOW_PTM_RIPPLE: u8 = 1 << 0;

pub const CHARGE_OPTION0_EN_CMP_LATCH: u8 = 1 << 7; // EN_CMP_LATCH is in LSB (0x00)
pub const CHARGE_OPTION0_VSYS_UVP_ENZ: u8 = 1 << 6;
pub const CHARGE_OPTION0_EN_LEARN: u8 = 1 << 5;
pub const CHARGE_OPTION0_IADPT_GAIN: u8 = 1 << 4;
pub const CHARGE_OPTION0_IBAT_GAIN: u8 = 1 << 3;
pub const CHARGE_OPTION0_EN_LDO: u8 = 1 << 2;
pub const CHARGE_OPTION0_EN_IIN_DPM: u8 = 1 << 1;
pub const CHARGE_OPTION0_CHRG_INHIBIT: u8 = 1 << 0;

// ChargerStatus (21/20h) bit masks
pub const CHARGER_STATUS_STAT_AC: u8 = 1 << 7;
pub const CHARGER_STATUS_ICO_DONE: u8 = 1 << 6;
pub const CHARGER_STATUS_IN_VAP: u8 = 1 << 5;
pub const CHARGER_STATUS_IN_VINDPM: u8 = 1 << 4;
pub const CHARGER_STATUS_IN_IIN_DPM: u8 = 1 << 3;
pub const CHARGER_STATUS_IN_FCHRG: u8 = 1 << 2;
pub const CHARGER_STATUS_IN_PCHRG: u8 = 1 << 1;
pub const CHARGER_STATUS_IN_OTG: u8 = 1 << 0;

pub const CHARGER_STATUS_FAULT_ACOV: u8 = 1 << 7;
pub const CHARGER_STATUS_FAULT_BATOC: u8 = 1 << 6;
pub const CHARGER_STATUS_FAULT_ACOC: u8 = 1 << 5;
pub const CHARGER_STATUS_FAULT_SYSOVP: u8 = 1 << 4;
pub const CHARGER_STATUS_FAULT_VSYS_UVP: u8 = 1 << 3;
pub const CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF: u8 = 1 << 2;
pub const CHARGER_STATUS_FAULT_OTG_OVP: u8 = 1 << 1;
pub const CHARGER_STATUS_FAULT_OTG_UVP: u8 = 1 << 0;

// ProchotStatus (23/22h) bit masks
pub const PROCHOT_STATUS_EN_PROCHOT_EXT: u8 = 1 << 6;
pub const PROCHOT_STATUS_PROCHOT_WIDTH: u8 = 0b11 << 4;
pub const PROCHOT_STATUS_PROCHOT_CLEAR: u8 = 1 << 3;
pub const PROCHOT_STATUS_STAT_VAP_FAIL: u8 = 1 << 1;
pub const PROCHOT_STATUS_STAT_EXIT_VAP: u8 = 1 << 0;

pub const PROCHOT_STATUS_STAT_VINDPM: u8 = 1 << 7;
pub const PROCHOT_STATUS_STAT_COMP: u8 = 1 << 6;
pub const PROCHOT_STATUS_STAT_ICRIT: u8 = 1 << 5;
pub const PROCHOT_STATUS_STAT_INOM: u8 = 1 << 4;
pub const PROCHOT_STATUS_STAT_IDCHG1: u8 = 1 << 3;
pub const PROCHOT_STATUS_STAT_VSYS: u8 = 1 << 2;
pub const PROCHOT_STATUS_STAT_BAT_REMOVAL: u8 = 1 << 1;
pub const PROCHOT_STATUS_STAT_ADPT_REMOVAL: u8 = 1 << 0;

// ChargeOption1 (31/30h) bit masks
pub const CHARGE_OPTION1_EN_IBAT: u8 = 1 << 7;
pub const CHARGE_OPTION1_EN_PROCHOT_LPWR: u8 = 1 << 6;
pub const CHARGE_OPTION1_PSYS_CONFIG: u8 = 0b11 << 4;
pub const CHARGE_OPTION1_RSNS_RAC: u8 = 1 << 3;
pub const CHARGE_OPTION1_RSNS_RSR: u8 = 1 << 2;
pub const CHARGE_OPTION1_PSYS_RATIO: u8 = 1 << 1;

pub const CHARGE_OPTION1_CMP_REF: u8 = 1 << 7;
pub const CHARGE_OPTION1_CMP_POL: u8 = 1 << 6;
pub const CHARGE_OPTION1_CMP_DEG: u8 = 0b11 << 4;
pub const CHARGE_OPTION1_FORCE_CONV_OFF: u8 = 1 << 3;
pub const CHARGE_OPTION1_EN_PTM: u8 = 1 << 2;
pub const CHARGE_OPTION1_EN_SHIP_DCHG: u8 = 1 << 1; // EN_SHIP_DCHG is in LSB (0x30)
pub const CHARGE_OPTION1_AUTO_WAKEUP_EN: u8 = 1 << 0;

// ChargeOption2 (33/32h) bit masks
pub const CHARGE_OPTION2_PKPWR_TOVLD_DEG: u8 = 0b11 << 6;
pub const CHARGE_OPTION2_EN_PKPWR_IIN_DPM: u8 = 1 << 5;
pub const CHARGE_OPTION2_EN_PKPWR_VSYS: u8 = 1 << 4;
pub const CHARGE_OPTION2_STAT_PKPWR_OVLD: u8 = 1 << 3;
pub const CHARGE_OPTION2_STAT_PKPWR_RELAX: u8 = 1 << 2;
pub const CHARGE_OPTION2_PKPWR_TMAX: u8 = 0b11;

pub const CHARGE_OPTION2_EN_EXTILIM: u8 = 1 << 7;
pub const CHARGE_OPTION2_EN_ICHG_IDCHG: u8 = 1 << 6;
pub const CHARGE_OPTION2_Q2_OCP: u8 = 1 << 5;
pub const CHARGE_OPTION2_ACX_OCP: u8 = 1 << 4;
pub const CHARGE_OPTION2_EN_ACOC: u8 = 1 << 3;
pub const CHARGE_OPTION2_ACOC_VTH: u8 = 1 << 2;
pub const CHARGE_OPTION2_EN_BATOC: u8 = 1 << 1;
pub const CHARGE_OPTION2_BATOC_VTH: u8 = 1 << 0;

// ChargeOption3 (35/34h) bit masks
pub const CHARGE_OPTION3_EN_HIZ: u8 = 1 << 7;
pub const CHARGE_OPTION3_RESET_REG: u8 = 1 << 6;
pub const CHARGE_OPTION3_RESET_VINDPM: u8 = 1 << 5;
pub const CHARGE_OPTION3_EN_OTG: u8 = 1 << 4;
pub const CHARGE_OPTION3_EN_ICO_MODE: u8 = 1 << 3;
pub const CHARGE_OPTION3_EN_PORT_CTRL: u8 = 1 << 2;
pub const CHARGE_OPTION3_EN_VSYS_MIN_SOFT_SR: u8 = 1 << 1;
pub const CHARGE_OPTION3_EN_OTG_BIGCAP: u8 = 1 << 0;

pub const CHARGE_OPTION3_BATFET_ENZ: u8 = 1 << 7;
pub const CHARGE_OPTION3_EN_VBUS_VAP: u8 = 1 << 6;
pub const CHARGE_OPTION3_OTG_VAP_MODE: u8 = 1 << 5;
pub const CHARGE_OPTION3_IL_AVG: u8 = 0b11 << 3;
pub const CHARGE_OPTION3_CMP_EN: u8 = 1 << 2;
pub const CHARGE_OPTION3_BATFETOFF_HIZ: u8 = 1 << 1;
pub const CHARGE_OPTION3_PSYS_OTG_IDCHG: u8 = 1 << 0;

// ProchotOption0 (37/36h) bit masks
pub const PROCHOT_OPTION0_ILIM2_VTH: u8 = 0b11111 << 3;
pub const PROCHOT_OPTION0_ICRIT_DEG: u8 = 0b11 << 1;
pub const PROCHOT_OPTION0_PROCHOT_VINDPM_80_90: u8 = 1 << 0;

pub const PROCHOT_OPTION0_VSYS_TH1: u8 = 0b111111 << 2;
pub const PROCHOT_OPTION0_INOM_DEG: u8 = 1 << 1;
pub const PROCHOT_OPTION0_LOWER_PROCHOT_VINDPM: u8 = 1 << 0;

// ProchotOption1 (39/38h) bit masks
pub const PROCHOT_OPTION1_IDCHG_TH1: u8 = 0b111111 << 2;
pub const PROCHOT_OPTION1_IDCHG_DEG1: u8 = 0b11;

pub const PROCHOT_OPTION1_PP_VINDPM: u8 = 1 << 7;
pub const PROCHOT_OPTION1_PP_COMP: u8 = 1 << 6;
pub const PROCHOT_OPTION1_PP_ICRIT: u8 = 1 << 5;
pub const PROCHOT_OPTION1_PP_INOM: u8 = 1 << 4;
pub const PROCHOT_OPTION1_PP_IDCHG1: u8 = 1 << 3;
pub const PROCHOT_OPTION1_PP_VSYS: u8 = 1 << 2;
pub const PROCHOT_OPTION1_PP_BATPRES: u8 = 1 << 1;
pub const PROCHOT_OPTION1_PP_ACOK: u8 = 1 << 0;

// ADCOption (3B/3Ah) bit masks
pub const ADC_OPTION_ADC_CONV: u8 = 1 << 7;
pub const ADC_OPTION_ADC_START: u8 = 1 << 6;
pub const ADC_OPTION_ADC_FULLSCALE: u8 = 1 << 5;

pub const ADC_OPTION_EN_ADC_CMPIN: u8 = 1 << 7;
pub const ADC_OPTION_EN_ADC_VBUS: u8 = 1 << 6;
pub const ADC_OPTION_EN_ADC_PSYS: u8 = 1 << 5;
pub const ADC_OPTION_EN_ADC_IIN: u8 = 1 << 4;
pub const ADC_OPTION_EN_ADC_IDCHG: u8 = 1 << 3;
pub const ADC_OPTION_EN_ADC_ICHG: u8 = 1 << 2;
pub const ADC_OPTION_EN_ADC_VSYS: u8 = 1 << 1;
pub const ADC_OPTION_EN_ADC_VBAT: u8 = 1 << 0;

// ChargeOption4 (3D/3Ch) bit masks
pub const CHARGE_OPTION4_VSYS_UVP: u8 = 0b111 << 5;
pub const CHARGE_OPTION4_EN_DITHER: u8 = 0b11 << 3;
pub const CHARGE_OPTION4_VSYS_UVP_NO_HICCUP: u8 = 1 << 2;
pub const CHARGE_OPTION4_PP_VBUS_VAP: u8 = 1 << 1;
pub const CHARGE_OPTION4_STAT_VBUS_VAP: u8 = 1 << 0;

pub const CHARGE_OPTION4_IDCHG_DEG2: u8 = 0b11 << 6;
pub const CHARGE_OPTION4_IDCHG_TH2: u8 = 0b111 << 3;
pub const CHARGE_OPTION4_PP_IDCHG2: u8 = 1 << 2;
pub const CHARGE_OPTION4_STAT_IDCHG2: u8 = 1 << 1;
pub const CHARGE_OPTION4_STAT_PTM: u8 = 1 << 0;

// VMIN Active Protection (3F/3Eh) bit masks
pub const VMIN_ACTIVE_PROTECTION_VBUS_VAP_TH: u8 = 0b1111111 << 1;

pub const VMIN_ACTIVE_PROTECTION_VSYS_TH2: u8 = 0b111111 << 2;
pub const VMIN_ACTIVE_PROTECTION_EN_VSYSTH2_FOLLOW_VSYSTH1: u8 = 1 << 1;
pub const VMIN_ACTIVE_PROTECTION_EN_FRS: u8 = 1 << 0;

// Shutdown Control (18h) bit masks
// No specific bit masks defined in datasheet for this register, only value 0x10 for ship mode.
