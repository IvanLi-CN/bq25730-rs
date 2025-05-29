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
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0x00, 0x00],
        ), // 3200mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3200);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0x01, 0x00],
        ), // 3264mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3264);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0xFF, 0x00],
        ), // 19520mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 19520);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_vsys_min() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        &VsysMin(1000).to_u16().to_le_bytes(), // 1000mV (raw = 10)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        &VsysMin(1000).to_u16().to_le_bytes(), // 1000mV (raw = 10)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        &VsysMin(23000).to_u16().to_le_bytes(), // 23000mV (raw = 230)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vsys_min(VsysMin(23000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_vsys_min() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(1000).to_u16().to_le_bytes().to_vec(),
        ), // 1000mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(1000).to_u16().to_le_bytes().to_vec(),
        ), // 1000mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(23000).to_u16().to_le_bytes().to_vec(),
        ), // 23000mV
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 23000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_host() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        &IinHost(100).to_u16().to_le_bytes(), // 100mA (offset)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(100))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        &IinHost(200).to_u16().to_le_bytes(), // 200mA (raw = 1)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(200))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinHost,
        &IinHost(10000).to_u16().to_le_bytes(), // 10000mA (raw = 99)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_host(IinHost(10000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_host() -> Result<(), Error<ErrorKind>> {
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHost as u8],
        IinHost(100).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHost as u8],
        IinHost(200).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHost as u8],
        IinHost(10000).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        &IinDpm(100).to_u16().to_le_bytes(), // 100mA (offset)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(100))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        &IinDpm(200).to_u16().to_le_bytes(), // 200mA (raw = 1)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(200))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::IinDpm,
        &IinDpm(10000).to_u16().to_le_bytes(), // 10000mA (raw = 99)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_iin_dpm(IinDpm(10000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpm as u8],
        IinDpm(100).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpm as u8],
        IinDpm(200).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpm as u8],
        IinDpm(10000).to_u16().to_le_bytes().to_vec(),
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}
