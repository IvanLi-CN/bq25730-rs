#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use bq25730_async_rs::{data_types::*, registers::Register, Error};
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_otg_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGVoltage as u8, 0x88, 0x13], // 5000mV (raw = 625) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_otg_voltage(OtgVoltage(5000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_voltage() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGVoltage as u8],
        vec![0x88, 0x13], // 5000mV (raw = 625)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let voltage = charger.read_otg_voltage()?;
    assert_eq!(voltage.0, 5000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_otg_current() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8, 0x00, 0x00], // 0mA
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_otg_current(OtgCurrent(0))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8, 0x00, 0x0A], // 1000mA (raw = 10) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_otg_current(OtgCurrent(1000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8, 0x00, 0x7F], // 12700mA (raw = 127) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.set_otg_current(OtgCurrent(12700))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_current() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8],
        vec![0x00, 0x00], // 0mA
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 0);
    charger.i2c.done();

    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8],
        vec![0x00, 0x0A], // 1000mA
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 1000);
    charger.i2c.done();

    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrent as u8],
        vec![0x00, 0x7F], // 12700mA
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let current = charger.read_otg_current()?;
    assert_eq!(current.0, 12700);
    charger.i2c.done();

    Ok(())
}
