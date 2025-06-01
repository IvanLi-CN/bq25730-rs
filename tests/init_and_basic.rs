#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{
    RegisterAccess,
    data_types::{Config, SenseResistorValue},
}; // Updated imports
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_new() {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call to provide both rsns_bat and rsns_ac
    let config = Config::new(4, SenseResistorValue::R5mOhm, SenseResistorValue::R5mOhm);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    assert_eq!(charger.address(), BQ25730_I2C_ADDRESS);
    charger.i2c.done();
}

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    let cell_count = 4;
    // Corrected Config::new call
    let config = Config::new(
        cell_count,
        SenseResistorValue::default(),
        SenseResistorValue::R10mOhm,
    );

    let charge_option0_bytes = config.charge_option0.to_msb_lsb_bytes();
    let charge_current_bytes = config.charge_current.to_le_bytes();
    let charge_voltage_bytes = config.charge_voltage.to_le_bytes();
    let (lsb_co1, msb_co1) = config.charge_option1.to_msb_lsb_bytes();
    let input_voltage_bytes = config.input_voltage.to_le_bytes();
    let vsys_min_bytes = config.vsys_min.to_le_bytes();
    let iin_host_bytes = config.iin_host.to_le_bytes();

    // Expected LSB for ChargerStatus after clearing FAULT_SYSOVP (bit 3) and FAULT_VSYS_UVP (bit 0) from 0xFF
    let expected_charger_status_lsb_after_clear = 0xFF
        & !(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_SYSOVP.bits()
            | bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_VSYS_UVP.bits());
    let expected_charger_status_msb_after_clear = 0xFF; // Assuming MSB (status flags) is not changed by this operation in init

    let expectations = [
        // 1. Write ChargeOption0, ChargeCurrent, ChargeVoltage (Registers 0x00-0x05)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::ChargeOption0 as u8,
                charge_option0_bytes.0,  // ChargeOption0 LSB (0x00)
                charge_option0_bytes.1,  // ChargeOption0 MSB (0x01)
                charge_current_bytes[0], // ChargeCurrent LSB (0x02)
                charge_current_bytes[1], // ChargeCurrent MSB (0x03)
                charge_voltage_bytes[0], // ChargeVoltage LSB (0x04)
                charge_voltage_bytes[1], // ChargeVoltage MSB (0x05)
            ],
        ),
        // 2. Write ChargeOption1 (Registers 0x30-0x31)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::ChargeOption1 as u8,
                lsb_co1, // ChargeOption1 LSB (0x30)
                msb_co1, // ChargeOption1 MSB (0x31)
            ],
        ),
        // 3. Write InputVoltage, VsysMin, IinHost (Registers 0x0A-0x0F)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::InputVoltage as u8,
                input_voltage_bytes[0], // InputVoltage LSB (0x0A)
                input_voltage_bytes[1], // InputVoltage MSB (0x0B)
                vsys_min_bytes[0],      // VsysMin LSB (0x0C)
                vsys_min_bytes[1],      // VsysMin MSB (0x0D)
                iin_host_bytes[0],      // IinHost LSB (0x0E)
                iin_host_bytes[1],      // IinHost MSB (0x0F)
            ],
        ),
        // 4. Read ChargerStatus (Registers 0x20-0x21)
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargerStatus as u8],
            vec![0xFF, 0xFF], // Mock: LSB (Faults), MSB (Status) - Assume all flags initially set
        ),
        // 5. Write ChargerStatus (Registers 0x20-0x21) to clear specific faults
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::ChargerStatus as u8,
                expected_charger_status_lsb_after_clear, // LSB with FAULT_SYSOVP and FAULT_VSYS_UVP cleared
                expected_charger_status_msb_after_clear, // MSB (status flags)
            ],
        ),
    ];

    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.write_registers(Register::ChargeOption0, &[0x01, 0x02])?;
    charger.i2c.done();
    Ok(())
}

#[test]
fn test_read_registers_invalid_length() -> Result<(), Error<ErrorKind>> {
    let expectations = [];
    let i2c = I2cMock::new(&expectations);
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
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
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let result = charger.write_registers(Register::ChargeOption0, &[]);
    match result {
        Err(Error::InvalidData) => {
            charger.i2c.done();
            Ok(())
        }
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}
