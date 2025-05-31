#![allow(clippy::approx_constant)]

use bq25730_async_rs::{
    data_types::{Config, SenseResistorValue}, // Updated imports
    registers::Register,
    BQ25730_I2C_ADDRESS,
};

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

// ChargeCurrent (03/02h)
#[test]
fn test_charge_control_set_charge_current(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm; // Assuming 5mOhm for this test
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let current_0ma = bq25730_async_rs::data_types::ChargeCurrent { milliamps: 0, rsns_bat };
    let raw_0ma = current_0ma.to_raw();
    let d1_d0_0ma = raw_0ma & 0x03;
    let d6_d2_0ma = (raw_0ma >> 2) & 0x1F;
    let expectations_0ma = [
        // Read LSB (0x02)
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]), // Initial LSB value
        // Write LSB (0x02)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8, d1_d0_0ma << 6]),
        // Read MSB (0x03)
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x00]), // Initial MSB value
        // Write MSB (0x03)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8, d6_d2_0ma]),
    ];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = bq25730_async_rs::Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    charger_0ma.set_charge_current(current_0ma)?;
    charger_0ma.i2c.done();

    // Test case 2: 8192mA (Max for 5mOhm is 127*128 = 16256mA, so this should be clamped if > 7bit raw)
    // For 5mOhm, LSB = 128mA. 8192mA / 128mA = 64 (0x40)
    let current_8192ma = bq25730_async_rs::data_types::ChargeCurrent { milliamps: 8192, rsns_bat };
    let raw_8192ma = current_8192ma.to_raw(); // Should be 64 (0x40)
    let d1_d0_8192ma = raw_8192ma & 0x03; // 0x00
    let d6_d2_8192ma = (raw_8192ma >> 2) & 0x1F; // 0x10
    let expectations_8192ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8, d1_d0_8192ma << 6]),
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8, d6_d2_8192ma]),
    ];
    let i2c_8192ma = I2cMock::new(&expectations_8192ma);
    let mut charger_8192ma = bq25730_async_rs::Bq25730::new(i2c_8192ma, BQ25730_I2C_ADDRESS, config);
    charger_8192ma.set_charge_current(current_8192ma)?;
    charger_8192ma.i2c.done();

    // Test case 3: 2560mA
    // For 5mOhm, LSB = 128mA. 2560mA / 128mA = 20 (0x14)
    let current_2560ma = bq25730_async_rs::data_types::ChargeCurrent { milliamps: 2560, rsns_bat };
    let raw_2560ma = current_2560ma.to_raw(); // Should be 20 (0x14)
    let d1_d0_2560ma = raw_2560ma & 0x03; // 0x00
    let d6_d2_2560ma = (raw_2560ma >> 2) & 0x1F; // 0x05
    let expectations_2560ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8, d1_d0_2560ma << 6]),
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8, d6_d2_2560ma]),
    ];
    let i2c_2560ma = I2cMock::new(&expectations_2560ma);
    let mut charger_2560ma = bq25730_async_rs::Bq25730::new(i2c_2560ma, BQ25730_I2C_ADDRESS, config);
    charger_2560ma.set_charge_current(current_2560ma)?;
    charger_2560ma.i2c.done();

    Ok(())
}

#[test]
fn test_charge_control_read_charge_current(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let expectations_0ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]), // LSB (D1-D0 = 0)
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x00]), // MSB (D6-D2 = 0)
    ];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = bq25730_async_rs::Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_0ma.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent { milliamps: 0, rsns_bat }
    );
    charger_0ma.i2c.done();

    // Test case 2: 8192mA (raw_7bit = 64 (0x40)) -> D1-D0 = 0, D6-D2 = 0x10
    let expectations_8192ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]), // LSB (D1-D0 = 0)
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x10]), // MSB (D6-D2 = 0x10)
    ];
    let i2c_8192ma = I2cMock::new(&expectations_8192ma);
    let mut charger_8192ma = bq25730_async_rs::Bq25730::new(i2c_8192ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_8192ma.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent { milliamps: 8192, rsns_bat }
    );
    charger_8192ma.i2c.done();

    // Test case 3: 2560mA (raw_7bit = 20 (0x14)) -> D1-D0 = 0, D6-D2 = 0x05
    let expectations_2560ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrent as u8], vec![0x00]), // LSB (D1-D0 = 0)
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::ChargeCurrentMsb as u8], vec![0x05]), // MSB (D6-D2 = 0x05)
    ];
    let i2c_2560ma = I2cMock::new(&expectations_2560ma);
    let mut charger_2560ma = bq25730_async_rs::Bq25730::new(i2c_2560ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_2560ma.read_charge_current()?,
        bq25730_async_rs::data_types::ChargeCurrent { milliamps: 2560, rsns_bat }
    );
    charger_2560ma.i2c.done();

    Ok(())
}

// ChargeVoltage (05/04h)
#[test]
fn test_charge_control_set_charge_voltage(
) -> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeVoltage as u8, 0x00, 0x04], // LSB, MSB for 1024mV (raw 12-bit value 0x080 -> 0x0400)
        ),
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

    charger.set_charge_voltage(bq25730_async_rs::data_types::ChargeVoltage(1024))?;

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
            bq25730_async_rs::data_types::ChargeVoltage(1024) // 1024mV
                .to_u16() // This should be 0x0400 for 1024mV (raw 12-bit 0x080)
                .to_le_bytes()
                .to_vec(),
        ),
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

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
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                bq25730_async_rs::registers::Register::ChargeOption0 as u8,
                0x00,
                0x00,
            ],
        ), // LSB, MSB
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
            );
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![
                bq25730_async_rs::registers::Register::ChargeOption0 as u8,
                lsb,
                msb,
            ]
        }), // LSB, MSB (EN_CMP_LATCH)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
            );
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![
                bq25730_async_rs::registers::Register::ChargeOption0 as u8,
                lsb,
                msb,
            ]
        }), // LSB, MSB (EN_LWPWR)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option = bq25730_async_rs::data_types::ChargeOption0::from_u16(
                (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                    | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits()
                        as u16)
                        << 8),
            );
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![
                bq25730_async_rs::registers::Register::ChargeOption0 as u8,
                lsb,
                msb,
            ]
        }), // LSB, MSB (EN_CMP_LATCH | EN_LWPWR)
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);

    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        0x0000,
    ))?;
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
    ))?;
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
    ))?;
    charger.set_charge_option0(bq25730_async_rs::data_types::ChargeOption0::from_u16(
        (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
            | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8),
    ))?;

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
        ),
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(
                bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ),
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![bq25730_async_rs::registers::Register::ChargeOption0 as u8],
            bq25730_async_rs::data_types::ChargeOption0::from_u16(
                (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ),
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
        ),
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);

    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(0x0000)
    );
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16
        )
    );
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            (bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8
        )
    );
    assert_eq!(
        charger.read_charge_option0()?,
        bq25730_async_rs::data_types::ChargeOption0::from_u16(
            (bq25730_async_rs::registers::ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                | ((bq25730_async_rs::registers::ChargeOption0MsbFlags::EN_LWPWR.bits() as u16)
                    << 8)
        )
    );

    charger.i2c.done();
    Ok(())
}
