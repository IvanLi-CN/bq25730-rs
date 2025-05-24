#![no_std]
#[cfg(feature = "defmt")]
extern crate defmt; // Make defmt available for derive macros

use core::ops::{Deref, DerefMut};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

pub mod data_types;
mod errors;
pub mod registers;
use data_types::*; // Import BQ25730 data types
pub use errors::Error;
use registers::Register; // Use BQ25730 registers

/// Trait for abstracting register access, with or without CRC.
pub trait RegisterAccess<E> {
    /// The buffer type used for reading multiple registers.
    type ReadBuffer: Default + Extend<u8> + Deref<Target = [u8]> + DerefMut + Sized;
    /// The buffer type used for reading multiple registers.

    /// Reads a single register.
    fn read_register(
        &mut self,
        reg: Register,
    ) -> impl core::future::Future<Output = Result<u8, Error<E>>>;

    /// Reads multiple registers starting from `reg`.
    fn read_registers(
        &mut self,
        reg: Register,
        len: usize,
    ) -> impl core::future::Future<Output = Result<Self::ReadBuffer, Error<E>>>;

    /// Writes a single register.
    fn write_register(
        &mut self,
        reg: Register,
        value: u8,
    ) -> impl core::future::Future<Output = Result<(), Error<E>>>;

    /// Writes multiple registers starting from `reg`.
    fn write_registers(
        &mut self,
        reg: Register,
        values: &[u8],
    ) -> impl core::future::Future<Output = Result<(), Error<E>>>;
}

/// BQ25730 driver
pub struct Bq25730<I2C>
where
    I2C: I2c,
{
    address: u8,
    i2c: I2C,
}

impl<I2C, E> Bq25730<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Creates a new instance of the BQ25730 driver.
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C peripheral.
    /// * `address` - The I2C address of the BQ25730 chip.
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self {
            address,
            i2c,
        }
    }
}

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


impl<I2C, E> Bq25730<I2C>
where
    I2C: I2c<Error = E> + Send,
    Self: RegisterAccess<E>,
{
    /// Initializes the BQ25730 charger.
    /// This function sets basic configuration and clears status flags.
    pub async fn init(&mut self) -> Result<(), Error<E>> {
        // Example initialization steps (refer to datasheet for recommended sequence)

        // Set default ChargeOption0 (e.g., enable IIN_DPM, disable Charge Inhibit)
        // Assuming default values for other bits for now.
        let charge_option0_lsb: u8 = registers::CHARGE_OPTION0_EN_IIN_DPM;
        let charge_option0_msb: u8 = 0; // Assuming default MSB
                                        // ChargeOption0 is a 16-bit register (01/00h). Write to LSB (0x00) first.
        self.write_registers(
            Register::ChargeOption0,
            &[charge_option0_lsb, charge_option0_msb],
        )
        .await?;

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
        // Need to confirm this with datasheet.
        // For now, let's clear the fault flags in ChargerStatus LSB.
        let flags_to_clear: u8 = registers::CHARGER_STATUS_FAULT_ACOV
            | registers::CHARGER_STATUS_FAULT_BATOC
            | registers::CHARGER_STATUS_FAULT_ACOC
            | registers::CHARGER_STATUS_FAULT_SYSOVP
            | registers::CHARGER_STATUS_FAULT_VSYS_UVP
            | registers::CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF
            | registers::CHARGER_STATUS_FAULT_OTG_OVP
            | registers::CHARGER_STATUS_FAULT_OTG_UVP;
        // Note: SysStat register address is 0x20, which is the LSB of ChargerStatus.
        // Writing to ChargerStatus LSB (0x20) clears these flags.
        // Clear fault flags in ChargerStatus LSB (0x20) by writing 1s.
        self.write_register(Register::ChargerStatus, flags_to_clear)
            .await?;

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
        let raw_status = self.read_registers(Register::ChargerStatusMsb, 2).await?;
        let lsb = raw_status.as_ref()[0];
        let msb = raw_status.as_ref()[1];

        Ok(ChargerStatus {
            stat_ac: (msb & registers::CHARGER_STATUS_STAT_AC) != 0,
            ico_done: (msb & registers::CHARGER_STATUS_ICO_DONE) != 0,
            in_vap: (msb & registers::CHARGER_STATUS_IN_VAP) != 0,
            in_vindpm: (msb & registers::CHARGER_STATUS_IN_VINDPM) != 0,
            in_iin_dpm: (msb & registers::CHARGER_STATUS_IN_IIN_DPM) != 0,
            in_fchrg: (msb & registers::CHARGER_STATUS_IN_FCHRG) != 0,
            in_pchrg: (msb & registers::CHARGER_STATUS_IN_PCHRG) != 0,
            in_otg: (msb & registers::CHARGER_STATUS_IN_OTG) != 0,
            fault_acov: (lsb & registers::CHARGER_STATUS_FAULT_ACOV) != 0,
            fault_batoc: (lsb & registers::CHARGER_STATUS_FAULT_BATOC) != 0,
            fault_acoc: (lsb & registers::CHARGER_STATUS_FAULT_ACOC) != 0,
            fault_sysovp: (lsb & registers::CHARGER_STATUS_FAULT_SYSOVP) != 0,
            fault_vsys_uvp: (lsb & registers::CHARGER_STATUS_FAULT_VSYS_UVP) != 0,
            fault_force_converter_off: (lsb & registers::CHARGER_STATUS_FAULT_FORCE_CONVERTER_OFF)
                != 0,
            fault_otg_ovp: (lsb & registers::CHARGER_STATUS_FAULT_OTG_OVP) != 0,
            fault_otg_uvp: (lsb & registers::CHARGER_STATUS_FAULT_OTG_UVP) != 0,
        })
    }

    /// Reads the Prochot Status register.
    pub async fn read_prochot_status(&mut self) -> Result<ProchotStatus, Error<E>> {
        let raw_status = self.read_registers(Register::ProchotStatusMsb, 2).await?;
        let lsb = raw_status.as_ref()[0]; // ProchotStatus LSB (0x22)
        let msb = raw_status.as_ref()[1]; // ProchotStatus MSB (0x23)

        // Read ChargeOption4 LSB (0x3C) for stat_idchg2 and stat_ptm
        let raw_charge_option4 = self.read_registers(Register::ChargeOption4, 1).await?;
        let charge_option4_lsb = raw_charge_option4.as_ref()[0]; // ChargeOption4 LSB (0x3C)

        Ok(ProchotStatus {
            en_prochot_ext: (msb & registers::PROCHOT_STATUS_EN_PROCHOT_EXT) != 0,
            prochot_width: (msb & registers::PROCHOT_STATUS_PROCHOT_WIDTH) >> 4,
            prochot_clear: (msb & registers::PROCHOT_STATUS_PROCHOT_CLEAR) != 0,
            stat_vap_fail: (msb & registers::PROCHOT_STATUS_STAT_VAP_FAIL) != 0,
            stat_exit_vap: (msb & registers::PROCHOT_STATUS_STAT_EXIT_VAP) != 0,
            stat_vindpm: (lsb & registers::PROCHOT_STATUS_STAT_VINDPM) != 0,
            stat_comp: (lsb & registers::PROCHOT_STATUS_STAT_COMP) != 0,
            stat_icrit: (lsb & registers::PROCHOT_STATUS_STAT_ICRIT) != 0,
            stat_inom: (lsb & registers::PROCHOT_STATUS_STAT_INOM) != 0,
            stat_idchg1: (lsb & registers::PROCHOT_STATUS_STAT_IDCHG1) != 0,
            stat_vsys: (lsb & registers::PROCHOT_STATUS_STAT_VSYS) != 0,
            stat_bat_removal: (lsb & registers::PROCHOT_STATUS_STAT_BAT_REMOVAL) != 0,
            stat_adpt_removal: (lsb & registers::PROCHOT_STATUS_STAT_ADPT_REMOVAL) != 0,
            stat_idchg2: (charge_option4_lsb & registers::CHARGE_OPTION4_STAT_IDCHG2) != 0,
            stat_ptm: (charge_option4_lsb & registers::CHARGE_OPTION4_STAT_PTM) != 0,
        })
    }
    /// Reads all ADC measurement registers.
    pub async fn read_adc_measurements(&mut self) -> Result<AdcMeasurements, Error<E>> {
        let raw_measurements = self.read_registers(Register::ADCPSYS, 8).await?;
        Ok(AdcMeasurements::from_register_values(
            raw_measurements.as_ref()[0], // ADCPSYS
            raw_measurements.as_ref()[1], // ADCVBUS
            raw_measurements.as_ref()[2], // ADCIDCHG
            raw_measurements.as_ref()[3], // ADCICHG
            raw_measurements.as_ref()[4], // ADCCMPIN
            raw_measurements.as_ref()[5], // ADCIIN
            raw_measurements.as_ref()[6], // ADCVBAT
            raw_measurements.as_ref()[7], // ADCVSYS
        ))
    }

    /// Reads the Charge Current register and returns the value in mA.
    pub async fn read_charge_current(&mut self) -> Result<ChargeCurrent, Error<E>> {
        let raw_current = self.read_registers(Register::ChargeCurrentMsb, 2).await?;
        Ok(ChargeCurrent::from_register_value(
            raw_current.as_ref()[1],
            raw_current.as_ref()[0],
        ))
    }

    /// Writes the Charge Current register with the value in mA.
    pub async fn set_charge_current(&mut self, current: ChargeCurrent) -> Result<(), Error<E>> {
        let (msb, lsb) = current.to_msb_lsb_bytes();
        // ChargeCurrent is a 13-bit register (03/02h). Write to LSB (0x02) first.
        self.write_registers(Register::ChargeCurrent, &[lsb, msb])
            .await
    }

    /// Reads the Charge Voltage register and returns the value in mV.
    pub async fn read_charge_voltage(&mut self) -> Result<ChargeVoltage, Error<E>> {
        let raw_voltage = self.read_registers(Register::ChargeVoltageMsb, 2).await?;
        Ok(ChargeVoltage::from_register_value(
            raw_voltage.as_ref()[1],
            raw_voltage.as_ref()[0],
        ))
    }

    /// Writes the Charge Voltage register with the value in mV.
    pub async fn set_charge_voltage(&mut self, voltage: ChargeVoltage) -> Result<(), Error<E>> {
        let (msb, lsb) = voltage.to_msb_lsb_bytes();
        // ChargeVoltage is a 12-bit register (05/04h). Write to LSB (0x04) first.
        self.write_registers(Register::ChargeVoltage, &[lsb, msb])
            .await
    }

    /// Reads the OTG Voltage register and returns the value in mV.
    pub async fn read_otg_voltage(&mut self) -> Result<OtgVoltage, Error<E>> {
        let raw_voltage = self.read_registers(Register::OTGVoltageMsb, 2).await?;
        Ok(OtgVoltage::from_register_value(
            raw_voltage.as_ref()[1],
            raw_voltage.as_ref()[0],
        ))
    }

    /// Writes the OTG Voltage register with the value in mV.
    pub async fn set_otg_voltage(&mut self, voltage: OtgVoltage) -> Result<(), Error<E>> {
        let (msb, lsb) = voltage.to_msb_lsb_bytes();
        // OTGVoltage is an 11-bit register (07/06h). Write to LSB (0x06) first.
        self.write_registers(Register::OTGVoltage, &[lsb, msb])
            .await
    }

    /// Reads the OTG Current register and returns the value in mA.
    pub async fn read_otg_current(&mut self) -> Result<OtgCurrent, Error<E>> {
        let raw_current = self.read_registers(Register::OTGCurrentMsb, 2).await?;
        Ok(OtgCurrent::from_register_value(
            raw_current.as_ref()[1],
            raw_current.as_ref()[0],
        ))
    }

    /// Writes the OTG Current register with the value in mA.
    pub async fn set_otg_current(&mut self, current: OtgCurrent) -> Result<(), Error<E>> {
        let (msb, lsb) = current.to_msb_lsb_bytes();
        // OTGCurrent is a 10-bit register (09/08h). Write to LSB (0x08) first.
        self.write_registers(Register::OTGCurrent, &[lsb, msb])
            .await
    }

    /// Reads the Input Voltage register and returns the value in mV.
    pub async fn read_input_voltage(&mut self) -> Result<InputVoltage, Error<E>> {
        let raw_voltage = self.read_registers(Register::InputVoltageMsb, 2).await?;
        Ok(InputVoltage::from_register_value(
            raw_voltage.as_ref()[1],
            raw_voltage.as_ref()[0],
        ))
    }
    /// Writes the Input Voltage register with the value in mV.
    pub async fn set_input_voltage(&mut self, voltage: InputVoltage) -> Result<(), Error<E>> {
        let (msb, lsb) = voltage.to_msb_lsb_bytes();
        // InputVoltage is a 8-bit register (0B/0Ah). Write to LSB (0x0A) first.
        self.write_registers(Register::InputVoltage, &[lsb, msb])
            .await
    }

    /// Reads the Minimum System Voltage register and returns the value in mV.
    pub async fn read_vsys_min(&mut self) -> Result<VsysMin, Error<E>> {
        let raw_voltage = self.read_register(Register::VsysMin).await?;
        Ok(VsysMin::from_register_value(raw_voltage))
    }

    /// Writes the Minimum System Voltage register with the value in mV.
    pub async fn set_vsys_min(&mut self, voltage: VsysMin) -> Result<(), Error<E>> {
        let raw_value = voltage.to_register_value();
        let lsb = (raw_value & 0xFF) as u8;
        let msb = ((raw_value >> 8) & 0x0F) as u8; // 12-bit value, MSB uses bits 0-3
        self.write_registers(Register::VsysMinMsb, &[lsb, msb])
            .await
    }

    /// Reads the IIN_HOST register and returns the value in mA.
    pub async fn read_iin_host(&mut self) -> Result<IinHost, Error<E>> {
        let raw_current = self.read_register(Register::IinHost).await?;
        Ok(IinHost::from_register_value(raw_current))
    }

    /// Writes the IIN_HOST register with the value in mA.
    pub async fn set_iin_host(&mut self, current: IinHost) -> Result<(), Error<E>> {
        let (msb, lsb) = current.to_msb_lsb_bytes();
        // IIN_HOST is a 7-bit register (0F/0Eh). Write to LSB (0x0E) first.
        self.write_registers(Register::IinHost, &[lsb, msb]).await
    }

    /// Reads the IIN_DPM register and returns the value in mA.
    pub async fn read_iin_dpm(&mut self) -> Result<IinDpm, Error<E>> {
        let raw_current = self.read_register(Register::IinDpm).await?;
        Ok(IinDpm::from_register_value(raw_current))
    }

    /// Writes the IIN_DPM register with the value in mA.
    pub async fn set_iin_dpm(&mut self, current: IinDpm) -> Result<(), Error<E>> {
        let (msb, lsb) = current.to_msb_lsb_bytes();
        // IIN_DPM is a 7-bit register (25/24h). Write to LSB (0x24) first.
        self.write_registers(Register::IinDpm, &[lsb, msb]).await
    }

    /// Sets the ChargeOption0 register.
    pub async fn set_charge_option0(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ChargeOption0 is a 16-bit register (01/00h). Write to LSB (0x00) first.
        self.write_registers(Register::ChargeOption0, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption0 register.
    pub async fn read_charge_option0(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption0Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ChargeOption1 register.
    pub async fn set_charge_option1(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ChargeOption1 is a 16-bit register (31/30h). Write to LSB (0x30) first.
        self.write_registers(Register::ChargeOption1, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption1 register.
    pub async fn read_charge_option1(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption1Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ChargeOption2 register.
    pub async fn set_charge_option2(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ChargeOption2 is a 16-bit register (33/32h). Write to LSB (0x32) first.
        self.write_registers(Register::ChargeOption2, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption2 register.
    pub async fn read_charge_option2(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption2Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ChargeOption3 register.
    pub async fn set_charge_option3(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ChargeOption3 is a 16-bit register (35/34h). Write to LSB (0x34) first.
        self.write_registers(Register::ChargeOption3, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption3 register.
    pub async fn read_charge_option3(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption3Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ChargeOption4 register.
    pub async fn set_charge_option4(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ChargeOption4 is a 16-bit register (3D/3Ch). Write to LSB (0x3C) first.
        self.write_registers(Register::ChargeOption4, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption4 register.
    pub async fn read_charge_option4(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption4Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ProchotOption0 register.
    pub async fn set_prochot_option0(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ProchotOption0 is a 16-bit register (37/36h). Write to LSB (0x36) first.
        self.write_registers(Register::ProchotOption0, &[lsb, msb])
            .await
    }

    /// Reads the ProchotOption0 register.
    pub async fn read_prochot_option0(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ProchotOption0Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ProchotOption1 register.
    pub async fn set_prochot_option1(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ProchotOption1 is a 16-bit register (39/38h). Write to LSB (0x38) first.
        self.write_registers(Register::ProchotOption1, &[lsb, msb])
            .await
    }

    /// Reads the ProchotOption1 register.
    pub async fn read_prochot_option1(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ProchotOption1Msb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the ADCOption register.
    pub async fn set_adc_option(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // ADCOption is a 16-bit register (3B/3Ah). Write to LSB (0x3A) first.
        self.write_registers(Register::ADCOption, &[lsb, msb]).await
    }

    /// Reads the ADCOption register.
    pub async fn read_adc_option(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self.read_registers(Register::ADCOptionMsb, 2).await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Sets the VMIN_ACTIVE_PROTECTION register.
    pub async fn set_vmin_active_protection(&mut self, lsb: u8, msb: u8) -> Result<(), Error<E>> {
        // VMIN_ACTIVE_PROTECTION is a 16-bit register (3F/3Eh). Write to LSB (0x3E) first.
        self.write_registers(Register::VMINActiveProtection, &[lsb, msb])
            .await
    }

    /// Reads the VMIN_ACTIVE_PROTECTION register.
    pub async fn read_vmin_active_protection(&mut self) -> Result<(u8, u8), Error<E>> {
        let raw_options = self
            .read_registers(Register::VMINActiveProtectionMsb, 2)
            .await?;
        Ok((raw_options.as_ref()[0], raw_options.as_ref()[1]))
    }

    /// Enters ship mode.
    pub async fn enter_ship_mode(&mut self) -> Result<(), Error<E>> {
        // To enter ship mode, write 0x0013 to the ShipMode register (0x40).
        // This is a 16-bit register. LSB is 0x13, MSB is 0x00.
        self.write_registers(registers::Register::ShipMode, &[0x13, 0x00])
            .await
    }
}
