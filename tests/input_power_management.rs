#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::data_types::*;
use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_input_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0x00, 0x00], // 3200mV (offset)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_input_voltage(InputVoltage(3200))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0x01, 0x00], // 3264mV (raw = 1) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_input_voltage(InputVoltage(3264))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0xFF, 0x00], // 19520mV (raw = 255) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 19520);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_vsys_min() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(1000).to_u16().to_le_bytes()[0],
            VsysMin(1000).to_u16().to_le_bytes()[1],
        ], // 1000mV (raw = 10)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(1000).to_u16().to_le_bytes()[0],
            VsysMin(1000).to_u16().to_le_bytes()[1],
        ], // 1000mV (raw = 10)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(23000).to_u16().to_le_bytes()[0],
            VsysMin(23000).to_u16().to_le_bytes()[1],
        ], // 23000mV (raw = 230)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 23000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_host() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinHost as u8,
            IinHost(100).to_u16().to_le_bytes()[0],
            IinHost(100).to_u16().to_le_bytes()[1],
        ], // 100mA (offset)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_iin_host(IinHost(100))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinHost as u8,
            IinHost(200).to_u16().to_le_bytes()[0],
            IinHost(200).to_u16().to_le_bytes()[1],
        ], // 200mA (raw = 1)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_iin_host(IinHost(200))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinHost as u8,
            IinHost(10000).to_u16().to_le_bytes()[0],
            IinHost(10000).to_u16().to_le_bytes()[1],
        ], // 10000mA (raw = 99)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHost as u8],
        IinHost(200).to_u16().to_le_bytes().to_vec(),
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHost as u8],
        IinHost(10000).to_u16().to_le_bytes().to_vec(),
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_host()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinDpm as u8,
            IinDpm(100).to_u16().to_le_bytes()[0],
            IinDpm(100).to_u16().to_le_bytes()[1],
        ], // 100mA (offset)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_iin_dpm(IinDpm(100))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinDpm as u8,
            IinDpm(200).to_u16().to_le_bytes()[0],
            IinDpm(200).to_u16().to_le_bytes()[1],
        ], // 200mA (raw = 1)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_iin_dpm(IinDpm(200))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::IinDpm as u8,
            IinDpm(10000).to_u16().to_le_bytes()[0],
            IinDpm(10000).to_u16().to_le_bytes()[1],
        ], // 10000mA (raw = 99)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 100);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpm as u8],
        IinDpm(200).to_u16().to_le_bytes().to_vec(),
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 200);
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpm as u8],
        IinDpm(10000).to_u16().to_le_bytes().to_vec(),
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_iin_dpm()?;
    assert_eq!(current.0, 10000);
    charger.i2c.done();

    Ok(())
}
