#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{
    data_types::{Config, SenseResistorValue}, // Updated imports
    BQ25730_I2C_ADDRESS,
};
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_read_charger_status() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargerStatus as u8],
        vec![0x80, 0x80], // LSB: Fault ACOV, MSB: STAT_AC
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
    let status = charger.read_charger_status()?;
    assert!(status
        .status_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFlags::STAT_AC));
    assert!(status
        .fault_flags
        .contains(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_ACOV));
    charger.i2c.done();

    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargerStatus as u8],
        vec![0x00, 0x00], // All false
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
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
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ProchotStatus as u8],
        vec![0x01, 0x40], // LSB: STAT_ADPT_REMOVAL, MSB: EN_PROCHOT_EXT
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
    let status = charger.read_prochot_status()?;
    assert!(status
        .msb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusMsbFlags::EN_PROCHOT_EXT));
    assert!(status
        .lsb_flags
        .contains(bq25730_async_rs::registers::ProchotStatusFlags::STAT_ADPT_REMOVAL));
    charger.i2c.done();

    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ProchotStatus as u8],
        vec![0x00, 0x00], // All false
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
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
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_adc_measurements() -> Result<(), Error<ErrorKind>> {
    // For 5mOhm RsnsAc
    // Corrected Config::new call
    let config_5m_ohm_ac = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations_5m_ohm = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ADCPSYS as u8],
            vec![
                0x01, // ADCPSYS (raw=1) -> 12mV
                0x01, // ADCVBUS (raw=1) -> 96mV
                0x01, // ADCIDCHG (raw=1) -> 512mA (for default RsnsBat::R5mOhm)
                0x01, // ADCICHG (raw=1) -> 128mA (for default RsnsBat::R5mOhm)
                0x01, // ADCCMPIN (raw=1) -> 12mV
                0x01, // ADCIIN (raw=1) -> 100mA (for RsnsAc::R5mOhm)
                0x00, // ADCVBAT_LSB
                0x2E, // ADCVBAT_MSB (raw=46) -> 2880 (offset for 4S) + 46*64 = 5824mV
            ],
        ),
    ];
    let i2c_5m_ohm = I2cMock::new(&expectations_5m_ohm);
    let mut charger_5m_ohm = bq25730_async_rs::Bq25730::new(i2c_5m_ohm, BQ25730_I2C_ADDRESS, config_5m_ohm_ac);
    let measurements_5m_ohm = charger_5m_ohm.read_adc_measurements()?;
    assert_eq!(measurements_5m_ohm.psys.0, 12);
    assert_eq!(measurements_5m_ohm.vbus.0, 96);
    assert_eq!(measurements_5m_ohm.idchg.milliamps, 512);
    assert_eq!(measurements_5m_ohm.ichg.milliamps, 128);
    assert_eq!(measurements_5m_ohm.cmpin.0, 12);
    assert_eq!(measurements_5m_ohm.iin.milliamps, 100);
    assert_eq!(measurements_5m_ohm.vbat.0, 2880 + (46 * 64));
    assert_eq!(measurements_5m_ohm.vsys.0, 2880 + (46 * 64));
    charger_5m_ohm.i2c.done();

    // For 10mOhm RsnsAc
    // Corrected Config::new call
    let config_10m_ohm_ac = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R10mOhm);
    let expectations_10m_ohm = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ADCPSYS as u8],
            vec![
                0x01, // ADCPSYS (raw=1) -> 12mV
                0x01, // ADCVBUS (raw=1) -> 96mV
                0x01, // ADCIDCHG (raw=1) -> 256mA (for default RsnsBat::R5mOhm, this test should ideally use RsnsBat::R10mOhm for idchg/ichg if config implies it)
                0x01, // ADCICHG (raw=1) -> 64mA (for default RsnsBat::R5mOhm)
                0x01, // ADCCMPIN (raw=1) -> 12mV
                0x01, // ADCIIN (raw=1) -> 50mA (for RsnsAc::R10mOhm)
                0x00, // ADCVBAT_LSB
                0x2E, // ADCVBAT_MSB (raw=46) -> 5824mV
            ],
        ),
    ];
    let i2c_10m_ohm = I2cMock::new(&expectations_10m_ohm);
    let mut charger_10m_ohm = bq25730_async_rs::Bq25730::new(i2c_10m_ohm, BQ25730_I2C_ADDRESS, config_10m_ohm_ac);
    let measurements_10m_ohm = charger_10m_ohm.read_adc_measurements()?;
    assert_eq!(measurements_10m_ohm.psys.0, 12);
    assert_eq!(measurements_10m_ohm.vbus.0, 96);
    // The AdcIchg/AdcIdchg use RsnsBat::default() which is R5mOhm.
    // If the test intent was to use R10mOhm for battery path as well, the mock data or AdcIchg/Idchg creation needs adjustment.
    // For now, asserting against the RsnsBat::default() behavior.
    assert_eq!(measurements_10m_ohm.idchg.milliamps, 512); // Still 512mA due to RsnsBat::default()
    assert_eq!(measurements_10m_ohm.ichg.milliamps, 128);   // Still 128mA due to RsnsBat::default()
    assert_eq!(measurements_10m_ohm.cmpin.0, 12);
    assert_eq!(measurements_10m_ohm.iin.milliamps, 50);
    assert_eq!(measurements_10m_ohm.vbat.0, 2880 + (46 * 64));
    assert_eq!(measurements_10m_ohm.vsys.0, 2880 + (46 * 64));
    charger_10m_ohm.i2c.done();

    Ok(())
}
