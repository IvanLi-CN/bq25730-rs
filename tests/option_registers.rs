#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_charge_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option1(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x3F], // Default LSB 0x00, MSB 0x3F
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option1(0x00, 0x3F)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option1()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption1,
        &[0x00, 0x3F], // Default LSB 0x00, MSB 0x3F
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option1()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x3F);
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
    charger.set_charge_option2(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption2,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option2(0xFF, 0xFF)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option2() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption2,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option2()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption2,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option2()?;
    assert_eq!(lsb, 0xFF);
    assert_eq!(msb, 0xFF);
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
    charger.set_charge_option3(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption3,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option3(0xFF, 0xFF)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option3() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption3,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option3()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption3,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_charge_option3()?;
    assert_eq!(lsb, 0xFF);
    assert_eq!(msb, 0xFF);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_charge_option4() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption4,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option4(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption4,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_charge_option4(0xFF, 0xFF)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_charge_option4() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption4,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let value = charger.read_charge_option4()?;
    assert_eq!(value, (0x00, 0x00));
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption4,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let value = charger.read_charge_option4()?;
    assert_eq!(value, (0xFF, 0xFF));
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_prochot_option0() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption0,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_prochot_option0(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption0,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_prochot_option0(0xFF, 0xFF)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_prochot_option0() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption0,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_prochot_option0()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption0,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_prochot_option0()?;
    assert_eq!(lsb, 0xFF);
    assert_eq!(msb, 0xFF);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_prochot_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption1,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_prochot_option1(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption1,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_prochot_option1(0xFF, 0xFF)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_prochot_option1() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption1,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_prochot_option1()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ProchotOption1,
        &[0xFF, 0xFF],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_prochot_option1()?;
    assert_eq!(lsb, 0xFF);
    assert_eq!(msb, 0xFF);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_adc_option() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ADCOption,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_adc_option(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ADCOption,
        &[0x00, 0x20], // Default LSB 0x00, MSB 0x20 (ADC_FULLSCALE=1b)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_adc_option(0x00, 0x20)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_adc_option() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ADCOption,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_adc_option()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ADCOption,
        &[0x00, 0x20], // Default LSB 0x00, MSB 0x20 (ADC_FULLSCALE=1b)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_adc_option()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x20);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_vmin_active_protection() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VMINActiveProtection,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vmin_active_protection(0x00, 0x00)?;
    charger.i2c.done();

    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VMINActiveProtection,
        &[0x04, 0x00], // Default for 1S battery (LSB 0x04, MSB 0x00)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.set_vmin_active_protection(0x04, 0x00)?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_vmin_active_protection() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VMINActiveProtection,
        &[0x00, 0x00],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_vmin_active_protection()?;
    assert_eq!(lsb, 0x00);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VMINActiveProtection,
        &[0x04, 0x00], // Default for 1S battery (LSB 0x04, MSB 0x00)
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let (lsb, msb) = charger.read_vmin_active_protection()?;
    assert_eq!(lsb, 0x04);
    assert_eq!(msb, 0x00);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_enter_ship_mode() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        // Read ChargeOption1 (0x30/0x31) - assuming default 0x3F00
        // Read ChargeOption1 (0x30/0x31) - assuming default 0x3F00
        read_registers_transaction(BQ25730_I2C_ADDRESS, Register::ChargeOption1, &[0x00, 0x3F]), // Default LSB 0x00, MSB 0x3F
        // Write ChargeOption1 (0x30/0x31) with EN_SHIP_DCHG bit set (bit 1 of LSB)
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption1,
            &[0x02, 0x3F], // LSB (0x00 | EN_SHIP_DCHG), MSB (0x3F)
        ),
    ];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.enter_ship_mode()?;
    charger.i2c.done();

    Ok(())
}
