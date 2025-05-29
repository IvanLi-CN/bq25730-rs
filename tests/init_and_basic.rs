#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::RegisterAccess;
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_new() {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4); // Declare as mutable
    assert_eq!(charger.address(), BQ25730_I2C_ADDRESS);
    charger.i2c.done();
}

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    let expectations = [
        // Read ChargeOption1 to determine RSNS_RAC setting (added for init)
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8],
            vec![0x00, 0x00], // Mocked value: LSB 0x00, MSB 0x00 (RSNS_RAC = 0)
        ),
        // Read ChargeOption0 to preserve other settings
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8],
            vec![0x0E, 0xE7], // Default LSB 0x0E, Default MSB 0xE7
        ),
        // Write the modified ChargeOption0 (LSB first) to enable IIN_DPM
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8, 0x0E | 0x02, 0xE7], // LSB with EN_IIN_DPM (bit 1) set
        ),
        // Set IIN_HOST: 3200mA (raw = 31)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::IinHost as u8, 00, 31]),
        // Set VSYS_MIN: 3500mV (raw = 35)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::VsysMin as u8, 00, 35]),
        // Read current ChargerStatus LSB (0x20)
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargerStatus as u8],
            vec![0xFF, 0xFF],
        ), // Assume initial state is all flags set
        // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3) by writing 0
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargerStatus as u8, 0xE7, 0xFF], // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3)
        ),
    ];

    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.init()?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ManufacturerID as u8],
        vec![0x40],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let value = charger.read_register(Register::ManufacturerID)?;
    assert_eq!(value, 0x40);
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_write_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::VsysMin as u8, 0x23],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.write_register(Register::VsysMin, 0x23)?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption0 as u8],
        vec![0x01, 0x02],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let values = charger.read_registers(Register::ChargeOption0, 2)?;
    assert_eq!(values.as_ref() as &[u8], &[0x01, 0x02]);
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_write_registers() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeOption0 as u8, 0x01, 0x02],
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    charger.write_registers(Register::ChargeOption0, &[0x01, 0x02])?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers_invalid_length() -> Result<(), Error<ErrorKind>> {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
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
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4);
    let result = charger.write_registers(Register::ChargeOption0, &[]);
    match result {
        Err(Error::InvalidData) => {
            charger.i2c.done();
            Ok(())
        }
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}
