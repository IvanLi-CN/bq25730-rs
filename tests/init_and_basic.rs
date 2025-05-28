#![allow(clippy::approx_constant)]

include!("common.rs");

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::RegisterAccess;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
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
        // Read ChargeOption0 to preserve other settings
        read_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption0,
            &[0x0E, 0xE7], // Default LSB 0x0E, Default MSB 0xE7
        ),
        // Write the modified ChargeOption0 (LSB first) to enable IIN_DPM
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargeOption0,
            &[0x0E | 0x02, 0xE7], // LSB with EN_IIN_DPM (bit 1) set
        ),
        // Set IIN_HOST: 3200mA (raw = 31)
        write_register_transaction(BQ25730_I2C_ADDRESS, Register::IinHost, 31),
        // Set VSYS_MIN: 3500mV (raw = 35)
        write_register_transaction(BQ25730_I2C_ADDRESS, Register::VsysMin, 35),
        // Read current ChargerStatus LSB (0x20)
        read_register_transaction(BQ25730_I2C_ADDRESS, Register::ChargerStatus, 0xFF), // Assume initial state is all flags set
        // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3) by writing 0
        write_register_transaction(
            BQ25730_I2C_ADDRESS,
            Register::ChargerStatus,
            0xFF & !(0x10 | 0x08), // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3)
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
    charger.write_registers(Register::ChargeOption0, &[0x01, 0x02])?;
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
