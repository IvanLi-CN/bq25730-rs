#![allow(clippy::approx_constant)]

use bq25730_async_rs::{
    BQ25730_I2C_ADDRESS, Bq25730,
    data_types::{
        ChargeCurrentSetting, ChargeOption0, ChargeVoltageSetting, Config, SenseResistorValue,
    },
    registers::{ChargeOption0Flags, ChargeOption0MsbFlags, Register},
};

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

// ChargeCurrent (03/02h)
#[test]
fn test_charge_control_set_charge_current()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm; // Assuming 5mOhm for this test
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let current_0ma = ChargeCurrentSetting {
        milliamps: 0,
        rsns_bat,
    };
    let (lsb_0ma, msb_0ma) = current_0ma.to_msb_lsb_bytes();
    let expectations_0ma = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8, lsb_0ma, msb_0ma],
    )];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    charger_0ma.set_charge_current_setting(current_0ma)?;
    charger_0ma.i2c.done();

    // Test case 2: 8192mA (Max for 5mOhm is 127*128 = 16256mA, so this should be clamped if > 7bit raw)
    // For 5mOhm, LSB = 128mA. 8192mA / 128mA = 64 (0x40)
    let current_8192ma = ChargeCurrentSetting {
        milliamps: 8192,
        rsns_bat,
    };
    let (lsb_8192ma, msb_8192ma) = current_8192ma.to_msb_lsb_bytes();
    let expectations_8192ma = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8, lsb_8192ma, msb_8192ma],
    )];
    let i2c_8192ma = I2cMock::new(&expectations_8192ma);
    let mut charger_8192ma = Bq25730::new(i2c_8192ma, BQ25730_I2C_ADDRESS, config);
    charger_8192ma.set_charge_current_setting(current_8192ma)?;
    charger_8192ma.i2c.done();

    // Test case 3: 2560mA
    // For 5mOhm, LSB = 128mA. 2560mA / 128mA = 20 (0x14)
    let current_2560ma = ChargeCurrentSetting {
        milliamps: 2560,
        rsns_bat,
    };
    let (lsb_2560ma, msb_2560ma) = current_2560ma.to_msb_lsb_bytes();
    let expectations_2560ma = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8, lsb_2560ma, msb_2560ma],
    )];
    let i2c_2560ma = I2cMock::new(&expectations_2560ma);
    let mut charger_2560ma = Bq25730::new(i2c_2560ma, BQ25730_I2C_ADDRESS, config);
    charger_2560ma.set_charge_current_setting(current_2560ma)?;
    charger_2560ma.i2c.done();

    Ok(())
}

#[test]
fn test_charge_control_read_charge_current()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let expectations_0ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8],
        vec![0x00, 0x00], // LSB, MSB for 0mA
    )];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_0ma.read_charge_current_setting()?,
        ChargeCurrentSetting {
            milliamps: 0,
            rsns_bat
        }
    );
    charger_0ma.i2c.done();

    // Test case 2: 8192mA (raw_7bit = 64 (0x40)) -> LSB=0x00, MSB=0x10
    let expectations_8192ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8],
        vec![0x00, 0x10], // LSB, MSB for 8192mA
    )];
    let i2c_8192ma = I2cMock::new(&expectations_8192ma);
    let mut charger_8192ma = Bq25730::new(i2c_8192ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_8192ma.read_charge_current_setting()?,
        ChargeCurrentSetting {
            milliamps: 8192,
            rsns_bat
        }
    );
    charger_8192ma.i2c.done();

    // Test case 3: 2560mA (raw_7bit = 20 (0x14)) -> LSB=0x00, MSB=0x05
    let expectations_2560ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeCurrent as u8],
        vec![0x00, 0x05], // LSB, MSB for 2560mA
    )];
    let i2c_2560ma = I2cMock::new(&expectations_2560ma);
    let mut charger_2560ma = Bq25730::new(i2c_2560ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_2560ma.read_charge_current_setting()?,
        ChargeCurrentSetting {
            milliamps: 2560,
            rsns_bat
        }
    );
    charger_2560ma.i2c.done();

    Ok(())
}

// ChargeVoltage (05/04h)
#[test]
fn test_charge_control_set_charge_voltage()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let voltage_to_set = ChargeVoltageSetting::from_millivolts(1024);
    let (lsb, msb) = voltage_to_set.to_msb_lsb_bytes();
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeVoltage as u8, lsb, msb],
    )];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

    charger.set_charge_voltage_setting(voltage_to_set)?;

    charger.i2c.done();
    Ok(())
}

#[test]
fn test_charge_control_read_charge_voltage()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expected_voltage = ChargeVoltageSetting::from_millivolts(1024);
    let (lsb, msb) = expected_voltage.to_msb_lsb_bytes();
    let expectations = [embedded_hal_mock::eh1::i2c::Transaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::ChargeVoltage as u8],
        vec![lsb, msb],
    )];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

    assert_eq!(charger.read_charge_voltage_setting()?, expected_voltage);

    charger.i2c.done();
    Ok(())
}

// ChargeOption0 (01/00h)
#[test]
fn test_charge_control_set_charge_option0()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8, 0x00, 0x00],
        ), // LSB, MSB
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option = ChargeOption0::from_u16(ChargeOption0Flags::EN_CMP_LATCH.bits() as u16);
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![Register::ChargeOption0 as u8, lsb, msb]
        }), // LSB, MSB (EN_CMP_LATCH)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option =
                ChargeOption0::from_u16((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8);
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![Register::ChargeOption0 as u8, lsb, msb]
        }), // LSB, MSB (EN_LWPWR)
        I2cTransaction::write(BQ25730_I2C_ADDRESS, {
            let option = ChargeOption0::from_u16(
                (ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                    | ((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8),
            );
            let (lsb, msb) = option.to_msb_lsb_bytes();
            vec![Register::ChargeOption0 as u8, lsb, msb]
        }), // LSB, MSB (EN_CMP_LATCH | EN_LWPWR)
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

    charger.set_charge_option0(ChargeOption0::from_u16(0x0000))?;
    charger.set_charge_option0(ChargeOption0::from_u16(
        ChargeOption0Flags::EN_CMP_LATCH.bits() as u16,
    ))?;
    charger.set_charge_option0(ChargeOption0::from_u16(
        (ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8,
    ))?;
    charger.set_charge_option0(ChargeOption0::from_u16(
        (ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
            | ((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8),
    ))?;

    charger.i2c.done();
    Ok(())
}

#[test]
fn test_charge_control_read_charge_option0()
-> Result<(), bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>> {
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8],
            ChargeOption0::from_u16(0x0000)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ),
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8],
            ChargeOption0::from_u16(ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ),
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8],
            ChargeOption0::from_u16((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8)
                .to_u16()
                .to_le_bytes()
                .to_vec(),
        ),
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::ChargeOption0 as u8],
            ChargeOption0::from_u16(
                (ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                    | ((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8),
            )
            .to_u16()
            .to_le_bytes()
            .to_vec(),
        ),
    ];
    let i2c = I2cMock::new(&expectations);
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);

    assert_eq!(
        charger.read_charge_option0()?,
        ChargeOption0::from_u16(0x0000)
    );
    assert_eq!(
        charger.read_charge_option0()?,
        ChargeOption0::from_u16(ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
    );
    assert_eq!(
        charger.read_charge_option0()?,
        ChargeOption0::from_u16((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8)
    );
    assert_eq!(
        charger.read_charge_option0()?,
        ChargeOption0::from_u16(
            (ChargeOption0Flags::EN_CMP_LATCH.bits() as u16)
                | ((ChargeOption0MsbFlags::EN_LWPWR.bits() as u16) << 8)
        )
    );

    charger.i2c.done();
    Ok(())
}
