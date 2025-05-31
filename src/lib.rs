#![no_std]
#[cfg(feature = "defmt")]
extern crate defmt; // Make defmt available for derive macros

use core::ops::{Deref, DerefMut};

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

pub mod data_types;
pub mod errors;
pub mod registers;
use crate::data_types::{
    AdcCmpin, AdcIchg, AdcIdchg, AdcIin, AdcMeasurements, AdcPsys, AdcVbat, AdcVbus, AdcVsys,
    ChargeCurrent, ChargeOption0, ChargeOption1, ChargeOption2, ChargeOption3, ChargeVoltage,
    ChargerStatus, IinDpm, IinHost, InputVoltage, OtgCurrent, OtgVoltage, ProchotStatus, VsysMin,
}; // Import BQ25730 data types
use crate::registers::{ChargeOption1Flags, ChargerStatusFaultFlags, ChargerStatusFlags}; // Import necessary flags
pub use errors::Error;
use registers::Register; // Use BQ25730 registers

/// The default I2C address of the BQ25730 chip.
pub const BQ25730_I2C_ADDRESS: u8 = 0x6B;

/// Trait for abstracting register access, with or without CRC.
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "RegisterAccess",),
    async(feature = "async", keep_self)
)]
#[allow(async_fn_in_trait)]
pub trait RegisterAccess<E> {
    /// The buffer type used for reading multiple registers.
    type ReadBuffer: Default + Extend<u8> + Deref<Target = [u8]> + DerefMut + Sized;
    /// The buffer type used for reading multiple registers.
    /// Reads a single register.
    async fn read_register(&mut self, reg: Register) -> Result<u8, Error<E>>;

    /// Reads multiple registers starting from `reg`.
    async fn read_registers(
        &mut self,
        reg: Register,
        len: usize,
    ) -> Result<Self::ReadBuffer, Error<E>>;

    /// Writes a single register.
    async fn write_register(&mut self, reg: Register, value: u8) -> Result<(), Error<E>>;

    /// Writes multiple registers starting from `reg`.
    async fn write_registers(&mut self, reg: Register, values: &[u8]) -> Result<(), Error<E>>;
}

/// BQ25730 driver
pub struct Bq25730<I2C>
where
    I2C: I2c + 'static, // Add 'static lifetime bound
{
    address: u8,
    pub i2c: I2C,             // Make i2c field public for testing
    cell_count: u8,           // 新增字段：电池节数
    rsns_rac_is_5m_ohm: bool, // 存储 RSNS_RAC 的值，true 表示 5mOhm，false 表示 10mOhm
}

/// Trait for abstracting register access, with or without CRC.
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Bq25730",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Bq25730<I2C>
where
    I2C: I2c<Error = E> + Send,
{
    /// Creates a new instance of the BQ25730 driver.
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral.
    /// * `address` - The I2C address of the BQ25730 chip.
    /// * `cell_count` - The number of battery cells (e.g., 4 for 4S, 5 for 5S).
    pub fn new(i2c: I2C, address: u8, cell_count: u8) -> Self {
        Self {
            address,
            i2c,
            cell_count,
            rsns_rac_is_5m_ohm: false, // Initialize with a default value
        }
    }

    /// Returns the I2C address of the BQ25730 chip.
    pub fn address(&self) -> u8 {
        self.address
    }
}

/// Trait for abstracting register access, with or without CRC.
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Bq25730",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> RegisterAccess<E> for Bq25730<I2C>
where
    I2C: I2c<Error = E> + Send,
{
    type ReadBuffer = heapless::Vec<u8, 30>; // Same buffer type as Enabled mode

    async fn read_register(&mut self, reg: Register) -> Result<u8, Error<E>> {
        let mut data = [0u8; 1];
        self.i2c
            .write_read(self.address, &[reg as u8], &mut data)
            .await
            .map_err(Error::I2c)?;
        Ok(data[0])
    }

    async fn read_registers(
        &mut self,
        reg: Register,
        len: usize,
    ) -> Result<Self::ReadBuffer, Error<E>> {
        if len == 0 || len > 30 {
            #[cfg(feature = "defmt")]
            defmt::error!("Invalid read length: {}", len);
            return Err(Error::InvalidData);
        }

        let mut buffer: heapless::Vec<u8, 30> = heapless::Vec::new();
        buffer.resize(len, 0).map_err(|_| Error::InvalidData)?;

        self.i2c
            .write_read(self.address, &[reg as u8], &mut buffer)
            .await
            .map_err(Error::I2c)?;

        Ok(buffer)
    }

    async fn write_register(&mut self, reg: Register, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[reg as u8, value])
            .await
            .map_err(Error::I2c)
    }

    async fn write_registers(&mut self, reg: Register, values: &[u8]) -> Result<(), Error<E>> {
        if values.is_empty() || values.len() > 30 {
            #[cfg(feature = "defmt")]
            defmt::error!("Invalid write length: {}", values.len());
            return Err(Error::InvalidData);
        }

        let mut data_to_write = heapless::Vec::<u8, 31>::new(); // register + values
        data_to_write
            .push(reg as u8)
            .map_err(|_| Error::InvalidData)?;
        data_to_write
            .extend_from_slice(values)
            .map_err(|_| Error::InvalidData)?;

        self.i2c
            .write(self.address, &data_to_write)
            .await
            .map_err(Error::I2c)
    }
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Bq25730",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Bq25730<I2C>
where
    I2C: I2c<Error = E> + Send,
    Self: RegisterAccess<E>,
{
    /// Initializes the BQ25730 charger.
    /// This function sets basic configuration and clears status flags.
    pub async fn init(&mut self) -> Result<(), Error<E>> {
        // Example initialization steps (refer to datasheet for recommended sequence)

        // Read ChargeOption1 to determine RSNS_RAC setting
        let charge_option1 = self.read_charge_option1().await?;
        self.rsns_rac_is_5m_ohm = charge_option1
            .msb_flags
            .contains(registers::ChargeOption1MsbFlags::RSNS_RAC);

        // Set default ChargeOption0 (e.g., enable IIN_DPM, disable Charge Inhibit)
        // Assuming default values for other bits for now.
        // Read current ChargeOption0 values to preserve other settings
        let charge_option0 = self.read_charge_option0().await?;
        // Set EN_IIN_DPM bit (if not already set by default)
        // charge_option0.en_iin_dpm = true; // This is already handled by read_charge_option0
        // Write the modified ChargeOption0
        self.set_charge_option0(charge_option0).await?;

        // Set default Input Current Limit (e.g., 3.2A, which is 3200mA)
        // IIN_HOST LSB is 100mA, offset is 100mA. 3200mA = (raw * 100) + 100 => raw = 31
        let iin_host_ma = IinHost(3200);
        self.set_iin_host(iin_host_ma).await?;

        // Set default Minimum System Voltage (e.g., 3.5V, which is 3500mV)
        // VSYS_MIN LSB is 100mV. 3500mV = raw * 100 => raw = 35
        let vsys_min_mv = VsysMin(3500);
        self.set_vsys_min(vsys_min_mv).await?;

        // Clear all status flags by writing 1s to the SysStat register
        // Refer to datasheet for which flags are clearable by writing 1.
        // Assuming all bits in SysStat are clearable by writing 1 for now.
        // For now, let's clear the fault flags in ChargerStatus LSB.
        // Clear all status flags
        // Clear all status flags by writing 1s to the SysStat register
        // Read current ChargerStatus (0x20/0x21)
        let mut charger_status = self.read_charger_status().await?;
        // Clear Fault SYSOVP and Fault VSYS_UVP by setting them to 0
        charger_status.fault_flags.remove(
            registers::ChargerStatusFaultFlags::FAULT_SYSOVP
                | registers::ChargerStatusFaultFlags::FAULT_VSYS_UVP,
        );
        self.set_charger_status(charger_status).await?;

        Ok(())
    }

    // TODO: Implement core functions based on BQ25730 datasheet
    // - Register read/write functions for specific data types (already started)
    // - Charging control
    // - OTG control
    // - Status and measurement reading (using the new data types) (already started)
    // - Protection configuration
    // - Other features (DPM, ICO, Peak Power, PROCHOT)

    /// Reads the Charger Status register.
    pub async fn read_charger_status(&mut self) -> Result<ChargerStatus, Error<E>> {
        // ChargerStatus is a 16-bit register (0x20/0x21). Read from LSB address (0x20).
        let raw_status = self.read_registers(Register::ChargerStatus, 2).await?;
        let lsb = raw_status.as_ref()[0];
        let msb = raw_status.as_ref()[1];

        Ok(ChargerStatus {
            status_flags: ChargerStatusFlags::from_bits_truncate(msb),
            fault_flags: ChargerStatusFaultFlags::from_bits_truncate(lsb),
        })
    }

    /// Sets the Charger Status register.
    pub async fn set_charger_status(&mut self, status: ChargerStatus) -> Result<(), Error<E>> {
        let (lsb, msb) = status.to_msb_lsb_bytes();
        self.write_registers(Register::ChargerStatus, &[lsb, msb])
            .await
    }

    /// Reads the Prochot Status register.
    pub async fn read_prochot_status(&mut self) -> Result<ProchotStatus, Error<E>> {
        let raw_status = self.read_registers(Register::ProchotStatus, 2).await?; // Read from LSB address (0x22)
                                                                                 // let lsb = raw_status.as_ref()[0]; // ProchotStatus LSB (0x22)
                                                                                 // let msb = raw_status.as_ref()[1]; // ProchotStatus MSB (0x23)

        // Read ChargeOption4 LSB (0x3C) for stat_idchg2 and stat_ptm
        // NOTE: ChargeOption4 is not defined in data_types.rs, this might be a placeholder or error in original code.
        // Assuming for now that these bits are part of ProchotStatus or another register.
        // For now, we will remove the dependency on ChargeOption4.
        // let raw_charge_option4 = self.read_register(Register::ChargeOption4).await?; // Read single 8-bit register
        // let charge_option4_lsb = raw_charge_option4; // ChargeOption4 LSB (0x3C)

        Ok(ProchotStatus {
            msb_flags: registers::ProchotStatusMsbFlags::from_bits_truncate(raw_status.as_ref()[1]),
            lsb_flags: registers::ProchotStatusFlags::from_bits_truncate(raw_status.as_ref()[0]),
            prochot_width: ((raw_status.as_ref()[1] >> 4) & 0x03), // Bits 5:4 of MSB
        })
    }
    /// Reads all ADC measurement registers.
    pub async fn read_adc_measurements(&mut self) -> Result<AdcMeasurements, Error<E>> {
        // Determine ADC offset based on cell count
        let offset_mv = match self.cell_count {
            1..=4 => 2880, // 2.88V for 1S-4S
            5 => 8160,     // 8.16V for 5S
            _ => {
                #[cfg(feature = "defmt")]
                defmt::warn!(
                    "Unsupported cell count: {}. Using 2.88V offset.",
                    self.cell_count
                );
                2880 // Default to 2.88V for unsupported cell counts
            }
        };

        // Read each 8-bit ADC register individually
        let psys_raw = self.read_registers(Register::ADCPSYS, 1).await?;
        let vbus_raw = self.read_registers(Register::ADCVBUS, 1).await?;
        let idchg_raw = self.read_registers(Register::ADCIDCHG, 1).await?;
        let ichg_raw = self.read_registers(Register::ADCICHG, 1).await?;
        let cmpin_raw = self.read_registers(Register::ADCCMPIN, 1).await?;
        let iin_raw = self.read_registers(Register::ADCIIN, 1).await?;
        let vbat_raw = self.read_registers(Register::ADCVBAT, 2).await?; // Reads 0x2C (LSB), 0x2D (MSB)
        // ADCVSYS data is in register 0x2D (which is ADCVBAT_MSB).
        // So, vsys_msb_raw is the second byte read for ADCVBAT.
        let vsys_msb_raw = vbat_raw.as_ref()[1]; // This is the content of 0x2D

        Ok(AdcMeasurements {
            vbat: AdcVbat::from_register_value(vbat_raw.as_ref()[0], vbat_raw.as_ref()[1], offset_mv), // Use from_register_value
            vsys: AdcVsys::from_register_value(0, vsys_msb_raw, offset_mv), // LSB for VSYS ADC is not used from a separate reg
            ichg: AdcIchg::from_u8(ichg_raw[0]),
            idchg: AdcIdchg::from_u8(idchg_raw[0]),
            iin: AdcIin::from_u8(iin_raw[0], self.rsns_rac_is_5m_ohm),
            psys: AdcPsys::from_u8(psys_raw[0]),
            vbus: AdcVbus::from_u8(vbus_raw[0]),
            cmpin: AdcCmpin::from_u8(cmpin_raw[0]),
        })
    }

    /// Sets the ADCOption register.
    pub async fn set_adc_option(&mut self, options: data_types::AdcOption) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(registers::Register::ADCOption, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ADCOption register.
    pub async fn read_adc_option(&mut self) -> Result<data_types::AdcOption, Error<E>> {
        let raw_options = self
            .read_registers(registers::Register::ADCOption, 2)
            .await?;
        Ok(data_types::AdcOption::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Reads the Charge Current register and returns the value in mA.
    pub async fn read_charge_current(&mut self) -> Result<ChargeCurrent, Error<E>> {
        // ChargeCurrent is a 13-bit register (03/02h). Read from LSB address (0x02).
        let raw_current = self.read_registers(Register::ChargeCurrent, 2).await?;
        Ok(ChargeCurrent::from_u16(u16::from_le_bytes([
            raw_current.as_ref()[0],
            raw_current.as_ref()[1],
        ])))
    }

    /// Writes the Charge Current register with the value in mA.
    pub async fn set_charge_current(&mut self, current: ChargeCurrent) -> Result<(), Error<E>> {
        let raw_value = current.to_u16();
        self.write_registers(Register::ChargeCurrent, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the Charge Voltage register and returns the value in mV.
    /// Reads the Charge Voltage register and returns the value in mV.
    pub async fn read_charge_voltage(&mut self) -> Result<ChargeVoltage, Error<E>> {
        // ChargeVoltage is a 12-bit register (05/04h). Read from LSB address (0x04).
        let raw_voltage = self.read_registers(Register::ChargeVoltage, 2).await?;
        Ok(ChargeVoltage::from_u16(u16::from_le_bytes([
            raw_voltage.as_ref()[0],
            raw_voltage.as_ref()[1],
        ])))
    }

    /// Writes the Charge Voltage register with the value in mV.
    pub async fn set_charge_voltage(&mut self, voltage: ChargeVoltage) -> Result<(), Error<E>> {
        let raw_value = voltage.to_u16();
        self.write_registers(Register::ChargeVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the OTG Voltage register and returns the value in mV.
    pub async fn read_otg_voltage(&mut self) -> Result<OtgVoltage, Error<E>> {
        // OTGVoltage is an 11-bit register (07/06h). Read from LSB address (0x06).
        let raw_voltage = self.read_registers(Register::OTGVoltage, 2).await?;
        Ok(OtgVoltage::from_u16(u16::from_le_bytes([
            raw_voltage.as_ref()[0],
            raw_voltage.as_ref()[1],
        ])))
    }

    /// Writes the OTG Voltage register with the value in mV.
    pub async fn set_otg_voltage(&mut self, voltage: OtgVoltage) -> Result<(), Error<E>> {
        let raw_value = voltage.to_u16();
        self.write_registers(Register::OTGVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the OTG Current register and returns the value in mA.
    pub async fn read_otg_current(&mut self) -> Result<OtgCurrent, Error<E>> {
        let raw_current = self.read_registers(Register::OTGCurrent, 2).await?;
        Ok(OtgCurrent::from_u16(u16::from_le_bytes([
            raw_current.as_ref()[0],
            raw_current.as_ref()[1],
        ])))
    }

    /// Writes the OTG Current register with the value in mA.
    pub async fn set_otg_current(&mut self, current: OtgCurrent) -> Result<(), Error<E>> {
        let raw_value = current.to_u16();
        self.write_registers(Register::OTGCurrent, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the Input Voltage register and returns the value in mV.
    pub async fn read_input_voltage(&mut self) -> Result<InputVoltage, Error<E>> {
        // InputVoltage is a 9-bit register (0B/0Ah). Read from LSB address (0x0A).
        let raw_voltage = self.read_registers(Register::InputVoltage, 2).await?;
        Ok(InputVoltage::from_u16(u16::from_le_bytes([
            raw_voltage.as_ref()[0],
            raw_voltage.as_ref()[1],
        ])))
    }
    /// Writes the Input Voltage register with the value in mV.
    pub async fn set_input_voltage(&mut self, voltage: InputVoltage) -> Result<(), Error<E>> {
        let raw_value = voltage.to_u16();
        self.write_registers(Register::InputVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the Minimum System Voltage register and returns the value in mV.
    pub async fn read_vsys_min(&mut self) -> Result<VsysMin, Error<E>> {
        let raw_voltage = self.read_registers(Register::VsysMin, 2).await?;
        Ok(VsysMin::from_u16(u16::from_le_bytes([
            raw_voltage[0],
            raw_voltage[1],
        ])))
    }

    /// Writes the Minimum System Voltage register with the value in mV.
    pub async fn set_vsys_min(&mut self, voltage: VsysMin) -> Result<(), Error<E>> {
        let raw_value = voltage.to_u16();
        self.write_registers(Register::VsysMin, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the IIN_HOST register and returns the value in mA.
    pub async fn read_iin_host(&mut self) -> Result<IinHost, Error<E>> {
        let raw_current = self.read_registers(Register::IinHost, 2).await?;
        Ok(IinHost::from_u16(u16::from_le_bytes([
            raw_current[0],
            raw_current[1],
        ])))
    }

    /// Writes the IIN_HOST register with the value in mA.
    pub async fn set_iin_host(&mut self, current: IinHost) -> Result<(), Error<E>> {
        let raw_value = current.to_u16();
        self.write_registers(Register::IinHost, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the IIN_DPM register and returns the value in mA.
    pub async fn read_iin_dpm(&mut self) -> Result<IinDpm, Error<E>> {
        let raw_current = self.read_registers(Register::IinDpm, 2).await?;
        Ok(IinDpm::from_u16(u16::from_le_bytes([
            raw_current[0],
            raw_current[1],
        ])))
    }

    /// Writes the IIN_DPM register with the value in mA.
    pub async fn set_iin_dpm(&mut self, current: IinDpm) -> Result<(), Error<E>> {
        let raw_value = current.to_u16();
        self.write_registers(Register::IinDpm, &raw_value.to_le_bytes())
            .await
    }

    /// Sets the ChargeOption0 register.
    pub async fn set_charge_option0(&mut self, options: ChargeOption0) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::ChargeOption0, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ChargeOption0 register.
    pub async fn read_charge_option0(&mut self) -> Result<ChargeOption0, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption0, 2).await?;
        Ok(ChargeOption0::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Sets the ChargeOption1 register.
    pub async fn set_charge_option1(&mut self, options: ChargeOption1) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::ChargeOption1, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ChargeOption1 register.
    pub async fn read_charge_option1(&mut self) -> Result<ChargeOption1, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption1, 2).await?;
        Ok(ChargeOption1::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Sets the ChargeOption2 register.
    pub async fn set_charge_option2(&mut self, options: ChargeOption2) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::ChargeOption2, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ChargeOption2 register.
    pub async fn read_charge_option2(&mut self) -> Result<ChargeOption2, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption2, 2).await?;
        Ok(ChargeOption2::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Sets the ChargeOption3 register.
    pub async fn set_charge_option3(&mut self, options: ChargeOption3) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::ChargeOption3, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ChargeOption3 register.
    pub async fn read_charge_option3(&mut self) -> Result<ChargeOption3, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption3, 2).await?;
        Ok(ChargeOption3::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Enters ship mode.
    /// This function sets the SHIP_MODE bit in ChargeOption1 register.
    pub async fn enter_ship_mode(&mut self) -> Result<(), Error<E>> {
        // Read current ChargeOption1 values to preserve other settings
        let mut charge_option1 = self.read_charge_option1().await?;
        // Set EN_SHIP_DCHG bit
        charge_option1
            .lsb_flags
            .insert(ChargeOption1Flags::EN_SHIP_DCHG);
        // Write the modified ChargeOption1
        self.set_charge_option1(charge_option1).await?;

        Ok(())
    }
}
