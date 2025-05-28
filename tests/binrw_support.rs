#![cfg(feature = "binrw")]

use binrw::{BinRead, BinWrite};
use bq25730_async_rs::data_types::*;

// Helper function to test serialization and deserialization
fn test_binrw_roundtrip<T>(original_value: T)
where
    T: BinRead
        + BinWrite
        + PartialEq
        + std::fmt::Debug
        + binrw::meta::ReadEndian
        + binrw::meta::WriteEndian,
    for<'a> <T as BinRead>::Args<'a>: Default,
    for<'a> <T as BinWrite>::Args<'a>: Default,
{
    let mut buffer = std::io::Cursor::new(Vec::new());
    original_value.write(&mut buffer).unwrap();
    buffer.set_position(0); // Reset cursor position for reading
    let read_value = T::read(&mut buffer).unwrap();
    assert_eq!(original_value, read_value);
}

#[test]
fn test_charger_status_binrw() {
    let original = ChargerStatus {
        stat_ac: true,
        ico_done: false,
        in_vap: true,
        in_vindpm: false,
        in_iin_dpm: true,
        in_fchrg: false,
        in_pchrg: true,
        in_otg: false,
        fault_acov: true,
        fault_batoc: false,
        fault_acoc: true,
        fault_sysovp: false,
        fault_vsys_uvp: true,
        fault_force_converter_off: false,
        fault_otg_ovp: true,
        fault_otg_uvp: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_prochot_status_binrw() {
    let original = ProchotStatus {
        en_prochot_ext: true,
        prochot_width: 0b10, // Example value
        prochot_clear: false,
        stat_vap_fail: true,
        stat_exit_vap: false,
        stat_vindpm: true,
        stat_comp: false,
        stat_icrit: true,
        stat_inom: false,
        stat_idchg1: true,
        stat_vsys: false,
        stat_bat_removal: true,
        stat_adpt_removal: false,
        stat_idchg2: true,
        stat_ptm: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_current_binrw() {
    let original = ChargeCurrent(1024); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_voltage_binrw() {
    let original = ChargeVoltage(16384); // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_otg_voltage_binrw() {
    let original = OtgVoltage(5000); // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_otg_current_binrw() {
    let original = OtgCurrent(1500); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_input_voltage_binrw() {
    // InputVoltage has precision loss when converting from mV to register value and back.
    // Test with a value that aligns with the register LSB.
    // (12000 - 3200) / 64 = 137.5. So, use 137 * 64 + 3200 = 11968
    let original = InputVoltage(11968);
    test_binrw_roundtrip(original);
}

#[test]
fn test_vsys_min_binrw() {
    let original = VsysMin(3500); // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_iin_host_binrw() {
    let original = IinHost(2000); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_iin_dpm_binrw() {
    let original = IinDpm(2500); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_psys_binrw() {
    let original = AdcPsys(120); // Example value in mW
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_vbus_binrw() {
    // AdcVbus has precision loss. 5000 / 96 = 52.08. Use 52 * 96 = 4992
    let original = AdcVbus(4992);
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_idchg_binrw() {
    let original = AdcIdchg(1024); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_ichg_binrw() {
    let original = AdcIchg(512); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_cmpin_binrw() {
    let original = AdcCmpin(24); // Example value in mV
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_iin_binrw() {
    let original = AdcIin(1000); // Example value in mA
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_vbat_binrw() {
    // AdcVbat has precision loss. 4000 / 64 = 62.5. Use 62 * 64 = 3968
    let original = AdcVbat(3968);
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_vsys_binrw() {
    // AdcVsys has precision loss. 12000 / 64 = 187.5. Use 187 * 64 = 11968
    let original = AdcVsys(11968);
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_option_binrw() {
    let original = AdcOption {
        adc_conv: true,
        adc_start: false,
        adc_fullscale: true,
        en_adc_cmpin: false,
        en_adc_vbus: true,
        en_adc_psys: false,
        en_adc_iin: true,
        en_adc_idchg: false,
        en_adc_ichg: true,
        en_adc_vsys: false,
        en_adc_vbat: true,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option0_binrw() {
    let original = ChargeOption0 {
        en_ico_mode: true,
        en_acov: false,
        en_batoc: true,
        en_acoc: false,
        en_sysovp: true,
        en_vsys_uvp: false,
        en_force_converter_off: true,
        en_otg_ovp: false,
        en_otg_uvp: true,
        en_vap: false,
        en_vindpm: true,
        en_comp: false,
        en_icrit: true,
        en_inom: false,
        en_idchg1: true,
        en_vsys: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option1_binrw() {
    let original = ChargeOption1 {
        en_ptm: true,
        en_idchg2: false,
        en_bat_removal: true,
        en_adpt_removal: false,
        en_vsys_prochot: true,
        en_idchg1_prochot: false,
        en_inom_prochot: true,
        en_icrit_prochot: false,
        en_comp_prochot: true,
        en_vindpm_prochot: false,
        en_vap_prochot: true,
        en_otg_uvp_prochot: false,
        en_otg_ovp_prochot: true,
        en_force_converter_off_prochot: false,
        en_vsys_uvp_prochot: true,
        en_sysovp_prochot: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option2_binrw() {
    let original = ChargeOption2 {
        en_acoc_prochot: true,
        en_batoc_prochot: false,
        en_acov_prochot: true,
        en_ico_prochot: false,
        en_vindpm_prochot_2: true,
        en_comp_prochot_2: false,
        en_icrit_prochot_2: true,
        en_inom_prochot_2: false,
        en_idchg1_prochot_2: true,
        en_vsys_prochot_2: false,
        en_bat_removal_prochot_2: true,
        en_adpt_removal_prochot_2: false,
        en_idchg2_prochot_2: true,
        en_ptm_prochot_2: false,
        en_otg_ovp_prochot_2: true,
        en_otg_uvp_prochot_2: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_charge_option3_binrw() {
    let original = ChargeOption3 {
        en_acov_prochot_3: true,
        en_batoc_prochot_3: false,
        en_acoc_prochot_3: true,
        en_ico_prochot_3: false,
        en_vindpm_prochot_3: true,
        en_comp_prochot_3: false,
        en_icrit_prochot_3: true,
        en_inom_prochot_3: false,
        en_idchg1_prochot_3: true,
        en_vsys_prochot_3: false,
        en_bat_removal_prochot_3: true,
        en_adpt_removal_prochot_3: false,
        en_idchg2_prochot_3: true,
        en_ptm_prochot_3: false,
        en_otg_ovp_prochot_3: true,
        en_otg_uvp_prochot_3: false,
    };
    test_binrw_roundtrip(original);
}

#[test]
fn test_adc_measurements_binrw() {
    // AdcMeasurements is a composite type, its fields also have precision loss.
    // Use register-aligned values for its fields.
    let original = AdcMeasurements {
        psys: AdcPsys(120),    // 120 / 12 = 10. 10 * 12 = 120. No loss.
        vbus: AdcVbus(4992),   // 4992 / 96 = 52. 52 * 96 = 4992. No loss.
        idchg: AdcIdchg(1024), // 1024 / 512 = 2. 2 * 512 = 1024. No loss.
        ichg: AdcIchg(512),    // 512 / 128 = 4. 4 * 128 = 512. No loss.
        cmpin: AdcCmpin(24),   // 24 / 12 = 2. 2 * 12 = 24. No loss.
        iin: AdcIin(800),      // 800 / 100 = 8. 8 * 100 = 800. No loss.
        vbat: AdcVbat(3968),   // 3968 / 64 = 62. 62 * 64 = 3968. No loss.
        vsys: AdcVsys(11968),  // 11968 / 64 = 187. 187 * 64 = 11968. No loss.
    };
    test_binrw_roundtrip(original);
}
