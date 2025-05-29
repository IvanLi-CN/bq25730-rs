#![allow(clippy::approx_constant)]

use bq25730_async_rs::{registers::Register, BQ25730_I2C_ADDRESS};

include!("common.rs");

// ChargeCurrent (03/02h)
#[test]
fn test_charge_control_set_charge_current(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeCurrent,
            &[0x00, 0x00],
        ), // 0mA
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeCurrent,
            &[0x00, 0x10],
        ), // 8192mA
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeCurrent,
            &[0x00, 0x05],
        ), // 2560mA
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    charger.set_charge_current(bq25730_async_rs::data_types::ChargeCurrent(0))?;
    charger.set_charge_current(bq25730_async_rs::data_types::ChargeCurrent(8192))?;
    charger.set_charge_current(bq25730_async_rs::data_types::ChargeCurrent(2560))?;

    charger.i2c.done();
    Ok(())
}

#[test]
fn test_charge_control_read_charge_current(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeCurrent as u8],
            bq25730_async_rs::data_types::ChargeCurrent(0)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ), // 0mA
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeCurrent as u8],
            bq25730_async_rs::data_types::ChargeCurrent(8192)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ), // 8192mA
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeCurrent as u8],
            bq25730_async_rs::data_types::ChargeCurrent(2560)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ), // 2560mA
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    assert_eq!(
        charger.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent(0)
    );
    assert_eq!(
        charger.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent(8192)
    );
    assert_eq!(
        charger.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent(2560)
    );

    charger.i2c.done();
    Ok(())
}

// ChargeVoltage (05/04h)
#[test]
fn test_charge_control_set_charge_voltage(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeVoltage as u8, 0x00, 0x04],
        ), // 1024mV (raw = 128)
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    charger.set_charge_voltage(bq25730_async_rs::data_types::ChargeVoltage(1024))?; // Min voltage is 1024mV

    charger.i2c.done();
    Ok(())
}

#[test]
fn test_charge_control_read_charge_voltage(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeVoltage as u8],
            bq25730_async_rs::data_types::ChargeVoltage(1024)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ), // 1024mV (raw = 128)
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    assert_eq!(
        charger.read_charge_voltage()?,
        bq25730_async_rs::data_types::ChargeVoltage(1024)
    );

    charger.i2c.done();
    Ok(())
}

// ChargeOption0 (01/00h)
#[test]
fn test_charge_control_set_charge_option0(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeOption0,
            &[0x00, 0x00],
        ), // LSB, MSB
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeOption0,
            {
                let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                    bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
                );
                let (lsb, msb) = option.to_msb_lsb_bytes();
                &[lsb, msb]
            },
        ), // LSB, MSB (EN_CMP_LATCH)
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeOption0,
            {
                let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                    (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16)
                        << 8,
                );
                let (lsb, msb) = option.to_msb_lsb_bytes();
                &[lsb, msb]
            },
        ), // LSB, MSB (EN_LWPWR)
        write_registers_transaction(
            BQ25730_I2C_ADDRESS,
            bq25730_async_rs::registers::Register::ChargeOption0,
            {
                let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                    (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                        | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits()
                            as u16)
                            << 8),
                );
                let (lsb, msb) = option.to_msb_lsb_bytes();
                &[lsb, msb]
            },
        ), // LSB, MSB (EN_CMP_LATCH | EN_LWPWR)
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        0x0000,
    ))?; // LSB, MSB
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
    ))?; // LSB, MSB (EN_CMP_LATCH)
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
    ))?; // LSB, MSB (EN_LWPWR)
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
            | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8),
    ))?; // LSB, MSB (EN_CMP_LATCH | EN_LWPWR)

    charger.i2c.done();
    Ok(())
}

#[test]
fn test_charge_control_read_charge_option0(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(0x0000)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ), // LSB, MSB
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(
                bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ), // LSB, MSB
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(
                (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ), // LSB, MSB
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(
                (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                    | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits()
                        as u16)
                        << 8),
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ), // LSB, MSB
    ];
    let mut charger = new_bq25730_with_mock(&expectations);

    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(0x0000)
    ); // LSB, MSB
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16
        )
    ); // LSB, MSB
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8
        )
    ); // LSB, MSB
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16)
                    << 8)
        )
    ); // LSB, MSB

    charger.i2c.done();
    Ok(())
}
