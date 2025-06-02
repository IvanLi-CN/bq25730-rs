#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::{
    BQ25730_I2C_ADDRESS, Error, RegisterAccess,
    data_types::{Config, SenseResistorValue},
    registers::Register,
};
use embedded_hal::i2c::ErrorKind;

#[cfg(not(feature = "async"))]
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

#[cfg(not(feature = "async"))]
#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    let cell_count = 4;
    // Corrected Config::new call
    let config = Config::new(
        cell_count,
        SenseResistorValue::default(), // R5mOhm for battery
        SenseResistorValue::R10mOhm,   // R10mOhm for AC
    );

    let charge_option0_bytes = config.charge_option0.to_msb_lsb_bytes();
    let (cc_lsb, cc_msb) = config.charge_current.to_msb_lsb_bytes();
    let (cv_lsb, cv_msb) = config.charge_voltage.to_msb_lsb_bytes();
    let (lsb_co1, msb_co1) = config.charge_option1.to_msb_lsb_bytes();
    let (iv_lsb, iv_msb) = config.input_voltage.to_msb_lsb_bytes();
    let (vm_lsb, vm_msb) = config.vsys_min.to_msb_lsb_bytes();
    let (ih_lsb, ih_msb) = config.iin_host.to_msb_lsb_bytes(config.rsns_ac);

    // Expected LSB for ChargerStatus after clearing FAULT_SYSOVP (bit 4) and FAULT_VSYS_UVP (bit 3) from 0xFF
    let expected_charger_status_lsb_after_clear = 0xFF
        & !(bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_SYSOVP.bits()
            | bq25730_async_rs::registers::ChargerStatusFaultFlags::FAULT_VSYS_UVP.bits());
    let expected_charger_status_msb_after_clear = 0xFF; // MSB (status flags) is not changed by this operation in init

    let init_bytes_00_09 = [
        charge_option0_bytes.0,
        charge_option0_bytes.1,
        cc_lsb,
        cc_msb,
        cv_lsb,
        cv_msb,
        config.otg_voltage.to_msb_lsb_bytes().0,
        config.otg_voltage.to_msb_lsb_bytes().1,
        config.otg_current.to_msb_lsb_bytes().0,
        config.otg_current.to_msb_lsb_bytes().1,
    ];

    let (co3_lsb, co3_msb) = config.charge_option3.to_msb_lsb_bytes();
    let (co4_lsb, co4_msb) = config.charge_option4.to_msb_lsb_bytes();
    let (vmin_lsb, vmin_msb) = config.vmin_active_protection.to_msb_lsb_bytes();

    let expectations = [
        // 1. Write ChargeOption0, ChargeCurrent, ChargeVoltage, OTGVoltage, OTGCurrent (Registers 0x00-0x09)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let mut v = vec![Register::ChargeOption0 as u8];
            v.extend_from_slice(&init_bytes_00_09);
            v
        }),
        // 2. Write ChargeOption1 (Registers 0x30-0x31)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption1 as u8, lsb_co1, msb_co1],
        ),
        // 3. Write InputVoltage, VsysMin, IinHost (Registers 0x0A-0x0F)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::InputVoltage as u8,
                iv_lsb,
                iv_msb,
                vm_lsb,
                vm_msb,
                ih_lsb,
                ih_msb,
            ],
        ),
        // 4. Write ChargeOption3 (Registers 0x34-0x35)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption3 as u8, co3_lsb, co3_msb],
        ),
        // 5. Write ChargeOption4 and VminActiveProtection (Registers 0x3C-0x3F)
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS, // First argument
            vec![
                // Second argument
                Register::ChargeOption4 as u8,
                co4_lsb,
                co4_msb,
                vmin_lsb,
                vmin_msb,
            ],
        ),
        // 6. Read ChargerStatus (Registers 0x20-0x21)
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,                 // First argument
            vec![Register::ChargerStatus as u8], // Second argument
            vec![0xFF, 0xFF],                    // Third argument
        ),
        // 7. Write ChargerStatus (Registers 0x20-0x21) to clear specific faults
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::ChargerStatus as u8,
                expected_charger_status_lsb_after_clear,
                expected_charger_status_msb_after_clear,
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

#[cfg(not(feature = "async"))]
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

#[cfg(not(feature = "async"))]
#[test]
fn test_write_register() -> Result<(), Error<ErrorKind>> {
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::VsysMin as u8, 0x23],
    )];
    let i2c = I2cMock::new(&expectations);
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.write_register(Register::VsysMin, 0x23)?;
    charger.i2c.done();
    Ok(())
}

#[cfg(not(feature = "async"))]
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

#[cfg(not(feature = "async"))]
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

#[cfg(not(feature = "async"))]
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

#[cfg(not(feature = "async"))]
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
