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
    // Config, RsnsAc, RsnsBat are handled by the pub use below
};
use crate::registers::{
    ChargeOption1Flags, ChargerStatusFaultFlags, ChargerStatusFlags,
};
pub use errors::Error;
use registers::Register;
pub use data_types::{SenseResistorValue, Config}; // Re-export Config and SenseResistorValue

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
        // Write ChargeOption0, ChargeCurrent, and ChargeVoltage (Registers 0x00-0x05)
        // These are contiguous registers.
        let charge_option0_bytes = self.config.charge_option0.to_msb_lsb_bytes();
        let charge_current_bytes = self.config.charge_current.to_le_bytes(); // Assuming to_le_bytes() is appropriate, or use to_msb_lsb_bytes() if available and needed for consistency
        let charge_voltage_bytes = self.config.charge_voltage.to_le_bytes(); // Assuming to_le_bytes() is appropriate

        self.write_registers(Register::ChargeOption0, &[
            charge_option0_bytes.0, // ChargeOption0 LSB
            charge_option0_bytes.1, // ChargeOption0 MSB
            charge_current_bytes[0], // ChargeCurrent LSB
            charge_current_bytes[1], // ChargeCurrent MSB
            charge_voltage_bytes[0], // ChargeVoltage LSB
            charge_voltage_bytes[1], // ChargeVoltage MSB
        ]).await?;

        // Write ChargeOption1 (Registers 0x30-0x31) - Not contiguous with the previous block
        let (lsb_co1, msb_co1) = self.config.charge_option1.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption1, &[lsb_co1, msb_co1]).await?;


        // Group 2: InputVoltage, VsysMin, IinHost (Registers 0x0A-0x0F)
        // These are contiguous registers.
        let input_voltage_bytes = self.config.input_voltage.to_le_bytes();
        let vsys_min_bytes = self.config.vsys_min.to_le_bytes();
        let iin_host_bytes = self.config.iin_host.to_le_bytes();
        self.write_registers(Register::InputVoltage, &[
            input_voltage_bytes[0], // InputVoltage LSB
            input_voltage_bytes[1], // InputVoltage MSB
            vsys_min_bytes[0],      // VsysMin LSB
            vsys_min_bytes[1],      // VsysMin MSB
            iin_host_bytes[0],      // IinHost LSB
            iin_host_bytes[1],      // IinHost MSB
        ]).await?;

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
            psys: AdcPsys::from_u8(adc_data_raw.as_ref()[0]),                     // ADCPSYS at 0x26
            vbus: AdcVbus::from_u8(adc_data_raw.as_ref()[1]),                     // ADCVBUS at 0x27
            idchg: AdcIdchg::from_raw(adc_data_raw.as_ref()[2], self.config.rsns_bat), // ADCIDCHG at 0x28
            ichg: AdcIchg::from_raw(adc_data_raw.as_ref()[3], self.config.rsns_bat), // ADCICHG at 0x29
            cmpin: AdcCmpin::from_u8(adc_data_raw.as_ref()[4]),                   // ADCCMPIN at 0x2A
            iin: AdcIin::from_raw(adc_data_raw.as_ref()[5], self.config.rsns_ac),   // ADCIIN at 0x2B
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

    /// Reads the Charge Current register and returns the value in mA.
    pub async fn read_charge_current(&mut self) -> Result<ChargeCurrent, Error<E>> {
        let lsb_byte = self.read_register(Register::ChargeCurrent).await?;
        let msb_byte = self.read_register(Register::ChargeCurrentMsb).await?;
        let d1_d0 = (lsb_byte >> 6) & 0x03;
        let d6_d2 = msb_byte & 0x1F;
        let raw_7bit = (d6_d2 << 2) | d1_d0;
        Ok(ChargeCurrent::from_raw(raw_7bit, self.config.rsns_bat))
    }

    /// Writes the Charge Current register with the value in mA.
    pub async fn set_charge_current(&mut self, current: ChargeCurrent) -> Result<(), Error<E>> {
        let raw_7bit = current.to_raw();
        let d1_d0 = raw_7bit & 0x03;
        let d6_d2 = (raw_7bit >> 2) & 0x1F;
        let mut lsb_val = self.read_register(Register::ChargeCurrent).await?;
        lsb_val &= !(0x03 << 6);
        lsb_val |= d1_d0 << 6;
        self.write_register(Register::ChargeCurrent, lsb_val)
            .await?;
        let mut msb_val = self.read_register(Register::ChargeCurrentMsb).await?;
        msb_val &= !0x1F;
        msb_val |= d6_d2;
        self.write_register(Register::ChargeCurrentMsb, msb_val)
            .await
    }

    /// Reads the Charge Voltage register and returns the value in mV.
    pub async fn read_charge_voltage(&mut self) -> Result<ChargeVoltage, Error<E>> {
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
        let msb_byte = self.read_register(Register::OTGCurrentMsb).await?;
        let raw_7bit = msb_byte & 0x7F;
        Ok(OtgCurrent::from_raw(raw_7bit, self.config.rsns_bat))
    }

    /// Writes the OTG Current register with the value in mA.
    pub async fn set_otg_current(&mut self, current: OtgCurrent) -> Result<(), Error<E>> {
        let raw_7bit = current.to_raw();
        let mut msb_val = self.read_register(Register::OTGCurrentMsb).await?;
        msb_val &= !0x7F;
        msb_val |= raw_7bit & 0x7F;
        self.write_register(Register::OTGCurrentMsb, msb_val).await
    }

    /// Reads the Input Voltage register and returns the value in mV.
    pub async fn read_input_voltage(&mut self) -> Result<InputVoltage, Error<E>> {
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
        let msb_byte = self.read_register(Register::IinHostMsb).await?;
        let raw_7bit = msb_byte & 0x7F;
        Ok(IinHost::from_raw(raw_7bit, self.config.rsns_ac))
    }

    /// Writes the IIN_HOST register with the value in mA.
    pub async fn set_iin_host(&mut self, current: IinHost) -> Result<(), Error<E>> {
        let raw_7bit = current.to_raw();
        let mut msb_val = self.read_register(Register::IinHostMsb).await?;
        msb_val &= !0x7F;
        msb_val |= raw_7bit & 0x7F;
        self.write_register(Register::IinHostMsb, msb_val).await
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
