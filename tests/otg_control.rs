#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::{data_types::*, registers::Register, Error, BQ25730_I2C_ADDRESS};
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_otg_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGVoltage,
        &[0xC4, 0x09], // 5000mV (raw = 2500) - LSB, MSB
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_otg_voltage(OtgVoltage(5000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGVoltage,
        &[0xC4, 0x09], // 5000mV (raw = 2500)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_otg_voltage()?;
    assert_eq!(voltage.0, 5000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_otg_current() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x00], // 0mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_otg_current(OtgCurrent(0))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x0A], // 1000mA (raw = 10) - LSB, MSB
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_otg_current(OtgCurrent(1000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x7F], // 12700mA (raw = 127) - LSB, MSB
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_otg_current(OtgCurrent(12700))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_current() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x00], // 0mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 0);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x0A], // 1000mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 1000);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::OTGCurrent,
        &[0x00, 0x7F], // 12700mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 12700);
    charger.i2c.done();

    Ok(())
}
