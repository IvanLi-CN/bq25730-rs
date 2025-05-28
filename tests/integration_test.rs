use bq25730_async_rs::registers; // Explicitly import registers module
use bq25730_async_rs::{errors::Error, Bq25730, BQ25730_I2C_ADDRESS};
use embedded_hal::i2c::ErrorKind;
use embedded_hal_mock::eh1::i2c::{Mock as MockI2c, Transaction as MockTransaction}; // Import ErrorKind

#[test]
fn test_init() -> Result<(), Error<ErrorKind>> {
    // Changed error type to ErrorKind
    // Expected I2C transactions during initialization
    let expectations = [
        // Set ChargeOption0: enable IIN_DPM (0x00)
        // init function reads current value, then sets EN_IIN_DPM bit, then writes back.
        // Assuming default ChargeOption0 is 0xE70E (LSB 0x0E, MSB 0xE7)
        MockTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::ChargeOption0 as u8],
            vec![0x0E, 0xE7], // Default LSB, MSB
        ),
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                registers::Register::ChargeOption0 as u8,
                0x0E, // LSB (0x0E | EN_IIN_DPM which is 0x02, results in 0x0E)
                0xE7, // MSB (default)
            ],
        ),
        // Set IIN_HOST: 3100mA (raw = 31)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::IinHost as u8, 31],
        ),
        // Set VSYS_MIN: 3500mV (raw = 35)
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::VsysMin as u8, 35],
        ),
        // Clear ChargerStatus flags (read current value, then write 0s to clear R/W bits)
        MockTransaction::write_read(
            BQ25730_I2C_ADDRESS,
            vec![registers::Register::ChargerStatus as u8],
            vec![0xFF], // Assume initial state is all flags set for LSB
        ),
        MockTransaction::write(
            BQ25730_I2C_ADDRESS,
            vec![
                registers::Register::ChargerStatus as u8,
                0xE7, // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3) by writing 0
            ],
        ),
    ];

    let i2c = MockI2c::new(&expectations);
    let mut charger = Bq25730::new(i2c, BQ25730_I2C_ADDRESS, 4); // Added cell_count for test

    // Call the init function
    charger.init()?;

    // Verify that all expected transactions occurred
    charger.i2c.done();

    Ok(())
}
