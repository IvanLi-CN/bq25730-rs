#![allow(clippy::approx_constant)]

use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};

use bq25730_async_rs::data_types::*;
use bq25730_async_rs::errors::Error;
use bq25730_async_rs::registers::Register;
use bq25730_async_rs::{
    BQ25730_I2C_ADDRESS,
    data_types::{Config, SenseResistorValue},
}; // Updated imports
use embedded_hal::i2c::ErrorKind;

#[test]
fn test_set_input_voltage() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0x00, 0x00], // 3200mV (offset)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_input_voltage(InputVoltage(3200))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0x01, 0x00], // 3264mV (raw = 1) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_input_voltage(InputVoltage(3264))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![Register::InputVoltage as u8, 0xFF, 0x00], // 19520mV (raw = 255) - LSB, MSB
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_input_voltage(InputVoltage(19520))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_input_voltage() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0x00, 0x00],
        ), // 3200mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3200);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0x01, 0x00],
        ), // 3264mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 3264);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::InputVoltage as u8],
            vec![0xFF, 0x00],
        ), // 19520mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_input_voltage()?;
    assert_eq!(voltage.0, 19520);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_vsys_min() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(1000).to_u16().to_le_bytes()[0],
            VsysMin(1000).to_u16().to_le_bytes()[1],
        ], // 1000mV (raw = 10)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(1000).to_u16().to_le_bytes()[0],
            VsysMin(1000).to_u16().to_le_bytes()[1],
        ], // 1000mV (raw = 10)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_vsys_min(VsysMin(1000))?;
    charger.i2c.done();

    let expectations = [I2cTransaction::write(
        BQ25730_I2C_ADDRESS,
        vec![
            Register::VsysMin as u8,
            VsysMin(23000).to_u16().to_le_bytes()[0],
            VsysMin(23000).to_u16().to_le_bytes()[1],
        ], // 23000mV (raw = 230)
    )];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    charger.set_vsys_min(VsysMin(23000))?;
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_read_vsys_min() -> Result<(), Error<ErrorKind>> {
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), SenseResistorValue::R5mOhm);
    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(1000).to_u16().to_le_bytes().to_vec(),
        ), // 1000mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(1000).to_u16().to_le_bytes().to_vec(),
        ), // 1000mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 1000);
    charger.i2c.done();

    let expectations = [
        embedded_hal_mock::eh1::i2c::Transaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::VsysMin as u8],
            VsysMin(23000).to_u16().to_le_bytes().to_vec(),
        ), // 23000mV
    ];
    let i2c = I2cMock::new(&expectations);
    let mut charger =
        bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, config);
    let voltage = charger.read_vsys_min()?;
    assert_eq!(voltage.0, 23000);
    charger.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_host() -> Result<(), Error<ErrorKind>> {
    let rsns_ac = SenseResistorValue::R5mOhm; // Assuming 5mOhm for this test
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), rsns_ac);

    // Test case 1: 100mA (offset value for 5mOhm)
    let current_100ma = IinHost {
        milliamps: 100,
        rsns_ac,
    };
    let raw_100ma = current_100ma.to_raw(); // Should be 0
    let expectations_100ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8],
            vec![0x80],
        ), // Mock initial read with D7 set
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8, 0x80 | raw_100ma],
        ),
    ];
    let i2c_100ma = I2cMock::new(&expectations_100ma);
    let mut charger_100ma = bq25730_async_rs::Bq25730::new(i2c_100ma, BQ25730_I2C_ADDRESS, config);
    charger_100ma.set_iin_host(current_100ma)?;
    charger_100ma.i2c.done();

    // Test case 2: 200mA (raw = 1 for 5mOhm)
    let current_200ma = IinHost {
        milliamps: 200,
        rsns_ac,
    };
    let raw_200ma = current_200ma.to_raw(); // Should be 1
    let expectations_200ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8],
            vec![0x00],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8, raw_200ma],
        ),
    ];
    let i2c_200ma = I2cMock::new(&expectations_200ma);
    let mut charger_200ma = bq25730_async_rs::Bq25730::new(i2c_200ma, BQ25730_I2C_ADDRESS, config);
    charger_200ma.set_iin_host(current_200ma)?;
    charger_200ma.i2c.done();

    // Test case 3: 10000mA (raw = 99 for 5mOhm)
    let current_10000ma = IinHost {
        milliamps: 10000,
        rsns_ac,
    };
    let raw_10000ma = current_10000ma.to_raw(); // Should be 99 (0x63)
    let expectations_10000ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8],
            vec![0x00],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinHostMsb as u8, raw_10000ma],
        ),
    ];
    let i2c_10000ma = I2cMock::new(&expectations_10000ma);
    let mut charger_10000ma =
        bq25730_async_rs::Bq25730::new(i2c_10000ma, BQ25730_I2C_ADDRESS, config);
    charger_10000ma.set_iin_host(current_10000ma)?;
    charger_10000ma.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_host() -> Result<(), Error<ErrorKind>> {
    let rsns_ac = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), rsns_ac);

    // Test case 1: 100mA (raw = 0)
    let expectations_100ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHostMsb as u8],
        vec![0x00], // MSB (raw_7bit = 0)
    )];
    let i2c_100ma = I2cMock::new(&expectations_100ma);
    let mut charger_100ma = bq25730_async_rs::Bq25730::new(i2c_100ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_100ma.read_iin_host()?,
        IinHost {
            milliamps: 100,
            rsns_ac
        }
    );
    charger_100ma.i2c.done();

    // Test case 2: 200mA (raw = 1)
    let expectations_200ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHostMsb as u8],
        vec![0x01], // MSB (raw_7bit = 1)
    )];
    let i2c_200ma = I2cMock::new(&expectations_200ma);
    let mut charger_200ma = bq25730_async_rs::Bq25730::new(i2c_200ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_200ma.read_iin_host()?,
        IinHost {
            milliamps: 200,
            rsns_ac
        }
    );
    charger_200ma.i2c.done();

    // Test case 3: 10000mA (raw = 99 (0x63))
    let expectations_10000ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinHostMsb as u8],
        vec![0x63], // MSB (raw_7bit = 99)
    )];
    let i2c_10000ma = I2cMock::new(&expectations_10000ma);
    let mut charger_10000ma =
        bq25730_async_rs::Bq25730::new(i2c_10000ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_10000ma.read_iin_host()?,
        IinHost {
            milliamps: 10000,
            rsns_ac
        }
    );
    charger_10000ma.i2c.done();

    Ok(())
}

#[test]
fn test_set_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let rsns_ac = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), rsns_ac);

    let current_100ma = IinDpm {
        milliamps: 100,
        rsns_ac,
    };
    let raw_100ma = current_100ma.to_raw();
    let expectations_100ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8],
            vec![0x80],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8, 0x80 | raw_100ma],
        ),
    ];
    let i2c_100ma = I2cMock::new(&expectations_100ma);
    let mut charger_100ma = bq25730_async_rs::Bq25730::new(i2c_100ma, BQ25730_I2C_ADDRESS, config);
    charger_100ma.set_iin_dpm(current_100ma)?;
    charger_100ma.i2c.done();

    let current_200ma = IinDpm {
        milliamps: 200,
        rsns_ac,
    };
    let raw_200ma = current_200ma.to_raw();
    let expectations_200ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8],
            vec![0x00],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8, raw_200ma],
        ),
    ];
    let i2c_200ma = I2cMock::new(&expectations_200ma);
    let mut charger_200ma = bq25730_async_rs::Bq25730::new(i2c_200ma, BQ25730_I2C_ADDRESS, config);
    charger_200ma.set_iin_dpm(current_200ma)?;
    charger_200ma.i2c.done();

    let current_10000ma = IinDpm {
        milliamps: 10000,
        rsns_ac,
    };
    let raw_10000ma = current_10000ma.to_raw();
    let expectations_10000ma = [
        I2cTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8],
            vec![0x00],
        ),
        I2cTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![Register::IinDpmMsb as u8, raw_10000ma],
        ),
    ];
    let i2c_10000ma = I2cMock::new(&expectations_10000ma);
    let mut charger_10000ma =
        bq25730_async_rs::Bq25730::new(i2c_10000ma, BQ25730_I2C_ADDRESS, config);
    charger_10000ma.set_iin_dpm(current_10000ma)?;
    charger_10000ma.i2c.done();

    Ok(())
}

#[test]
fn test_read_iin_dpm() -> Result<(), Error<ErrorKind>> {
    let rsns_ac = SenseResistorValue::R5mOhm;
    // Corrected Config::new call
    let config = Config::new(4, SenseResistorValue::default(), rsns_ac);

    let expectations_100ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpmMsb as u8],
        vec![0x00],
    )];
    let i2c_100ma = I2cMock::new(&expectations_100ma);
    let mut charger_100ma = bq25730_async_rs::Bq25730::new(i2c_100ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_100ma.read_iin_dpm()?,
        IinDpm {
            milliamps: 100,
            rsns_ac
        }
    );
    charger_100ma.i2c.done();

    let expectations_200ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpmMsb as u8],
        vec![0x01],
    )];
    let i2c_200ma = I2cMock::new(&expectations_200ma);
    let mut charger_200ma = bq25730_async_rs::Bq25730::new(i2c_200ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_200ma.read_iin_dpm()?,
        IinDpm {
            milliamps: 200,
            rsns_ac
        }
    );
    charger_200ma.i2c.done();

    let expectations_10000ma = [I2cTransaction::write_read(
        BQ25730_I2C_ADDRESS,
        vec![Register::IinDpmMsb as u8],
        vec![0x63],
    )];
    let i2c_10000ma = I2cMock::new(&expectations_10000ma);
    let mut charger_10000ma =
        bq25730_async_rs::Bq25730::new(i2c_10000ma, BQ25730_I2C_ADDRESS, config);
    assert_eq!(
        charger_10000ma.read_iin_dpm()?,
        IinDpm {
            milliamps: 10000,
            rsns_ac
        }
    );
    charger_10000ma.i2c.done();

    Ok(())
}
