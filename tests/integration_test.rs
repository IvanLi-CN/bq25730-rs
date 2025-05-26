use bq25730_async_rs::registers; // Explicitly import registers module
use bq25730_async_rs::{
    data_types::*, errors::Error, registers::Register, Bq25730, BQ25730_I2C_ADDRESS,
};
use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c::{Mock as MockI2c, Transaction as MockTransaction}; // Import ErrorKind

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    // Changed error type to ErrorKind
    // Expected I2C transactions during initialization
    let expectations = [
        // Set ChargeOption0: enable IIN_DPM (0x00)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                registers::Register::ChargeOption0 as u8,
                registers::CHARGE_OPTION0_EN_IIN_DPM,
                0x00,
            ],
        ),
        // Set IIN_HOST: 3200mA (raw = 31)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::IinHost as u8, 31],
        ),
        // Set VSYS_MIN: 3500mV (raw = 35)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::VsysMin as u8, 35],
        ),
        // Clear ChargerStatus flags (write 1s to clear)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                registers::Register::ChargerStatus as u8,
                registers::CHARGER_STATUS_FAULT_ACOV
                    | registers::CHARGER_STATUS_FAULT_BATOC
                    | registers::CHARGER_STATUS_FAULT_ACOC
                    | registers::CHARGER_STATUS_FAULT_SYSOVP
                    | registers::CHARGER_STATUS_FAULT_VSYS_UVP
                    | registers::CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF
                    | registers::CHARGER_STATUS_FAULT_OTG_OVP
                    | registers::CHARGER_STATUS_FAULT_OTG_UVP,
            ],
        ),
    ];

    let i2c = MockI2c::new(&expectations);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS);

    // Call the init function
    charger.init()?;

    // Verify that all expected transactions occurred
    charger.i2c.done();

    Ok(())
}
