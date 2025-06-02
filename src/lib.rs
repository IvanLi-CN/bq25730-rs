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
    AdcCmpin,
    AdcIchg,
    AdcIdchg,
    AdcIin,
    AdcMeasurements,
    AdcPsys,
    AdcVbat,
    AdcVbus,
    AdcVsys,
    // ChargeCurrentSetting, // Updated type
    ChargeOption0,
    ChargeOption1,
    ChargeOption2,
    ChargeOption3,
    ChargeOption4,
    // ChargeVoltageSetting, // Updated type
    ChargerStatus,
    IinDpm,
    // IinHostSetting, // Updated type - unused direct import
    // InputVoltageSetting, // Updated type - unused direct import
    // OtgCurrentSetting, // Updated type - unused direct import
    // OtgVoltageSetting, // Refactored - unused direct import
    ProchotStatus,
    // VsysMinSetting, // Updated type - unused direct import
    VminActiveProtection,
};
use crate::registers::{ChargeOption1Flags, ChargerStatusFaultFlags, ChargerStatusFlags};
pub use data_types::{Config, SenseResistorValue};
pub use errors::Error;
use registers::Register; // Re-export Config and SenseResistorValue

// SenseResistorValue enum is now defined in data_types.rs

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

    /// Writes multiple registers, potentially non-contiguous.
    /// Each tuple in `values` is (Register, value).
    async fn write_registers_bulk(&mut self, values: &[(Register, u8)]) -> Result<(), Error<E>>;
}

/// BQ25730 driver
pub struct Bq25730<I2C>
where
    I2C: I2c + 'static,
{
    address: u8,
    pub i2c: I2C,
    config: Config, // Replaced cell_count, rsns_bat, rsns_ac
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
    /// * `config` - The charger configuration.
    pub fn new(i2c: I2C, address: u8, config: Config) -> Self {
        Self {
            address,
            i2c,
            config,
        }
    }

    /// Returns the I2C address of the BQ25730 chip.
    pub fn address(&self) -> u8 {
        self.address
    }

    /// Returns a reference to the current configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Allows mutable access to the configuration.
    /// Be cautious when modifying the config after initialization,
    /// as some settings might require re-initialization or specific sequences.
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
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
    type ReadBuffer = heapless::Vec<u8, 30>;

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

        let mut data_to_write = heapless::Vec::<u8, 31>::new();
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

    async fn write_registers_bulk(&mut self, values: &[(Register, u8)]) -> Result<(), Error<E>> {
        // This basic implementation writes them one by one.
        // For true bulk write of contiguous registers, a different approach or I2C controller feature might be needed.
        // For now, this ensures correctness. If performance becomes an issue, this can be revisited.
        for &(reg, val) in values {
            self.write_register(reg, val).await?;
        }
        Ok(())
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
    /// Initializes the BQ25730 charger using the provided configuration.
    pub async fn init(&mut self) -> Result<(), Error<E>> {
        // Write ChargeOption0, ChargeCurrent, ChargeVoltage, OTGVoltage, and OTGCurrent (Registers 0x00-0x09)
        // These are contiguous registers.
        let charge_option0_bytes = self.config.charge_option0.to_msb_lsb_bytes();
        let (cc_lsb, cc_msb) = self.config.charge_current.to_msb_lsb_bytes();
        let (cv_lsb, cv_msb) = self.config.charge_voltage.to_msb_lsb_bytes();
        let (otg_v_lsb, otg_v_msb) = self.config.otg_voltage.to_msb_lsb_bytes();
        let (otg_c_lsb, otg_c_msb) = self.config.otg_current.to_msb_lsb_bytes();

        self.write_registers(
            Register::ChargeOption0,
            &[
                charge_option0_bytes.0, // ChargeOption0 LSB (0x00)
                charge_option0_bytes.1, // ChargeOption0 MSB (0x01)
                cc_lsb,                 // ChargeCurrent LSB (0x02)
                cc_msb,                 // ChargeCurrent MSB (0x03)
                cv_lsb,                 // ChargeVoltage LSB (0x04)
                cv_msb,                 // ChargeVoltage MSB (0x05)
                otg_v_lsb,              // OTGVoltage LSB (0x06)
                otg_v_msb,              // OTGVoltage MSB (0x07)
                otg_c_lsb,              // OTGCurrent LSB (0x08) - Should be 0x00
                otg_c_msb,              // OTGCurrent MSB (0x09)
            ],
        )
        .await?;

        // Write ChargeOption1 (Registers 0x30-0x31) - Not contiguous with the previous block
        let (lsb_co1, msb_co1) = self.config.charge_option1.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption1, &[lsb_co1, msb_co1])
            .await?;

        // Group 2: InputVoltage, VsysMin, IinHost (Registers 0x0A-0x0F)
        // These are contiguous registers.
        let (iv_lsb, iv_msb) = self.config.input_voltage.to_msb_lsb_bytes();
        let (vm_lsb, vm_msb) = self.config.vsys_min.to_msb_lsb_bytes();
        let (ih_lsb, ih_msb) = self.config.iin_host.to_msb_lsb_bytes(self.config.rsns_ac);
        self.write_registers(
            Register::InputVoltage, // Starts at 0x0A
            &[
                iv_lsb, // InputVoltage LSB (0x0A)
                iv_msb, // InputVoltage MSB (0x0B)
                vm_lsb, // VsysMin LSB (0x0C) - Should be 0x00
                vm_msb, // VsysMin MSB (0x0D)
                ih_lsb, // IinHost LSB (0x0E)
                ih_msb, // IinHost MSB (0x0F)
            ],
        )
        .await?;

        // Write ChargeOption3 (Registers 0x34-0x35)
        let (lsb_co3, msb_co3) = self.config.charge_option3.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption3, &[lsb_co3, msb_co3])
            .await?;

        // Write ChargeOption4 and VminActiveProtection (Registers 0x3C-0x3F)
        // These are contiguous registers.
        let (lsb_co4, msb_co4) = self.config.charge_option4.to_msb_lsb_bytes();
        let (lsb_vmin, msb_vmin) = self.config.vmin_active_protection.to_msb_lsb_bytes();
        self.write_registers(
            Register::ChargeOption4,
            &[
                lsb_co4,  // ChargeOption4 LSB (0x3C)
                msb_co4,  // ChargeOption4 MSB (0x3D)
                lsb_vmin, // VMINActiveProtection LSB (0x3E)
                msb_vmin, // VMINActiveProtection MSB (0x3F)
            ],
        )
        .await?;

        // Clear SYSOVP and VSYS_UVP faults from ChargerStatus
        let mut charger_status = self.read_charger_status().await?;
        charger_status.fault_flags.remove(
            registers::ChargerStatusFaultFlags::FAULT_SYSOVP
                | registers::ChargerStatusFaultFlags::FAULT_VSYS_UVP,
        );
        self.set_charger_status(charger_status).await?;

        Ok(())
    }

    /// Reads the Charger Status register.
    pub async fn read_charger_status(&mut self) -> Result<ChargerStatus, Error<E>> {
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
        let raw_status = self.read_registers(Register::ProchotStatus, 2).await?;
        Ok(ProchotStatus {
            msb_flags: registers::ProchotStatusMsbFlags::from_bits_truncate(raw_status.as_ref()[1]),
            lsb_flags: registers::ProchotStatusFlags::from_bits_truncate(raw_status.as_ref()[0]),
            prochot_width: ((raw_status.as_ref()[1] >> 4) & 0x03),
        })
    }
    /// Reads all ADC measurement registers.
    pub async fn read_adc_measurements(&mut self) -> Result<AdcMeasurements, Error<E>> {
        // Determine ADC offset based on cell count from config (if available and reliable)
        // For now, assuming a fixed cell count or a way to get it.
        // The `Config` struct itself doesn't store cell_count directly,
        // but its default values are derived from it.
        // This part might need adjustment if cell_count isn't fixed or passed differently.
        // For simplicity, let's assume a default or a way to access it.
        // A placeholder for cell_count logic:
        let cell_count_for_adc = 4; // Placeholder, ideally from config or a reliable source

        let offset_mv = match cell_count_for_adc {
            1..=4 => 2880,
            5 => 8160,
            _ => {
                #[cfg(feature = "defmt")]
                defmt::warn!(
                    "Unsupported cell count for ADC offset: {}. Using 2.88V offset.",
                    cell_count_for_adc
                );
                2880
            }
        };
        let adc_data_raw = self.read_registers(Register::ADCPSYS, 8).await?;
        Ok(AdcMeasurements {
            vbat: AdcVbat::from_register_value(
                adc_data_raw.as_ref()[6], // LSB for ADCVBAT is 0x2C
                adc_data_raw.as_ref()[7], // MSB for ADCVBAT is 0x2D
                offset_mv,
            ),
            // ADCVSYS is also at 0x2D (MSB) and 0x2C (LSB, though LSB is often 0 for 8-bit ADCs in 2-byte reads)
            psys: AdcPsys::from_u8(adc_data_raw.as_ref()[0]), // ADCPSYS at 0x26
            vbus: AdcVbus::from_u8(adc_data_raw.as_ref()[1]), // ADCVBUS at 0x27
            idchg: AdcIdchg::from_raw(adc_data_raw.as_ref()[2], self.config.rsns_bat), // ADCIDCHG at 0x28
            ichg: AdcIchg::from_raw(adc_data_raw.as_ref()[3], self.config.rsns_bat), // ADCICHG at 0x29
            cmpin: AdcCmpin::from_u8(adc_data_raw.as_ref()[4]), // ADCCMPIN at 0x2A
            iin: AdcIin::from_raw(adc_data_raw.as_ref()[5], self.config.rsns_ac), // ADCIIN at 0x2B
            vsys: AdcVsys::from_register_value(0, adc_data_raw.as_ref()[7], offset_mv), // Assuming MSB is at index 7 (0x2D)
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

    /// Reads the Charge Current register and returns the setting.
    pub async fn read_charge_current_setting(
        &mut self,
    ) -> Result<data_types::ChargeCurrentSetting, Error<E>> {
        let raw_value_bytes = self.read_registers(Register::ChargeCurrent, 2).await?;
        let raw_value =
            u16::from_le_bytes([raw_value_bytes.as_ref()[0], raw_value_bytes.as_ref()[1]]);
        Ok(data_types::ChargeCurrentSetting::from_raw(
            raw_value,
            self.config.rsns_bat,
        ))
    }

    /// Writes the Charge Current register with the setting.
    pub async fn set_charge_current_setting(
        &mut self,
        current: data_types::ChargeCurrentSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = current.to_raw();
        self.write_registers(Register::ChargeCurrent, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the Charge Voltage register and returns the setting.
    pub async fn read_charge_voltage_setting(
        &mut self,
    ) -> Result<data_types::ChargeVoltageSetting, Error<E>> {
        let raw_voltage_bytes = self.read_registers(Register::ChargeVoltage, 2).await?;
        let raw_voltage =
            u16::from_le_bytes([raw_voltage_bytes.as_ref()[0], raw_voltage_bytes.as_ref()[1]]);
        Ok(data_types::ChargeVoltageSetting::from_raw(
            raw_voltage,
            None,
        ))
    }

    /// Writes the Charge Voltage register with the setting.
    pub async fn set_charge_voltage_setting(
        &mut self,
        voltage: data_types::ChargeVoltageSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = voltage.to_raw();
        self.write_registers(Register::ChargeVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the OTG Voltage register and returns the setting.
    pub async fn read_otg_voltage_setting(
        &mut self,
    ) -> Result<data_types::OtgVoltageSetting, Error<E>> {
        let raw_voltage_bytes = self.read_registers(Register::OTGVoltage, 2).await?;
        let raw_voltage =
            u16::from_le_bytes([raw_voltage_bytes.as_ref()[0], raw_voltage_bytes.as_ref()[1]]);
        Ok(data_types::OtgVoltageSetting::from_raw(raw_voltage))
    }

    /// Writes the OTG Voltage register with the setting.
    pub async fn set_otg_voltage_setting(
        &mut self,
        voltage: data_types::OtgVoltageSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = voltage.to_raw();
        self.write_registers(Register::OTGVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the OTG Current register (REG0x09/08h) and returns the setting.
    /// REG0x08h (LSB) is reserved. REG0x09h (MSB) bits 6:0 define the current.
    pub async fn read_otg_current_setting(
        &mut self,
    ) -> Result<data_types::OtgCurrentSetting, Error<E>> {
        let msb_byte = self.read_register(Register::OTGCurrentMsb).await?;
        let raw_7bit = msb_byte & 0x7F; // Mask to get D6-D0
        Ok(data_types::OtgCurrentSetting::from_raw(
            raw_7bit,
            self.config.rsns_bat,
        ))
    }

    /// Writes the OTG Current register (REG0x09/08h) with the setting.
    /// REG0x08h (LSB) is reserved and written as 0x00.
    /// REG0x09h (MSB) bits 6:0 are set based on the current setting.
    pub async fn set_otg_current_setting(
        &mut self,
        current: data_types::OtgCurrentSetting,
    ) -> Result<(), Error<E>> {
        let raw_7bit_current = current.to_raw();
        // Read the current MSB value to preserve other bits (e.g., bit 7 if it's used for something else, though datasheet says reserved)
        let mut msb_val = self.read_register(Register::OTGCurrentMsb).await?;
        msb_val &= !0x7F; // Clear bits D6-D0
        msb_val |= raw_7bit_current & 0x7F; // Set new current bits

        // Write both LSB (0x00) and MSB
        self.write_registers(Register::OTGCurrent, &[0x00, msb_val])
            .await
    }

    /// Reads the Input Voltage (VINDPM) register and returns the setting.
    pub async fn read_input_voltage_setting(
        &mut self,
    ) -> Result<data_types::InputVoltageSetting, Error<E>> {
        let raw_voltage_bytes = self.read_registers(Register::InputVoltage, 2).await?;
        let raw_voltage =
            u16::from_le_bytes([raw_voltage_bytes.as_ref()[0], raw_voltage_bytes.as_ref()[1]]);
        Ok(data_types::InputVoltageSetting::from_raw(raw_voltage))
    }

    /// Writes the Input Voltage (VINDPM) register with the setting.
    pub async fn set_input_voltage_setting(
        &mut self,
        voltage: data_types::InputVoltageSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = voltage.to_raw();
        self.write_registers(Register::InputVoltage, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the Minimum System Voltage (VSYS_MIN) register and returns the setting.
    pub async fn read_vsys_min_setting(&mut self) -> Result<data_types::VsysMinSetting, Error<E>> {
        let raw_voltage_bytes = self.read_registers(Register::VsysMin, 2).await?;
        let raw_voltage =
            u16::from_le_bytes([raw_voltage_bytes.as_ref()[0], raw_voltage_bytes.as_ref()[1]]);
        Ok(data_types::VsysMinSetting::from_raw(raw_voltage))
    }

    /// Writes the Minimum System Voltage (VSYS_MIN) register with the setting.
    pub async fn set_vsys_min_setting(
        &mut self,
        voltage: data_types::VsysMinSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = voltage.to_raw();
        self.write_registers(Register::VsysMin, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the IIN_HOST register and returns the setting.
    pub async fn read_iin_host_setting(&mut self) -> Result<data_types::IinHostSetting, Error<E>> {
        let raw_value_bytes = self.read_registers(Register::IinHost, 2).await?;
        let raw_value =
            u16::from_le_bytes([raw_value_bytes.as_ref()[0], raw_value_bytes.as_ref()[1]]);
        Ok(data_types::IinHostSetting::from_raw(
            raw_value,
            self.config.rsns_ac,
        ))
    }

    /// Writes the IIN_HOST register with the setting.
    pub async fn set_iin_host_setting(
        &mut self,
        current: data_types::IinHostSetting,
    ) -> Result<(), Error<E>> {
        let raw_value = current.to_raw(self.config.rsns_ac);
        self.write_registers(Register::IinHost, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the IIN_DPM register and returns the value in mA.
    pub async fn read_iin_dpm(&mut self) -> Result<IinDpm, Error<E>> {
        let msb_byte = self.read_register(Register::IinDpmMsb).await?;
        let raw_7bit = msb_byte & 0x7F;
        Ok(IinDpm::from_raw(raw_7bit, self.config.rsns_ac))
    }

    /// Writes the IIN_DPM register with the value in mA.
    pub async fn set_iin_dpm(&mut self, current: IinDpm) -> Result<(), Error<E>> {
        let raw_7bit = current.to_raw();
        let mut msb_val = self.read_register(Register::IinDpmMsb).await?;
        msb_val &= !0x7F;
        msb_val |= raw_7bit & 0x7F;
        self.write_register(Register::IinDpmMsb, msb_val).await
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

    /// Sets the ChargeOption4 register.
    pub async fn set_charge_option4(&mut self, options: ChargeOption4) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::ChargeOption4, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the ChargeOption4 register.
    pub async fn read_charge_option4(&mut self) -> Result<ChargeOption4, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption4, 2).await?;
        Ok(ChargeOption4::from_u16(u16::from_le_bytes([
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

    /// Sets the VminActiveProtection register.
    pub async fn set_vmin_active_protection(
        &mut self,
        options: VminActiveProtection,
    ) -> Result<(), Error<E>> {
        let raw_value = options.to_u16();
        self.write_registers(Register::VMINActiveProtection, &raw_value.to_le_bytes())
            .await
    }

    /// Reads the VminActiveProtection register.
    pub async fn read_vmin_active_protection(&mut self) -> Result<VminActiveProtection, Error<E>> {
        let raw_options = self
            .read_registers(Register::VMINActiveProtection, 2)
            .await?;
        Ok(VminActiveProtection::from_u16(u16::from_le_bytes([
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ])))
    }

    /// Reads the VminActiveProtection settings as a structured type.
    pub async fn read_vmin_active_protection_settings(
        &mut self,
    ) -> Result<VminActiveProtection, Error<E>> {
        self.read_vmin_active_protection().await
    }

    /// Sets the VminActiveProtection settings from a structured type.
    pub async fn set_vmin_active_protection_settings(
        &mut self,
        settings: VminActiveProtection,
    ) -> Result<(), Error<E>> {
        self.set_vmin_active_protection(settings).await
    }

    /// Sets the VBUS_VAP_TH value from voltage in mV.
    pub async fn set_vbus_vap_th_mv(&mut self, voltage_mv: u16) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_vbus_vap_th_mv(voltage_mv);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Gets the VBUS_VAP_TH value in mV.
    pub async fn get_vbus_vap_th_mv(&mut self) -> Result<u16, Error<E>> {
        let vmin_prot = self.read_vmin_active_protection().await?;
        Ok(vmin_prot.vbus_vap_th_mv())
    }

    /// Sets the VSYS_TH2 value from voltage in mV (assuming 2s-5s mode).
    pub async fn set_vsys_th2_mv(&mut self, voltage_mv: u16) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_vsys_th2_mv(voltage_mv);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Gets the VSYS_TH2 value in mV (assuming 2s-5s mode).
    pub async fn get_vsys_th2_mv(&mut self) -> Result<u16, Error<E>> {
        let vmin_prot = self.read_vmin_active_protection().await?;
        Ok(vmin_prot.vsys_th2_mv())
    }

    /// Enables the EN_VSYSTH2_FOLLOW_VSYSTH1 bit.
    pub async fn enable_vsysth2_follow_vsysth1(&mut self) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_en_vsysth2_follow_vsysth1(true);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Disables the EN_VSYSTH2_FOLLOW_VSYSTH1 bit.
    pub async fn disable_vsysth2_follow_vsysth1(&mut self) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_en_vsysth2_follow_vsysth1(false);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Checks if the EN_VSYSTH2_FOLLOW_VSYSTH1 bit is enabled.
    pub async fn is_vsysth2_follow_vsysth1_enabled(&mut self) -> Result<bool, Error<E>> {
        let vmin_prot = self.read_vmin_active_protection().await?;
        Ok(vmin_prot.en_vsysth2_follow_vsysth1())
    }

    /// Enables the EN_FRS bit.
    pub async fn enable_frs(&mut self) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_en_frs(true);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Disables the EN_FRS bit.
    pub async fn disable_frs(&mut self) -> Result<(), Error<E>> {
        let mut vmin_prot = self.read_vmin_active_protection().await?;
        vmin_prot.set_en_frs(false);
        self.set_vmin_active_protection(vmin_prot).await
    }

    /// Checks if the EN_FRS bit is enabled.
    pub async fn is_frs_enabled(&mut self) -> Result<bool, Error<E>> {
        let vmin_prot = self.read_vmin_active_protection().await?;
        Ok(vmin_prot.en_frs())
    }
}
