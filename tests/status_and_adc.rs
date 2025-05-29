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
    assert!(status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::STAT_AC));
    assert!(status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_ACOV));
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargerStatus,
        &[0x00, 0x00], // All false
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_charger_status()?;
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::STAT_AC));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::ICO_DONE));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_VAP));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_VINDPM));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_IIN_DPM));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_FCHRG));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_PCHRG));
    assert!(!status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::IN_OTG));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_ACOV));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_BATOC));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_ACOC));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_SYSOVP));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_VSYS_UVP));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_FORCE_CONVERTER_OFF));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_OTG_OVP));
    assert!(!status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_OTG_UVP));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_prochot_status() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotStatus,
        &[0x01, 0x40], // LSB: STAT_ADPT_REMOVAL, MSB: EN_PROCHOT_EXT
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_prochot_status()?;
    assert!(status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::EN_PROCHOT_EXT));
    assert!(status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_ADPT_REMOVAL));
    // assert_eq!(status.stat_idchg2, true); // Removed as ChargeOption4 is gone
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotStatus,
        &[0x00, 0x00], // All false
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let status = charger.read_prochot_status()?;
    assert!(!status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::EN_PROCHOT_EXT));
    assert_eq!(status.prochot_width, 0);
    assert!(!status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::PROCHOT_CLEAR));
    assert!(!status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::STAT_VAP_FAIL));
    assert!(!status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::STAT_EXIT_VAP));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_VINDPM));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_COMP));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_ICRIT));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_INOM));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_IDCHG1));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_VSYS));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_BAT_REMOVAL));
    assert!(!status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_ADPT_REMOVAL));
    // assert_eq!(status.stat_idchg2, false); // Removed as ChargeOption4 is gone
    // assert_eq!(status.stat_ptm, false); // Removed as ChargeOption4 is gone
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_adc_measurements() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCPSYS as u8],
            vec![0x01],
        ), // 12mV (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCVBUS as u8],
            vec![0x01],
        ), // 96mV (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCIDCHG as u8],
            vec![0x01],
        ), // 512mA (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCICHG as u8],
            vec![0x01],
        ), // 128mA (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCCMPIN as u8],
            vec![0x01],
        ), // 12mV (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCIIN as u8],
            vec![0x01],
        ), // 100mA (raw = 1)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCVBAT as u8],
            vec![0x00, 0x01],
        ), // 2944mV (raw = 46)
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            bq25730_async_rs::BQ25730_I2C_ADDRESS,
            vec![Register::ADCVSYS as u8],
            vec![0x00, 0x01],
        ), // 64mV (raw = 1)
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let measurements = charger.read_adc_measurements()?;
    assert_eq!(measurements.psys.0, 12); // PSYS is in mV, 12mV/LSB
    assert_eq!(measurements.vbus.0, 96);
    assert_eq!(measurements.idchg.0, 512);
    assert_eq!(measurements.ichg.0, 128);
    assert_eq!(measurements.cmpin.0, 12);
    assert_eq!(measurements.iin.milliamps, 50);
    assert_eq!(measurements.vbat.0, 2880 + 64);
    assert_eq!(measurements.vsys.0, 2880 + 64); // Corrected expected value based on LSB_MV and offset_mv
    charger.i2c.done();

    Ok(())
}
