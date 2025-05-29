#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{
    data_types::ChargeOption1, data_types::ChargeOption2, data_types::ChargeOption3,
    BQ25730_I2C_ADDRESS,
};
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_charge_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option1(ChargeOption1::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x3E], // Default LSB 0x00, MSB 0x3E (bit 0 is reserved, should be 0)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option1(ChargeOption1::from_u16(0x3E00))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption1 as u8],
        vec![0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
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
    let mut charger = new_bq25730_with_mock(&expectations);
    let options = charger.read_charge_option1()?;
    assert_eq!(options, ChargeOption1::from_u16(0x3F00));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_charge_option2() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption2,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option2(ChargeOption2::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption2,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option2(ChargeOption2::from_u16(0xFFFF))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option2() -> Result<(), Error<ErrorKind>> {
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8],
        vec![0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let options = charger.read_charge_option2()?;
    assert_eq!(options, ChargeOption2::from_u16(0x0000));
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption2 as u8],
        vec![0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let options = charger.read_charge_option2()?;
    assert_eq!(options, ChargeOption2::from_u16(0xFFFF));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_charge_option3() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption3,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option3(ChargeOption3::from_u16(0x0000))?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption3,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option3(ChargeOption3::from_u16(0xFFFF))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option3() -> Result<(), Error<ErrorKind>> {
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8],
        vec![0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let options = charger.read_charge_option3()?;
    assert_eq!(options, ChargeOption3::from_u16(0x0000));
    charger.i2c.done();

    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption3 as u8],
        vec![0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let options = charger.read_charge_option3()?;
    assert_eq!(options, ChargeOption3::from_u16(0xFFFF));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_enter_ship_mode() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        // Read ChargeOption1 (0x30/0x31) - assuming default 0x3F00
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8],
            ChargeOption1::from_u16(0x3F00)
                .to_u16()
                .to_le_bytes()
                .to_vec(), // Default LSB 0x00, MSB 0x3F
        ),
        // Write ChargeOption1 (0x30/0x31) with EN_SHIP_DCHG bit set (bit 1 of LSB)
        embedded_hal_mock::eh1::i2c::Transaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8, 0x02, 0x3E], // LSB (0x00 | EN_SHIP_DCHG), MSB (0x3E)
        ),
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.enter_ship_mode()?;
    charger.i2c.done();

    Ok(())
}
