#![allow(unused_imports)]
#![allow(clippy::await_holding_refcell_ref)]

use bq25730_async_rs::{
    Bq25730,
    data_types::*,
    errors::Error,
    registers::{
        Register,
        CHARGE_OPTION0_EN_IIN_DPM,
        CHARGER_STATUS_FAULT_ACOV,
        CHARGER_STATUS_FAULT_BATOC,
        CHARGER_STATUS_FAULT_ACOC,
        CHARGER_STATUS_FAULT_SYSOVP,
        CHARGER_STATUS_FAULT_VSYS_UVP,
        CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF,
        CHARGER_STATUS_FAULT_OTG_OVP,
        CHARGER_STATUS_FAULT_OTG_UVP,
        CHARGE_OPTION0_CHRG_INHIBIT, // Add missing constants
        CHARGE_OPTION0_EN_LWPWR,
        CHARGE_OPTION0_EN_CMP_LATCH,
        CHARGER_STATUS_STAT_AC,
        CHARGER_STATUS_ICO_DONE,
        CHARGER_STATUS_IN_FCHRG,
        CHARGER_STATUS_IN_OTG,
        PROCHOT_STATUS_EN_PROCHOT_EXT, // Add missing constants
        PROCHOT_STATUS_STAT_VAP_FAIL,
        PROCHOT_STATUS_STAT_VINDPM,
        PROCHOT_STATUS_STAT_ICRIT,
        PROCHOT_STATUS_STAT_BAT_REMOVAL,
        CHARGE_OPTION4_STAT_IDCHG2,
        CHARGE_OPTION4_STAT_PTM,
        PROCHOT_STATUS_PROCHOT_WIDTH, // Add missing constant
        PROCHOT_STATUS_PROCHOT_CLEAR, // Add missing constant
        PROCHOT_STATUS_STAT_EXIT_VAP, // Add missing constant
        PROCHOT_STATUS_STAT_COMP, // Add missing constant
        PROCHOT_STATUS_STAT_INOM, // Add missing constant
        PROCHOT_STATUS_STAT_IDCHG1, // Add missing constant
        PROCHOT_STATUS_STAT_VSYS, // Add missing constant
        PROCHOT_STATUS_STAT_ADPT_REMOVAL, // Add missing constant
    },
    RegisterAccess,
};
use embedded_hal_async::i2c::{self, I2c};
use bq25730_async_rs::registers; // Import the registers module using the crate name

/// Mock I2C implementation for testing purposes.
/// It allows pre-setting expected write operations and read return values.
pub struct MockI2c {
    /// Expected write operations: (address, data)
    pub expected_writes: Vec<(u8, Vec<u8>)>,
    /// Pre-set read return values
    pub read_values: Vec<Vec<u8>>,
    /// Index for tracking current read operation
    read_idx: usize,
}

impl MockI2c {
    /// Creates a new `MockI2c` instance.
    pub fn new() -> Self {
        MockI2c {
            expected_writes: Vec::new(),
            read_values: Vec::new(),
            read_idx: 0,
        }
    }

    /// Sets the expected write operations.
    pub fn with_expected_writes(mut self, writes: Vec<(u8, Vec<u8>)>) -> Self {
        self.expected_writes = writes;
        self
    }

    /// Sets the pre-set read return values.
    pub fn with_read_values(mut self, reads: Vec<Vec<u8>>) -> Self {
        self.read_values = reads;
        self
    }

    /// Asserts that all expected writes have been performed.
    pub fn assert_all_writes_performed(&self) {
        assert!(self.expected_writes.is_empty(), "Not all expected writes were performed. Remaining: {:?}", self.expected_writes);
    }
}

impl i2c::ErrorType for MockI2c {
    type Error = i2c::ErrorKind; // Mock I2C uses ErrorKind for its error
}

impl i2c::I2c for MockI2c {
    async fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Self::Error> {
        #[cfg(feature = "defmt")]
        defmt::info!("MockI2c write: addr={}, data={:x}, len={}", addr, data, data.len());
        if let Some((expected_addr, expected_data)) = self.expected_writes.pop() {
            assert_eq!(addr, expected_addr, "Mismatched I2C address in write");
            assert_eq!(data, expected_data.as_slice(), "Mismatched I2C data in write");
            Ok(())
        } else {
            panic!("Unexpected I2C write: addr={}, data={:?}", addr, data);
        }
    }

    async fn write_read(&mut self, addr: u8, wr_data: &[u8], rd_data: &mut [u8]) -> Result<(), Self::Error> {
        if let Some((expected_addr, expected_wr_data)) = self.expected_writes.pop() {
            assert_eq!(addr, expected_addr, "Mismatched I2C address in write_read (write part)");
            assert_eq!(wr_data, expected_wr_data.as_slice(), "Mismatched I2C data in write_read (write part)");

            if self.read_idx < self.read_values.len() {
                let read_val = &self.read_values[self.read_idx];
                assert_eq!(rd_data.len(), read_val.len(), "Mismatched read data length");
                rd_data.copy_from_slice(read_val);
                self.read_idx += 1;
                Ok(())
            } else {
                panic!("Not enough pre-set read values for write_read");
            }
        } else {
            panic!("Unexpected I2C write_read: addr={}, wr_data={:?}", addr, wr_data);
        }
    }

    async fn transaction(
        &mut self,
        _address: u8,
        _operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}

// Test cases will go here

#[tokio::test]
async fn test_new_function() {
    let i2c = MockI2c::new();
    let address = 0x6B;
    let bq = Bq25730::new(i2c, address);

    assert_eq!(bq.address(), address);
    // We can't directly assert on the i2c field as it's private,
    // but its presence is implied by the driver's functionality.
}

#[tokio::test]
async fn test_init_function() {
    let expected_writes = vec![
        // init function writes in order:
        // 1. ChargeOption0 (0x00) LSB and MSB
        (0x6B, vec![Register::ChargeOption0 as u8, CHARGE_OPTION0_EN_IIN_DPM, 0]),
        // 2. IIN_HOST (0x0E) (3200mA = raw 31)
        (0x6B, vec![Register::IinHost as u8, 0x3F]), // 3200mA = raw 63 (0x3F)
        // 3. VSYS_MIN (0x0C) (3500mV = raw 35)
        (0x6B, vec![Register::VsysMin as u8, 0x23]),
        // 4. ChargerStatus (0x20) LSB to clear fault flags
        (0x6B, vec![Register::ChargerStatus as u8,
            CHARGER_STATUS_FAULT_ACOV
            | CHARGER_STATUS_FAULT_BATOC
            | CHARGER_STATUS_FAULT_ACOC
            | CHARGER_STATUS_FAULT_SYSOVP
            | CHARGER_STATUS_FAULT_VSYS_UVP
            | CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF
            | CHARGER_STATUS_FAULT_OTG_OVP
            | CHARGER_STATUS_FAULT_OTG_UVP
        ]),
    ];

    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect()); // Reverse to pop in order
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    bq.init().await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_write_register() {
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8, 0xCD]), // Expected write for ChargeOption0 with value 0xCD
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    bq.write_register(Register::ChargeOption0, 0xCD).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_registers() {
    let expected_reads = vec![
        vec![0x11, 0x22, 0x33], // Expected values for multiple register reads
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8]),
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    let values = bq.read_registers(Register::ChargeOption0, 3).await.unwrap();
    assert_eq!(values.as_ref() as &[u8], &[0x11, 0x22, 0x33]);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_write_registers() {
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8, 0xAA, 0xBB, 0xCC]), // Expected write for multiple registers
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    bq.write_registers(Register::ChargeOption0, &[0xAA, 0xBB, 0xCC]).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_registers_invalid_length() {
    let i2c = MockI2c::new();
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    // Test with length 0
    let result = bq.read_registers(Register::ChargeOption0, 0).await;
    assert!(matches!(result, Err(Error::InvalidData)));

    // Test with length > 30 (heapless::Vec capacity)
    let result = bq.read_registers(Register::ChargeOption0, 31).await;
    assert!(matches!(result, Err(Error::InvalidData)));
}

#[tokio::test]
async fn test_write_registers_invalid_length() {
    let i2c = MockI2c::new();
    let address = 0x6B;
    let mut bq = Bq25730::new(i2c, address);

    // Test with empty slice
    let result = bq.write_registers(Register::ChargeOption0, &[]).await;
    assert!(matches!(result, Err(Error::InvalidData)));

    // Test with slice length > 30 (heapless::Vec capacity + 1 for register address)
    let long_data = [0u8; 31];
    let result = bq.write_registers(Register::ChargeOption0, &long_data).await;
    assert!(matches!(result, Err(Error::InvalidData)));
}

#[tokio::test]
async fn test_set_charge_current() {
    // Test 0mA
    let expected_writes_0ma = vec![
        (0x6B, vec![Register::ChargeCurrent as u8, 0x00, 0x00]),
    ];
    let i2c_0ma = MockI2c::new().with_expected_writes(expected_writes_0ma.into_iter().rev().collect());
    let mut bq_0ma = Bq25730::new(i2c_0ma, 0x6B);
    bq_0ma.set_charge_current(ChargeCurrent(0)).await.unwrap();
    bq_0ma.i2c.assert_all_writes_performed();

    // Test max value (8192mA)
    let expected_writes_max = vec![
        (0x6B, vec![Register::ChargeCurrent as u8, 0x80, 0x00]), // 8192mA = raw 128 (0x80), LSB 0x80, MSB 0x00
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_charge_current(ChargeCurrent(8192)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (4000mA)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::ChargeCurrent as u8, 0x3E, 0x00]), // 4000mA = raw 62 (0x3E), LSB 0x3E, MSB 0x00
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_charge_current(ChargeCurrent(4000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_current() {
    // Test 0mA
    let expected_reads_0ma = vec![
        vec![0x00, 0x00], // LSB, MSB
    ];
    let expected_writes_0ma = vec![
        (0x6B, vec![Register::ChargeCurrent as u8]), // Corrected address
    ];
    let i2c_0ma = MockI2c::new()
        .with_read_values(expected_reads_0ma)
        .with_expected_writes(expected_writes_0ma.into_iter().rev().collect());
    let mut bq_0ma = Bq25730::new(i2c_0ma, 0x6B);
    let current_0ma = bq_0ma.read_charge_current().await.unwrap();
    assert_eq!(current_0ma.0, 0);
    bq_0ma.i2c.assert_all_writes_performed();

    // Test max value (8192mA)
    let expected_reads_max = vec![
        vec![0x80, 0x00], // LSB, MSB (raw 128)
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::ChargeCurrent as u8]), // Corrected address
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let current_max = bq_max.read_charge_current().await.unwrap();
    assert_eq!(current_max.0, 8192);
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (4000mA)
    let expected_reads_mid = vec![
        vec![0x3E, 0x00], // LSB, MSB (raw 62)
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::ChargeCurrent as u8]), // Corrected address
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let current_mid = bq_mid.read_charge_current().await.unwrap();
    assert_eq!(current_mid.0, 3968);
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_charge_voltage() {
    // Test min value (1024mV)
    let expected_writes_min = vec![
        (0x6B, vec![Register::ChargeVoltage as u8, 0, 0]), // 1024mV = raw 0, LSB 0, MSB 0
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_charge_voltage(ChargeVoltage(1024)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (19200mV)
    let expected_writes_max = vec![
        (0x6B, vec![Register::ChargeVoltage as u8, 0x00, 0x8E]), // 19200mV = raw 2272 (0x8E0), LSB 0x00, MSB 0x8E
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_charge_voltage(ChargeVoltage(19200)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (10000mV)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::ChargeVoltage as u8, 0x20, 0x46]), // 10000mV = raw 1122 (0x462), LSB 0x20, MSB 0x46
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_charge_voltage(ChargeVoltage(10000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_voltage() {
    // Test min value (1024mV)
    let expected_reads_min = vec![
        vec![0x00, 0x00], // LSB, MSB (raw 0)
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::ChargeVoltage as u8]), // Corrected address
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let voltage_min = bq_min.read_charge_voltage().await.unwrap();
    assert_eq!(voltage_min.0, 1024); // 1024mV = raw 0
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (19200mV)
    let expected_reads_max = vec![
        vec![0x00, 0x8E], // LSB, MSB (raw 2272)
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::ChargeVoltage as u8]), // Corrected address
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let voltage_max = bq_max.read_charge_voltage().await.unwrap();
    assert_eq!(voltage_max.0, 19200);
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (10000mV)
    let expected_reads_mid = vec![
        vec![0x20, 0x46], // LSB, MSB (raw 1122)
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::ChargeVoltage as u8]), // Corrected address
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let voltage_mid = bq_mid.read_charge_voltage().await.unwrap();
    assert_eq!(voltage_mid.0, 10000);
    bq_mid.i2c.assert_all_writes_performed();
}
#[tokio::test]
async fn test_set_charge_option0() {
    // Test setting a specific combination of LSB and MSB bits
    let lsb_value = registers::CHARGE_OPTION0_EN_IIN_DPM | registers::CHARGE_OPTION0_CHRG_INHIBIT;
    let msb_value = registers::CHARGE_OPTION0_EN_LWPWR | registers::CHARGE_OPTION0_EN_CMP_LATCH;

    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8, lsb_value, msb_value]),
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_charge_option0(lsb_value, msb_value).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_option0() {
    // Test reading a specific combination of LSB and MSB bits
    let expected_lsb = 0x0E; // Default LSB
    let expected_msb = 0xE7; // Default MSB

    let expected_reads = vec![
        vec![expected_lsb, expected_msb],
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_charge_option0().await.unwrap();
    assert_eq!(lsb, expected_lsb);
    assert_eq!(msb, expected_msb);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_otg_voltage() {
    // Test min value (3000mV)
    let expected_writes_min = vec![
        (0x6B, vec![Register::OTGVoltage as u8, 0x00, 0x00]), // 3000mV = raw 0, LSB 0, MSB 0
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_otg_voltage(OtgVoltage(3000)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (19200mV)
    let expected_writes_max = vec![
        (0x6B, vec![Register::OTGVoltage as u8, 0xE9, 0x07]), // 19200mV = raw 2025, LSB 0xE9, MSB 0x07 - Corrected comment and values
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_otg_voltage(OtgVoltage(19200)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (5000mV)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::OTGVoltage as u8, 0xFA, 0x00]), // 5000mV = raw 250, LSB 0xFA, MSB 0x00 - Corrected LSB value
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_otg_voltage(OtgVoltage(5000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_otg_voltage() {
    // Test min value (3000mV)
    let expected_reads_min = vec![
        vec![0x00, 0x00], // LSB, MSB (raw 0)
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::OTGVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let voltage_min = bq_min.read_otg_voltage().await.unwrap();
    assert_eq!(voltage_min.0, 3000);
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (19376mV)
    let expected_writes_max = vec![
        (0x6B, vec![Register::OTGVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(vec![vec![0xFF, 0x07]]) // Explicitly set read value for max case
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let voltage_max = bq_max.read_otg_voltage().await.unwrap();
    assert_eq!(voltage_max.0, 19376);
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (5000mV)
    let expected_reads_mid = vec![
        vec![0xFA, 0x00], // LSB, MSB (raw 250) - Corrected to hex
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::OTGVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let voltage_mid = bq_mid.read_otg_voltage().await.unwrap();
    assert_eq!(voltage_mid.0, 5000);
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_otg_current() {
    // Test min value (0mA)
    let expected_writes_min = vec![
        (0x6B, vec![Register::OTGCurrent as u8, 0x00, 0x00]), // 0mA = raw 0, LSB 0x00, MSB 0x00
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_otg_current(OtgCurrent(0)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (2560mA)
    let expected_writes_max = vec![
        (0x6B, vec![Register::OTGCurrent as u8, 0x19, 0x00]), // 2560mA = raw 25 (0x19), LSB 0x19, MSB 0x00
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_otg_current(OtgCurrent(2560)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::OTGCurrent as u8, 0x0A, 0x00]), // 1000mA = raw 10 (0x0A), LSB 0x0A, MSB 0x00
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_otg_current(OtgCurrent(1000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_otg_current() {
    // Test min value (0mA)
    let expected_reads_min = vec![
        vec![0x00, 0x00], // LSB, MSB (raw 0)
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::OTGCurrent as u8]), // Corrected address (LSB address)
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let current_min = bq_min.read_otg_current().await.unwrap();
    assert_eq!(current_min.0, 0);
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (2560mA)
    let expected_reads_max = vec![
        vec![0x19, 0x00], // LSB, MSB (raw 25)
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::OTGCurrent as u8]), // Corrected address (LSB address)
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let current_max = bq_max.read_otg_current().await.unwrap();
    assert_eq!(current_max.0, 2500); // 2500mA = raw 25
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_reads_mid = vec![
        vec![0x0A, 0x00], // LSB, MSB (raw 10)
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::OTGCurrent as u8]), // Corrected address (LSB address)
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let current_mid = bq_mid.read_otg_current().await.unwrap();
    assert_eq!(current_mid.0, 1000);
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_input_voltage_min() {
    // Test min value (3968mV)
    let expected_writes_min = vec![
        (0x6B, vec![Register::InputVoltage as u8, 0x0C, 0x00]), // 3968mV = raw 12, LSB 0x0C, MSB 0x00
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_input_voltage(InputVoltage(3968)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_input_voltage_max() {
    // Test max value (19200mV)
    let expected_writes_max = vec![
        (0x6B, vec![Register::InputVoltage as u8, 0xFA, 0x00]), // 19200mV = raw 250, LSB 0xFA, MSB 0x00 - Corrected LSB value
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_input_voltage(InputVoltage(19200)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_input_voltage_mid() {
    // Test middle value (5000mV)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::InputVoltage as u8, 0x1C, 0x00]), // 4992mV = raw 28 (0x1C), LSB 0x1C, MSB 0x00
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_input_voltage(InputVoltage(4992)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_input_voltage() {
    // Test min value (3520mV) - Corrected comment
    let expected_reads_min = vec![
        vec![0x05, 0x00], // LSB, MSB (raw 5) // 3520mV = raw 5
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::InputVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let voltage_min = bq_min.read_input_voltage().await.unwrap();
    assert_eq!(voltage_min.0, 3520);
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (19200mV)
    let expected_reads_max = vec![
        vec![0xFA, 0x00], // LSB, MSB (raw 250) // 19200mV = raw 250
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::InputVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let voltage_max = bq_max.read_input_voltage().await.unwrap();
    assert_eq!(voltage_max.0, 19200);
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (4992mV) - Corrected comment
    let expected_reads_mid = vec![
        vec![0x1C, 0x00], // LSB, MSB (raw 28) // 4992mV = raw 28
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::InputVoltage as u8]), // Corrected address (LSB address)
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let voltage_mid = bq_mid.read_input_voltage().await.unwrap();
    assert_eq!(voltage_mid.0, 4992);
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_vsys_min() {
    // Test min value (3500mV)
    let expected_writes_min = vec![
        (0x6B, vec![Register::VsysMin as u8, 0x23]), // 3500mV = raw 35
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_vsys_min(VsysMin(3500)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (17920mV)
    let expected_writes_max = vec![
        (0x6B, vec![Register::VsysMin as u8, 0xB3]), // 17920mV = raw 179 (0xB3)
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_vsys_min(VsysMin(17920)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (5000mV)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::VsysMin as u8, 0x32]), // 5000mV = raw 50
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_vsys_min(VsysMin(5000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_vsys_min() {
    // Test min value (3500mV)
    let expected_reads_min = vec![
        vec![0x23], // raw 35
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::VsysMin as u8]),
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let vsys_min = bq_min.read_vsys_min().await.unwrap();
    assert_eq!(vsys_min.0, 3500);
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (17920mV)
    let expected_reads_max = vec![
        vec![0xB3], // raw 179
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::VsysMin as u8]),
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let vsys_max = bq_max.read_vsys_min().await.unwrap();
    assert_eq!(vsys_max.0, 17900); // 17900mV = raw 179
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (5000mV)
    let expected_reads_mid = vec![
        vec![0x32], // raw 50
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::VsysMin as u8]),
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let vsys_mid = bq_mid.read_vsys_min().await.unwrap();
    assert_eq!(vsys_mid.0, 5000);
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_iin_host() {
    // Test min value (50mA)
    let expected_writes_min = vec![
        (0x6B, vec![Register::IinHost as u8, 0x00]), // 50mA = raw 0 (0x00)
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_iin_host(IinHost(50)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (3200mA)
    let expected_writes_max = vec![
        (0x6B, vec![Register::IinHost as u8, 0x3F]), // 3200mA = raw 63 (0x3F)
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_iin_host(IinHost(3200)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::IinHost as u8, 0x13]), // 1000mA = raw 19 (0x13)
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_iin_host(IinHost(1000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_iin_host() {
    // Test min value (50mA)
    let expected_reads_min = vec![
        vec![0x00], // raw 0 // 50mA = raw 0
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::IinHost as u8]),
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let iin_host = bq_min.read_iin_host().await.unwrap();
    assert_eq!(iin_host.0, 50); // 50mA = raw 0
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (3200mA)
    let expected_reads_max = vec![
        vec![0x3F], // raw 63 // 3200mA = raw 63
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::IinHost as u8]),
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let iin_host = bq_max.read_iin_host().await.unwrap();
    assert_eq!(iin_host.0, 3200); // 3200mA = raw 63
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_reads_mid = vec![
        vec![0x13], // raw 19 // 1000mA = raw 19
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::IinHost as u8]),
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let iin_host = bq_mid.read_iin_host().await.unwrap();
    assert_eq!(iin_host.0, 1000); // 1000mA = raw 19
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_iin_dpm() {
    // Test min value (50mA)
    let expected_writes_min = vec![
        (0x6B, vec![Register::IinDpm as u8, 0x00]), // 50mA = raw 0 (0x00)
    ];
    let i2c_min = MockI2c::new().with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    bq_min.set_iin_dpm(IinDpm(50)).await.unwrap();
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (3200mA)
    let expected_writes_max = vec![
        (0x6B, vec![Register::IinDpm as u8, 0x3F]), // 3200mA = raw 63 (0x3F)
    ];
    let i2c_max = MockI2c::new().with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    bq_max.set_iin_dpm(IinDpm(3200)).await.unwrap();
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_writes_mid = vec![
        (0x6B, vec![Register::IinDpm as u8, 0x13]), // 1000mA = raw 19 (0x13)
    ];
    let i2c_mid = MockI2c::new().with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    bq_mid.set_iin_dpm(IinDpm(1000)).await.unwrap();
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_iin_dpm() {
    // Test min value (50mA)
    let expected_reads_min = vec![
        vec![0x00], // raw 0 // 50mA = raw 0
    ];
    let expected_writes_min = vec![
        (0x6B, vec![Register::IinDpm as u8]),
    ];
    let i2c_min = MockI2c::new()
        .with_read_values(expected_reads_min)
        .with_expected_writes(expected_writes_min.into_iter().rev().collect());
    let mut bq_min = Bq25730::new(i2c_min, 0x6B);
    let iin_dpm = bq_min.read_iin_dpm().await.unwrap();
    assert_eq!(iin_dpm.0, 50); // 50mA = raw 0
    bq_min.i2c.assert_all_writes_performed();

    // Test max value (3200mA)
    let expected_reads_max = vec![
        vec![0x3F], // raw 63 // 3200mA = raw 63
    ];
    let expected_writes_max = vec![
        (0x6B, vec![Register::IinDpm as u8]),
    ];
    let i2c_max = MockI2c::new()
        .with_read_values(expected_reads_max)
        .with_expected_writes(expected_writes_max.into_iter().rev().collect());
    let mut bq_max = Bq25730::new(i2c_max, 0x6B);
    let iin_dpm = bq_max.read_iin_dpm().await.unwrap();
    assert_eq!(iin_dpm.0, 3200); // 3200mA = raw 63
    bq_max.i2c.assert_all_writes_performed();

    // Test middle value (1000mA)
    let expected_reads_mid = vec![
        vec![0x13], // raw 19 // 1000mA = raw 19
    ];
    let expected_writes_mid = vec![
        (0x6B, vec![Register::IinDpm as u8]),
    ];
    let i2c_mid = MockI2c::new()
        .with_read_values(expected_reads_mid)
        .with_expected_writes(expected_writes_mid.into_iter().rev().collect());
    let mut bq_mid = Bq25730::new(i2c_mid, 0x6B);
    let iin_dpm = bq_mid.read_iin_dpm().await.unwrap();
    assert_eq!(iin_dpm.0, 1000); // 1000mA = raw 19
    bq_mid.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charger_status() {
    // Test reading a specific status value
    let expected_lsb = registers::CHARGER_STATUS_FAULT_ACOV | registers::CHARGER_STATUS_FAULT_BATOC;
    let expected_msb = registers::CHARGER_STATUS_STAT_AC | registers::CHARGER_STATUS_ICO_DONE;

    let expected_reads = vec![
        vec![expected_lsb, expected_msb],
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargerStatus as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let status = bq.read_charger_status().await.unwrap();
    assert_eq!(status.stat_ac, (expected_msb & registers::CHARGER_STATUS_STAT_AC) != 0);
    assert_eq!(status.ico_done, (expected_msb & registers::CHARGER_STATUS_ICO_DONE) != 0);
    assert_eq!(status.in_vap, (expected_msb & registers::CHARGER_STATUS_IN_VAP) != 0);
    assert_eq!(status.in_vindpm, (expected_msb & registers::CHARGER_STATUS_IN_VINDPM) != 0);
    assert_eq!(status.in_iin_dpm, (expected_msb & registers::CHARGER_STATUS_IN_IIN_DPM) != 0);
    assert_eq!(status.in_fchrg, (expected_msb & registers::CHARGER_STATUS_IN_FCHRG) != 0);
    assert_eq!(status.in_pchrg, (expected_msb & registers::CHARGER_STATUS_IN_PCHRG) != 0);
    assert_eq!(status.in_otg, (expected_msb & registers::CHARGER_STATUS_IN_OTG) != 0);
    assert_eq!(status.fault_acov, (expected_lsb & registers::CHARGER_STATUS_FAULT_ACOV) != 0);
    assert_eq!(status.fault_batoc, (expected_lsb & registers::CHARGER_STATUS_FAULT_BATOC) != 0);
    assert_eq!(status.fault_acoc, (expected_lsb & registers::CHARGER_STATUS_FAULT_ACOC) != 0);
    assert_eq!(status.fault_sysovp, (expected_lsb & registers::CHARGER_STATUS_FAULT_SYSOVP) != 0);
    assert_eq!(status.fault_vsys_uvp, (expected_lsb & registers::CHARGER_STATUS_FAULT_VSYS_UVP) != 0);
    assert_eq!(status.fault_force_converter_off, (expected_lsb & registers::CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF) != 0);
    assert_eq!(status.fault_otg_ovp, (expected_lsb & registers::CHARGER_STATUS_FAULT_OTG_OVP) != 0);
    assert_eq!(status.fault_otg_uvp, (expected_lsb & registers::CHARGER_STATUS_FAULT_OTG_UVP) != 0);


    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_prochot_status() {
    // Test reading a specific prochot status value
    let expected_lsb = registers::PROCHOT_STATUS_STAT_VINDPM | registers::PROCHOT_STATUS_STAT_ICRIT | registers::PROCHOT_STATUS_STAT_BAT_REMOVAL;
    let expected_msb = registers::PROCHOT_STATUS_EN_PROCHOT_EXT | registers::PROCHOT_STATUS_PROCHOT_WIDTH | registers::PROCHOT_STATUS_PROCHOT_CLEAR | registers::PROCHOT_STATUS_STAT_VAP_FAIL | registers::PROCHOT_STATUS_STAT_EXIT_VAP;
    let expected_charge_option4_lsb = registers::CHARGE_OPTION4_STAT_IDCHG2 | registers::CHARGE_OPTION4_STAT_PTM;


    let expected_reads = vec![
        vec![expected_lsb, expected_msb], // ProchotStatus LSB/MSB
        vec![expected_charge_option4_lsb], // ChargeOption4 LSB
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ProchotStatus as u8]), // Corrected address (LSB address)
        (0x6B, vec![Register::ChargeOption4 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let status = bq.read_prochot_status().await.unwrap();
    assert_eq!(status.en_prochot_ext, (expected_msb & registers::PROCHOT_STATUS_EN_PROCHOT_EXT) != 0);
    assert_eq!(status.prochot_width, (expected_msb & registers::PROCHOT_STATUS_PROCHOT_WIDTH) >> 4);
    assert_eq!(status.prochot_clear, (expected_msb & registers::PROCHOT_STATUS_PROCHOT_CLEAR) != 0);
    assert_eq!(status.stat_vap_fail, (expected_msb & registers::PROCHOT_STATUS_STAT_VAP_FAIL) != 0);
    assert_eq!(status.stat_exit_vap, (expected_msb & registers::PROCHOT_STATUS_STAT_EXIT_VAP) != 0);
    assert_eq!(status.stat_vindpm, (expected_lsb & registers::PROCHOT_STATUS_STAT_VINDPM) != 0);
    assert_eq!(status.stat_comp, (expected_lsb & registers::PROCHOT_STATUS_STAT_COMP) != 0);
    assert_eq!(status.stat_icrit, (expected_lsb & registers::PROCHOT_STATUS_STAT_ICRIT) != 0);
    assert_eq!(status.stat_inom, (expected_lsb & registers::PROCHOT_STATUS_STAT_INOM) != 0);
    assert_eq!(status.stat_idchg1, (expected_lsb & registers::PROCHOT_STATUS_STAT_IDCHG1) != 0);
    assert_eq!(status.stat_vsys, (expected_lsb & registers::PROCHOT_STATUS_STAT_VSYS) != 0);
    assert_eq!(status.stat_bat_removal, (expected_lsb & registers::PROCHOT_STATUS_STAT_BAT_REMOVAL) != 0);
    assert_eq!(status.stat_adpt_removal, (expected_lsb & registers::PROCHOT_STATUS_STAT_ADPT_REMOVAL) != 0);
    assert_eq!(status.stat_idchg2, (expected_charge_option4_lsb & registers::CHARGE_OPTION4_STAT_IDCHG2) != 0);
    assert_eq!(status.stat_ptm, (expected_charge_option4_lsb & registers::CHARGE_OPTION4_STAT_PTM) != 0);


    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_adc_measurements() {
    // Test reading ADC measurements
    let expected_reads = vec![
        vec![0x01], // ADCPSYS
        vec![0x02], // ADCVBUS
        vec![0x03], // ADCIDCHG
        vec![0x04], // ADCICHG
        vec![0x05], // ADCCMPIN
        vec![0x06], // ADCIIN
        vec![0x07], // ADCVBAT
        vec![0x08], // ADCVSYS
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ADCPSYS as u8]),
        (0x6B, vec![Register::ADCVBUS as u8]),
        (0x6B, vec![Register::ADCIDCHG as u8]),
        (0x6B, vec![Register::ADCICHG as u8]),
        (0x6B, vec![Register::ADCCMPIN as u8]),
        (0x6B, vec![Register::ADCIIN as u8]),
        (0x6B, vec![Register::ADCVBAT as u8]),
        (0x6B, vec![Register::ADCVSYS as u8]),
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let measurements = bq.read_adc_measurements().await.unwrap();
    assert_eq!(measurements.psys.0, AdcPsys::from_register_value(0x01).0);
    assert_eq!(measurements.vbus.0, AdcVbus::from_register_value(0x02).0);
    assert_eq!(measurements.idchg.0, AdcIdchg::from_register_value(0x03).0);
    assert_eq!(measurements.ichg.0, AdcIchg::from_register_value(0x04).0);
    assert_eq!(measurements.cmpin.0, AdcCmpin::from_register_value(0x05).0);
    assert_eq!(measurements.iin.0, AdcIin::from_register_value(0x06).0);
    assert_eq!(measurements.vbat.0, AdcVbat::from_register_value(0x07).0);
    assert_eq!(measurements.vsys.0, AdcVsys::from_register_value(0x08).0);


    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_charge_option1() {
    // Test setting ChargeOption1 register
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption1 as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_charge_option1(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_option1() {
    // Test reading ChargeOption1 register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption1 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_charge_option1().await.unwrap();
    assert_eq!(lsb, 0xDD); // Should be MSB
    assert_eq!(msb, 0xCC); // Should be LSB

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_charge_option2() {
    // Test setting ChargeOption2 register
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption2 as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_charge_option2(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_option2() {
    // Test reading ChargeOption2 register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption2 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_charge_option2().await.unwrap();
    assert_eq!(lsb, 0xDD); // Should be MSB
    assert_eq!(msb, 0xCC); // Should be LSB

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_charge_option3() {
    // Test setting ChargeOption3 register
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption3 as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_charge_option3(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_option3() {
    // Test reading ChargeOption3 register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption3 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_charge_option3().await.unwrap();
    assert_eq!(lsb, 0xDD); // Should be MSB
    assert_eq!(msb, 0xCC); // Should be LSB

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_charge_option4() {
    // Test setting ChargeOption4 register
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption4 as u8, 0xAA]), // Example value
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_charge_option4(0xAA).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_charge_option4() {
    // Test reading ChargeOption4 register
    let expected_reads = vec![
        vec![0xCC], // Example value
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption4 as u8]), // Corrected address
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let value = bq.read_charge_option4().await.unwrap();
    assert_eq!(value, 0xCC);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_prochot_option0() {
    // Test setting ProchotOption0 register
    let expected_writes = vec![
        (0x6B, vec![Register::ProchotOption0 as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_prochot_option0(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_prochot_option0() {
    // Test reading ProchotOption0 register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ProchotOption0 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_prochot_option0().await.unwrap();
    assert_eq!(lsb, 0xCC);
    assert_eq!(msb, 0xDD);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_prochot_option1() {
    // Test setting ProchotOption1 register
    let expected_writes = vec![
        (0x6B, vec![Register::ProchotOption1 as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_prochot_option1(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_prochot_option1() {
    // Test reading ProchotOption1 register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ProchotOption1 as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_prochot_option1().await.unwrap();
    assert_eq!(lsb, 0xCC);
    assert_eq!(msb, 0xDD);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_adc_option() {
    // Test setting AdcOption register
    let expected_writes = vec![
        (0x6B, vec![Register::ADCOption as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_adc_option(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_adc_option() {
    // Test reading AdcOption register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ADCOption as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_adc_option().await.unwrap();
    assert_eq!(lsb, 0xDD); // Should be MSB
    assert_eq!(msb, 0xCC); // Should be LSB

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_set_vmin_active_protection() {
    // Test setting VminActiveProtection register
    let expected_writes = vec![
        (0x6B, vec![Register::VMINActiveProtection as u8, 0xAA, 0xBB]), // Example values
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    bq.set_vmin_active_protection(0xAA, 0xBB).await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_vmin_active_protection() {
    // Test reading VminActiveProtection register
    let expected_reads = vec![
        vec![0xCC, 0xDD], // Example values
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::VMINActiveProtection as u8]), // Corrected address (LSB address)
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let (lsb, msb) = bq.read_vmin_active_protection().await.unwrap();
    assert_eq!(lsb, 0xCC);
    assert_eq!(msb, 0xDD);

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_enter_ship_mode() {
    // Test entering ship mode
    let expected_writes = vec![
        // Write 0x0010 to ChargeOption4 (0x3C/0x3D)
        (0x6B, vec![Register::ChargeOption4 as u8, 0x10, 0x00]),
        // Write 0x0010 to ChargeOption3 (0x34/0x35)
        (0x6B, vec![Register::ChargeOption3 as u8, 0x10, 0x00]),
    ];
    let i2c = MockI2c::new().with_expected_writes(expected_writes.into_iter().rev().collect());
    let address = 0x6B; // Use a variable for address
    let mut bq = Bq25730::new(i2c, address); // Use the address variable

    bq.enter_ship_mode().await.unwrap();

    bq.i2c.assert_all_writes_performed();
}

#[tokio::test]
async fn test_read_register() {
    // Test reading a single register
    let expected_reads = vec![
        vec![0xAB], // Example value
    ];
    let expected_writes = vec![
        (0x6B, vec![Register::ChargeOption0 as u8]),
    ];
    let i2c = MockI2c::new()
        .with_read_values(expected_reads)
        .with_expected_writes(expected_writes.into_iter().rev().collect());
    let mut bq = Bq25730::new(i2c, 0x6B);

    let value = bq.read_register(Register::ChargeOption0).await.unwrap();
    assert_eq!(value, 0xAB);

    bq.i2c.assert_all_writes_performed();
}