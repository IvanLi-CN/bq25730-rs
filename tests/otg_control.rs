#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::BQ25730_I2C_ADDRESS;
use bq25730_async_rs::{data_types::*, registers::Register, Error}; // Removed Rsns from here
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_otg_voltage() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGVoltage as u8, 0x88, 0x13], // 5000mV (raw = 625) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_otg_voltage(OtgVoltage(5000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_voltage() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGVoltage as u8],
        vec![0x88, 0x13], // 5000mV (raw = 625)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_otg_voltage()?;
    assert_eq!(voltage.0, 5000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_otg_current() -> Result<(), Error<ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm; // Assuming 5mOhm for this test
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let current_0ma = OtgCurrent { milliamps: 0, rsns_bat };
    let raw_0ma = current_0ma.to_raw(); // Should be 0
    let expectations_0ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8], vec![0x80]), // Mock initial read with D7 set
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8, 0x80 | raw_0ma]),
    ];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = bq25730_async_rs::Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    charger_0ma.set_otg_current(current_0ma)?;
    charger_0ma.i2c.done();

    // Test case 2: 1000mA (raw = 10 for 5mOhm)
    let current_1000ma = OtgCurrent { milliamps: 1000, rsns_bat };
    let raw_1000ma = current_1000ma.to_raw(); // Should be 10 (0x0A)
    let expectations_1000ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8, raw_1000ma]),
    ];
    let i2c_1000ma = I2cMock::new(&expectations_1000ma);
    let mut charger_1000ma = bq25730_async_rs::Bq25730::new(i2c_1000ma, BQ25730_I2C_ADDRESS, config);
    charger_1000ma.set_otg_current(current_1000ma)?;
    charger_1000ma.i2c.done();

    // Test case 3: 12700mA (raw = 127 (0x7F) for 5mOhm)
    let current_12700ma = OtgCurrent { milliamps: 12700, rsns_bat };
    let raw_12700ma = current_12700ma.to_raw(); // Should be 127 (0x7F)
    let expectations_12700ma = [
        I2cTransaction::write_read(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8], vec![0x00]),
        I2cTransaction::write(BQ25730_I2C_ADDRESS, vec![Register::OTGCurrentMsb as u8, raw_12700ma]),
    ];
    let i2c_12700ma = I2cMock::new(&expectations_12700ma);
    let mut charger_12700ma = bq25730_async_rs::Bq25730::new(i2c_12700ma, BQ25730_I2C_ADDRESS, config);
    charger_12700ma.set_otg_current(current_12700ma)?;
    charger_12700ma.i2c.done();

    Ok(())
}

#[test]
fn test_read_otg_current() -> Result<(), Error<ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA (raw = 0)
    let expectations_0ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrentMsb as u8],
        vec![0x00], // MSB (raw_7bit = 0)
    )];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = bq25730_async_rs::Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_0ma.read_otg_current()?,
        OtgCurrent { milliamps: 0, rsns_bat }
    );
    charger_0ma.i2c.done();

    // Test case 2: 1000mA (raw = 10 (0x0A))
    let expectations_1000ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrentMsb as u8],
        vec![0x0A], // MSB (raw_7bit = 10)
    )];
    let i2c_1000ma = I2cMock::new(&expectations_1000ma);
    let mut charger_1000ma = bq25730_async_rs::Bq25730::new(i2c_1000ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_1000ma.read_otg_current()?,
        OtgCurrent { milliamps: 1000, rsns_bat }
    );
    charger_1000ma.i2c.done();

    // Test case 3: 12700mA (raw = 127 (0x7F))
    let expectations_12700ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrentMsb as u8],
        vec![0x7F], // MSB (raw_7bit = 127)
    )];
    let i2c_12700ma = I2cMock::new(&expectations_12700ma);
    let mut charger_12700ma = bq25730_async_rs::Bq25730::new(i2c_12700ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_12700ma.read_otg_current()?,
        OtgCurrent { milliamps: 12700, rsns_bat }
    );
    charger_12700ma.i2c.done();

    Ok(())
}
