#![allow(clippy::approx_constant)]

use embedded_hal::i2c::ErrorKind;

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{
    BQ25730_I2C_ADDRESS,
    data_types::{ChargeOption1, ChargeOption2, ChargeOption3, Config, SenseResistorValue}, // Updated imports
};

#[test]
fn test_set_charge_option1() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption1 as u8, 0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option1(ChargeOption1::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption1 as u8, 0x00, 0x3E], // Default LSB 0x00, MSB 0x3E (bit 0 is reserved, should be 0)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option1(ChargeOption1::from_u16(0x3E00))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option1() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption1 as u8],
        vec![0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option1()?;
    assert_eq!(options, ChargeOption1::from_u16(0x0000));
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8],
            vec![0x00, 0x3F],
        ), // Default LSB 0x00, MSB 0x3F
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option1()?;
    assert_eq!(options, ChargeOption1::from_u16(0x3F00));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_charge_option2() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8, 0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option2(ChargeOption2::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8, 0xFF, 0xFF],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option2(ChargeOption2::from_u16(0xFFFF))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option2() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8],
        vec![0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option2()?;
    assert_eq!(options, ChargeOption2::from_u16(0x0000));
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8],
        vec![0xFF, 0xFF],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option2()?;
    assert_eq!(options, ChargeOption2::from_u16(0xFFFF));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_charge_option3() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8, 0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option3(ChargeOption3::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8, 0xFF, 0xFF],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_charge_option3(ChargeOption3::from_u16(0xFFFF))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option3() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8],
        vec![0x00, 0x00],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option3()?;
    assert_eq!(options, ChargeOption3::from_u16(0x0000));
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8],
        vec![0xFF, 0xFF],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let options = charger.read_charge_option3()?;
    assert_eq!(options, ChargeOption3::from_u16(0xFFFF));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_enter_ship_mode() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [
        // Read ChargeOption1 (0x30/0x31) - assuming default 0x3F00
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8],
            ChargeOption1::from_u16(0x3F00) // Using default from data_types.rs
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ),
        // Write ChargeOption1 (0x30/0x31) with EN_SHIP_DCHG bit set (bit 1 of LSB)
        embedded_hal_mock::eh1::i2c::Transaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8, 0x02, 0x3E], // LSB (0x00 | EN_SHIP_DCHG), MSB (0x3E from default, due to truncate)
        ),
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.enter_ship_mode()?;
    charger.i2c.done();

    Ok(())
}
