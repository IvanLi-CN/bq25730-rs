/// Helper function to create a new `Bq25730` instance with a mock I2C bus.
pub fn new_bq25730_with_mock(
    expectations: &[embedded_hal_mock::eh1::i2c::Transaction],
) -> bq25730_async_rs::Bq25730<embedded_hal_mock::eh1::i2c::Mock> {
    let i2c = embedded_hal_mock::eh1::i2c::Mock::new(expectations);
    bq25730_async_rs::Bq25730::new(i2c, bq25730_async_rs::BQ25730_I2C_ADDRESS, 4)
    // 默认电池节数 4
}

/// Helper function to create a mock transaction for writing a single register.
pub fn write_register_transaction(
    address: u8,
    reg: bq25730_async_rs::registers::Register,
    value: u8,
) -> embedded_hal_mock::eh1::i2c::Transaction {
    embedded_hal_mock::eh1::i2c::Transaction::write(address, vec![reg as u8, value])
}

/// Helper function to create a mock transaction for reading a single register.
pub fn read_register_transaction(
    address: u8,
    reg: bq25730_async_rs::registers::Register,
    value: u8,
) -> embedded_hal_mock::eh1::i2c::Transaction {
    embedded_hal_mock::eh1::i2c::Transaction::write_read(address, vec![reg as u8], vec![value])
}

/// Helper function to create a mock transaction for writing multiple registers.
pub fn write_registers_transaction(
    address: u8,
    reg: bq25730_async_rs::registers::Register,
    values: &[u8],
) -> embedded_hal_mock::eh1::i2c::Transaction {
    let mut data = vec![reg as u8];
    data.extend_from_slice(values);
    embedded_hal_mock::eh1::i2c::Transaction::write(address, data)
}

/// Helper function to create a mock transaction for reading multiple registers.
pub fn read_registers_transaction(
    address: u8,
    reg: bq25730_async_rs::registers::Register,
    read_data: &[u8],
) -> embedded_hal_mock::eh1::i2c::Transaction {
    embedded_hal_mock::eh1::i2c::Transaction::write_read(
        address,
        vec![reg as u8],
        read_data.to_vec(),
    )
}

/// Helper function to assert that an error is an I2C error.
pub fn assert_i2c_error<T: core::fmt::Debug>(
    result: Result<T, bq25730_async_rs::errors::Error<embedded_hal::i2c::ErrorKind>>,
    expected_kind: embedded_hal::i2c::ErrorKind,
) {
    match result {
        Err(bq25730_async_rs::errors::Error::I2c(e)) => assert_eq!(e, expected_kind),
        _ => panic!(
            "Expected I2c error with kind {:?}, got {:?}",
            expected_kind, result
        ),
    }
}

/// Helper function to assert that an error is an InvalidData error.
pub fn assert_invalid_data_error<T: core::fmt::Debug, E: core::fmt::Debug>(
    result: Result<T, bq25730_async_rs::errors::Error<E>>,
) {
    match result {
        Err(bq25730_async_rs::errors::Error::InvalidData) => {}
        _ => panic!("Expected InvalidData error, got {:?}", result),
    }
}
