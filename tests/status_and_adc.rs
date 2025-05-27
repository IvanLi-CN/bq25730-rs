#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_read_charger_status() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargerStatus,
        &[0x80, 0x80], // LSB: Fault ACOV, MSB: STAT_AC
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_charger_status()?;
    assert_eq!(status.stat_ac, true);
    assert_eq!(status.fault_acov, true);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargerStatus,
        &[0x00, 0x00], // All false
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_charger_status()?;
    assert_eq!(status.stat_ac, false);
    assert_eq!(status.ico_done, false);
    assert_eq!(status.in_vap, false);
    assert_eq!(status.in_vindpm, false);
    assert_eq!(status.in_iin_dpm, false);
    assert_eq!(status.in_fchrg, false);
    assert_eq!(status.in_pchrg, false);
    assert_eq!(status.in_otg, false);
    assert_eq!(status.fault_acov, false);
    assert_eq!(status.fault_batoc, false);
    assert_eq!(status.fault_acoc, false);
    assert_eq!(status.fault_sysovp, false);
    assert_eq!(status.fault_vsys_uvp, false);
    assert_eq!(status.fault_force_converter_off, false);
    assert_eq!(status.fault_otg_ovp, false);
    assert_eq!(status.fault_otg_uvp, false);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_prochot_status() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        read_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ProchotStatus,
            &[0x01, 0x40], // LSB: STAT_ADPT_REMOVAL, MSB: EN_PROCHOT_EXT
        ),
        read_register_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption4,
            0x02, // STAT_IDCHG2
        ),
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_prochot_status()?;
    assert_eq!(status.en_prochot_ext, true);
    assert_eq!(status.stat_adpt_removal, true);
    assert_eq!(status.stat_idchg2, true);
    charger.i2c.done();

    let expectations = [
        read_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ProchotStatus,
            &[0x00, 0x00], // All false
        ),
        read_register_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption4,
            0x00, // All false
        ),
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_prochot_status()?;
    assert_eq!(status.en_prochot_ext, false);
    assert_eq!(status.prochot_width, 0);
    assert_eq!(status.prochot_clear, false);
    assert_eq!(status.stat_vap_fail, false);
    assert_eq!(status.stat_exit_vap, false);
    assert_eq!(status.stat_vindpm, false);
    assert_eq!(status.stat_comp, false);
    assert_eq!(status.stat_icrit, false);
    assert_eq!(status.stat_inom, false);
    assert_eq!(status.stat_idchg1, false);
    assert_eq!(status.stat_vsys, false);
    assert_eq!(status.stat_bat_removal, false);
    assert_eq!(status.stat_adpt_removal, false);
    assert_eq!(status.stat_idchg2, false);
    assert_eq!(status.stat_ptm, false);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_adc_measurements() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCPSYS,
            0x01,
        ), // 12mV (raw = 1)
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCVBUS,
            0x01,
        ), // 96mV
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCIDCHG,
            0x01,
        ), // 512mA
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCICHG,
            0x01,
        ), // 128mA
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCCMPIN,
            0x01,
        ), // 12mV
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCIIN,
            0x01,
        ), // 100mA
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCVBAT,
            0x01,
        ), // 64mV
        read_register_transaction(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            Register::ADCVSYS,
            0x01,
        ), // 64mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let measurements = charger.read_adc_measurements()?;
    assert_eq!(measurements.psys.0, 12); // PSYS is in mV, 12mV/LSB
    assert_eq!(measurements.vbus.0, 96);
    assert_eq!(measurements.idchg.0, 512);
    assert_eq!(measurements.ichg.0, 128);
    assert_eq!(measurements.cmpin.0, 12);
    assert_eq!(measurements.iin.0, 100);
    assert_eq!(measurements.vbat.0, 64);
    assert_eq!(measurements.vsys.0, 64);
    charger.i2c.done();

    Ok(())
}
