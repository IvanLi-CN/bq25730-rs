#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::{
    BQ25730_I2C_ADDRESS, Bq25730, Error,
    data_types::{Config, OtgCurrentSetting, OtgVoltageSetting, SenseResistorValue},
    registers::Register,
};
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_otg_voltage() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGVoltage as u8, 0xC4, 0x09], // 5000mV (raw = 0x0271 -> 0x09C4) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
    charger.set_otg_voltage_setting(OtgVoltageSetting::from_millivolts(5000))?;
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
        vec![0xC4, 0x09], // 5000mV (raw = 0x0271 -> 0x09C4)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, config);
    let voltage_setting = charger.read_otg_voltage_setting()?;
    assert_eq!(voltage_setting.to_millivolts(), 5000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_otg_current() -> Result<(), Error<ErrorKind>> {
    let rsns_bat = SenseResistorValue::R5mOhm; // Assuming 5mOhm for this test
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);

    // Test case 1: 0mA
    let current_0ma = OtgCurrentSetting {
        milliamps: 0,
        rsns_bat,
    };
    let raw_0ma = current_0ma.to_raw(); // Should be 0
    let expectations_0ma = [
        I2cTransaction::write_read(
            // First op: read OTGCurrentMsb
            BQ25730_I2C_ADDRESS,
            vec![Register::OTGCurrentMsb as u8],
            vec![0x80], // Mocked read value (e.g., D7 set, other bits 0)
        ),
        I2cTransaction::write(
            // Second op: write OTGCurrent (LSB=0x00, MSB=new_val)
            BQ25730_I2C_ADDRESS,
            vec![
                Register::OTGCurrent as u8,
                0x00,
                (0x80 & !0x7F) | (raw_0ma & 0x7F),
            ], // LSB is 0x00, MSB preserves D7 and sets D6-D0 from raw_0ma
        ),
    ];
    let i2c_0ma = I2cMock::new(&expectations_0ma);
    let mut charger_0ma = Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    charger_0ma.set_otg_current_setting(current_0ma)?;
    charger_0ma.i2c.done();

    // Test case 2: 1000mA (raw = 10 for 5mOhm)
    let current_1000ma = OtgCurrentSetting {
        milliamps: 1000,
        rsns_bat,
    };
    let raw_1000ma = current_1000ma.to_raw(); // Should be 10 (0x0A)
    let initial_msb_read_1000ma = 0x00; // Assume other bits in OTGCurrentMsb are 0
    let expectations_1000ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::OTGCurrentMsb as u8],
            vec![initial_msb_read_1000ma],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::OTGCurrent as u8,
                0x00,
                (initial_msb_read_1000ma & !0x7F) | (raw_1000ma & 0x7F),
            ],
        ),
    ];
    let i2c_1000ma = I2cMock::new(&expectations_1000ma);
    let mut charger_1000ma = Bq25730::new(i2c_1000ma, BQ25730_I2C_ADDRESS, config);
    charger_1000ma.set_otg_current_setting(current_1000ma)?;
    charger_1000ma.i2c.done();

    // Test case 3: 12700mA (raw = 127 (0x7F) for 5mOhm)
    let current_12700ma = OtgCurrentSetting {
        milliamps: 12700,
        rsns_bat,
    };
    let raw_12700ma = current_12700ma.to_raw(); // Should be 127 (0x7F)
    let initial_msb_read_12700ma = 0x00; // Assume other bits in OTGCurrentMsb are 0
    let expectations_12700ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::OTGCurrentMsb as u8],
            vec![initial_msb_read_12700ma],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                Register::OTGCurrent as u8,
                0x00,
                (initial_msb_read_12700ma & !0x7F) | (raw_12700ma & 0x7F),
            ],
        ),
    ];
    let i2c_12700ma = I2cMock::new(&expectations_12700ma);
    let mut charger_12700ma = Bq25730::new(i2c_12700ma, BQ25730_I2C_ADDRESS, config);
    charger_12700ma.set_otg_current_setting(current_12700ma)?;
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
    let mut charger_0ma = Bq25730::new(i2c_0ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_0ma.read_otg_current_setting()?,
        OtgCurrentSetting {
            milliamps: 0,
            rsns_bat
        }
    );
    charger_0ma.i2c.done();

    // Test case 2: 1000mA (raw = 10 (0x0A))
    let expectations_1000ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrentMsb as u8],
        vec![0x0A], // MSB (raw_7bit = 10)
    )];
    let i2c_1000ma = I2cMock::new(&expectations_1000ma);
    let mut charger_1000ma = Bq25730::new(i2c_1000ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_1000ma.read_otg_current_setting()?,
        OtgCurrentSetting {
            milliamps: 1000,
            rsns_bat
        }
    );
    charger_1000ma.i2c.done();

    // Test case 3: 12700mA (raw = 127 (0x7F))
    let expectations_12700ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::OTGCurrentMsb as u8],
        vec![0x7F], // MSB (raw_7bit = 127)
    )];
    let i2c_12700ma = I2cMock::new(&expectations_12700ma);
    let mut charger_12700ma = Bq25730::new(i2c_12700ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_12700ma.read_otg_current_setting()?,
        OtgCurrentSetting {
            milliamps: 12700,
            rsns_bat
        }
    );
    charger_12700ma.i2c.done();

    Ok(())
}
