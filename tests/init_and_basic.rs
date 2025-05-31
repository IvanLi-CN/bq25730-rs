#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{RegisterAccess, data_types::{Config, SenseResistorValue}}; // Updated imports
use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_new() {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call to provide both rsns_bat and rsns_ac
    let config = Config::new(4, SenseResistorValue::R5mOhm, SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    assert_eq!(charger.address(), BQ25730_I2C_ADDRESS);
    charger.i2c.done();
}

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    let cell_count = 4;
    // Corrected Config::new call
    let config = Config::new(cell_count, SenseResistorValue::default(), SenseResistorValue::R10mOhm);

    let expectations = [
        // Write ChargeOption0
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8, config.charge_option0.lsb_flags.bits()],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0Msb as u8, config.charge_option0.msb_flags.bits()],
        ),
        // Write ChargeOption1
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8, config.charge_option1.lsb_flags.bits()],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1Msb as u8, config.charge_option1.msb_flags.bits()],
        ),
        // Bulk write Group 1 (ChargeCurrent, ChargeVoltage)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8, (config.charge_current & 0xFF) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8, (config.charge_current >> 8) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeVoltage as u8, (config.charge_voltage & 0xFF) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeVoltageMsb as u8, (config.charge_voltage >> 8) as u8]),
        // Bulk write Group 2 (InputVoltage, VsysMin, IinHost)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::InputVoltage as u8, (config.input_voltage & 0xFF) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::InputVoltageMsb as u8, (config.input_voltage >> 8) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::VsysMin as u8, (config.vsys_min & 0xFF) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::VsysMinMsb as u8, (config.vsys_min >> 8) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::IinHost as u8, (config.iin_host & 0xFF) as u8]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::IinHostMsb as u8, (config.iin_host >> 8) as u8]),
        // Read current ChargerStatus
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargerStatus as u8],
            vec![0xFF, 0xFF], // LSB, MSB - Assume initial state is all flags set
        ),
        // Write to clear SYSOVP and VSYS_UVP faults
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargerStatus as u8, 0xE7, 0xFF], // LSB with FAULT_SYSOVP and FAULT_VSYS_UVP cleared
        ),
    ];

    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let value = charger.read_register(Register::ManufacturerID)?;
    assert_eq!(value, 0x40);
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_write_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::VsysMin as u8, 0x23], // This test writes only LSB of VsysMin, which is unusual.
                                               // VsysMin is typically an 8-bit value in the MSB register (0x0D).
                                               // The LSB register (0x0C) is reserved.
                                               // For a valid test, we should write to VsysMinMsb.
                                               // However, to keep the test logic similar for now, we'll assume this is testing raw byte write.
    )];
    let i2c = I2cMock::new(&expectations);
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.write_register(Register::VsysMin, 0x23)?; // Writing to LSB of VsysMin (0x0C)
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
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.write_registers(Register::ChargeOption0, &[0x01, 0x02])?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers_invalid_length() -> Result<(), Error<ErrorKind>> {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let result = charger.write_registers(Register::ChargeOption0, &[]);
    match result {
        Err(Error::InvalidData) => {
            charger.i2c.done();
            Ok(())
        }
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}
