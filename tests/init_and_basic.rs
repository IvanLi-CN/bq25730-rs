#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::{BQ25730_I2C_ADDRESS, RegisterAccess};
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::errors::Error;
use embedded_hal::i2c::ErrorKind;


#[test]
fn test_new() {
    let expectations = [];
    let mut charger = new_bq25730_with_mock(&expectations); // Declare as mutable
    assert_eq!(charger.address(), BQ25730_I2C_ADDRESS);
    charger.i2c.done();
}

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        // Set ChargeOption0: enable IIN_DPM (0x00)
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption0,
            &[0x0E, 0xE7], // LSB, MSB (Default LSB 0x0E, Default MSB 0xE7)
        ),
        // Set IIN_HOST: 3100mA (raw = 31)
        write_register_transaction(BQ25730_I2C_ADDRESS, Register::IinHost, 31), // 3200mA (raw = 31, default)
        // Set VSYS_MIN: 3500mV (raw = 35)
        write_register_transaction(BQ25730_I2C_ADDRESS, Register::VsysMin, 35),
        // Clear ChargerStatus flags (read current value, then write 0s to clear R/W bits)
        read_register_transaction(BQ25730_I2C_ADDRESS, Register::ChargerStatus, 0xFF), // Assume initial state is all flags set
        write_register_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargerStatus,
            0xE7, // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3) by writing 0
        ),
    ];

    let mut charger = new_bq25730_with_mock(&expectations);
    charger.init()?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ManufacturerID,
        0x40,
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let value = charger.read_register(Register::ManufacturerID)?;
    assert_eq!(value, 0x40);
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_write_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_register_transaction(
        BQ25730_I2C_ADDRESS,
        Register::VsysMin,
        0x23,
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger.write_register(Register::VsysMin, 0x23)?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers() -> Result<(), Error<ErrorKind>> {
    let expectations = [read_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption0,
        &[0x01, 0x02],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    let values = charger.read_registers(Register::ChargeOption0, 2)?;
    assert_eq!(values.as_ref() as &[u8], &[0x01, 0x02]);
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_write_registers() -> Result<(), Error<ErrorKind>> {
    let expectations = [write_registers_transaction(
        BQ25730_I2C_ADDRESS,
        Register::ChargeOption0,
        &[0x01, 0x02],
    )];
    let mut charger = new_bq25730_with_mock(&expectations);
    charger
        .write_registers(Register::ChargeOption0, &[0x01, 0x02])?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers_invalid_length() -> Result<(), Error<ErrorKind>> {
    let expectations = [];
    let mut charger = new_bq25730_with_mock(&expectations);
    let result = charger.read_registers(Register::ChargeOption0, 0);
    match result {
        Err(Error::InvalidData) => {
            charger.i2c.done();
            Ok(())
        }
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}

#[test]
fn test_write_registers_invalid_length() -> Result<(), Error<ErrorKind>> {
    let expectations = [];
    let mut charger = new_bq25730_with_mock(&expectations);
    let result = charger.write_registers(Register::ChargeOption0, &[]);
    match result {
        Err(Error::InvalidData) => {
            charger.i2c.done();
            Ok(())
        }
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}