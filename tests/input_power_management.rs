#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::data_types::*;
use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_input_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0x00, 0x00], // 3200mV (offset)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_input_voltage(InputVoltage(3200))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0x01, 0x00], // 3264mV (raw = 1) - LSB, MSB
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_input_voltage(InputVoltage(3264))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0xFF, 0x00], // 19520mV (raw = 255) - LSB, MSB
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_input_voltage(InputVoltage(19520))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_input_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0x00, 0x00], // 3200mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3200);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0x01, 0x00], // 3264mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3264);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::InputVoltage,
        &[0xFF, 0x00], // 19520mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 19520);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_vsys_min() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0x0A, // 1000mV (raw = 10)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0x0A, // 1000mV (raw = 10)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0xE6, // 23000mV (raw = 230)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(23000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_vsys_min() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0x0A, // 1000mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0x0A, // 1000mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0xE6, // 23000mV
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 23000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_host() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x00, // 100mA (offset)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(100))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x01, // 200mA (raw = 1)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(200))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x63, // 10000mA (raw = 99)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(10000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_host() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x00, // 100mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x01, // 200mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        0x63, // 10000mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x00, // 100mA (offset)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(100))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x01, // 200mA (raw = 1)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(200))?;
    charger.i2c.done();

    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x63, // 10000mA (raw = 99)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(10000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x00, // 100mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x01, // 200mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        0x63, // 10000mA
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}
