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
    pub i2c: I2C,   // Make i2c field public for testing
    cell_count: u8, // 新增字段：电池节数
}

/// Trait for abstracting register access, with or without CRC.
#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Bq25730",),
    async(feature = "async", keep_self)
)]
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
    /// * `cell_count` - The number of battery cells (e.g., 4 for 4S, 5 for 5S).
    pub fn new(i2c: I2C, address: u8, cell_count: u8) -> Self {
        Self {
            address,
            i2c,
            cell_count,
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

        // Set default ChargeOption0 (e.g., enable IIN_DPM, disable Charge Inhibit)
        // Assuming default values for other bits for now.
        // Read current ChargeOption0 values to preserve other settings
        let mut charge_option0 = self.read_charge_option0().await?;
        // Set EN_IIN_DPM bit
        charge_option0.en_iin_dpm = true;
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
        // Need to confirm this with datasheet.
        // For now, let's clear the fault flags in ChargerStatus LSB.
        // Clear all status flags
        // Read current ChargerStatus LSB (0x20)
        let mut current_charger_status_lsb = self.read_register(Register::ChargerStatus).await?;
        // Clear Fault SYSOVP (bit 4) and Fault VSYS_UVP (bit 3) by setting them to 0
        current_charger_status_lsb &=
            !(registers::CHARGER_STATUS_FAULT_SYSOVP | registers::CHARGER_STATUS_FAULT_VSYS_UVP);
        self.write_register(Register::ChargerStatus, current_charger_status_lsb)
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
        // ChargerStatus is a 16-bit register (0x20/0x21). Read from LSB address (0x20).
        let raw_status = self.read_registers(Register::ChargerStatus, 2).await?;
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
        let raw_status = self.read_registers(Register::ProchotStatus, 2).await?; // Read from LSB address (0x22)
        let lsb = raw_status.as_ref()[0]; // ProchotStatus LSB (0x22)
        let msb = raw_status.as_ref()[1]; // ProchotStatus MSB (0x23)

        // Read ChargeOption4 LSB (0x3C) for stat_idchg2 and stat_ptm
        // NOTE: ChargeOption4 is not defined in data_types.rs, this might be a placeholder or error in original code.
        // Assuming for now that these bits are part of ProchotStatus or another register.
        // For now, we will remove the dependency on ChargeOption4.
        // let raw_charge_option4 = self.read_register(Register::ChargeOption4).await?; // Read single 8-bit register
        // let charge_option4_lsb = raw_charge_option4; // ChargeOption4 LSB (0x3C)

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
            // stat_idchg2: (charge_option4_lsb & registers::CHARGE_OPTION4_STAT_IDCHG2) != 0,
            // stat_ptm: (charge_option4_lsb & registers::CHARGE_OPTION4_STAT_PTM) != 0,
            stat_idchg2: false, // Placeholder, needs to be verified from datasheet
            stat_ptm: false,    // Placeholder, needs to be verified from datasheet
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
        let psys_raw = self.read_register(Register::ADCPSYS).await?;
        let vbus_raw = self.read_register(Register::ADCVBUS).await?;
        let idchg_raw = self.read_register(Register::ADCIDCHG).await?;
        let ichg_raw = self.read_register(Register::ADCICHG).await?;
        let cmpin_raw = self.read_register(Register::ADCCMPIN).await?;
        let iin_raw = self.read_register(Register::ADCIIN).await?;
        let vbat_raw = self.read_register(Register::ADCVBAT).await?;
        let vsys_raw = self.read_register(Register::ADCVSYS).await?;

        Ok(AdcMeasurements {
            vbat: AdcVbat::from_register_value(vbat_raw, offset_mv),
            vsys: AdcVsys::from_register_value(vsys_raw, offset_mv),
            ichg: AdcIchg::from_register_value(ichg_raw),
            idchg: AdcIdchg::from_register_value(idchg_raw),
            iin: AdcIin::from_register_value(iin_raw),
            psys: AdcPsys::from_register_value(psys_raw),
            vbus: AdcVbus::from_register_value(vbus_raw),
            cmpin: AdcCmpin::from_register_value(cmpin_raw),
        })
    }

    /// Reads the Charge Current register and returns the value in mA.
    pub async fn read_charge_current(&mut self) -> Result<ChargeCurrent, Error<E>> {
        // ChargeCurrent is a 13-bit register (03/02h). Read from LSB address (0x02).
        let raw_current = self.read_registers(Register::ChargeCurrent, 2).await?;
        Ok(ChargeCurrent::from_register_value(
            raw_current.as_ref()[0], // LSB
            raw_current.as_ref()[1], // MSB
        ))
    }

    /// Writes the Charge Current register with the value in mA.
    pub async fn set_charge_current(&mut self, current: ChargeCurrent) -> Result<(), Error<E>> {
        let (lsb, msb) = current.to_msb_lsb_bytes();
        // ChargeCurrent is a 13-bit register (03/02h). Write to LSB (0x02) first.
        self.write_registers(Register::ChargeCurrent, &[lsb, msb])
            .await
    }

    /// Reads the Charge Voltage register and returns the value in mV.
    /// Reads the Charge Voltage register and returns the value in mV.
    pub async fn read_charge_voltage(&mut self) -> Result<ChargeVoltage, Error<E>> {
        // ChargeVoltage is a 12-bit register (05/04h). Read from LSB address (0x04).
        let raw_voltage = self.read_registers(Register::ChargeVoltage, 2).await?;
        Ok(ChargeVoltage::from_register_value(
            raw_voltage.as_ref()[0],
            raw_voltage.as_ref()[1],
        ))
    }

    /// Writes the Charge Voltage register with the value in mV.
    pub async fn set_charge_voltage(&mut self, voltage: ChargeVoltage) -> Result<(), Error<E>> {
        let (lsb, msb) = voltage.to_msb_lsb_bytes();
        // ChargeVoltage is a 12-bit register (05/04h). Write to LSB (0x04) first.
        self.write_registers(Register::ChargeVoltage, &[lsb, msb])
            .await
    }

    /// Reads the OTG Voltage register and returns the value in mV.
    pub async fn read_otg_voltage(&mut self) -> Result<OtgVoltage, Error<E>> {
        // OTGVoltage is an 11-bit register (07/06h). Read from LSB address (0x06).
        let raw_voltage = self.read_registers(Register::OTGVoltage, 2).await?;
        Ok(OtgVoltage::from_register_value(
            raw_voltage.as_ref()[0], // LSB
            raw_voltage.as_ref()[1], // MSB
        ))
    }

    /// Writes the OTG Voltage register with the value in mV.
    pub async fn set_otg_voltage(&mut self, voltage: OtgVoltage) -> Result<(), Error<E>> {
        let (lsb, msb) = voltage.to_msb_lsb_bytes();
        // OTGVoltage is an 11-bit register (07/06h). Write to LSB (0x06) first.
        self.write_registers(Register::OTGVoltage, &[lsb, msb])
            .await
    }

    /// Reads the OTG Current register and returns the value in mA.
    pub async fn read_otg_current(&mut self) -> Result<OtgCurrent, Error<E>> {
        let raw_current = self.read_registers(Register::OTGCurrent, 2).await?;
        Ok(OtgCurrent::from_register_value(
            raw_current.as_ref()[0],
            raw_current.as_ref()[1],
        ))
    }

    /// Writes the OTG Current register with the value in mA.
    pub async fn set_otg_current(&mut self, current: OtgCurrent) -> Result<(), Error<E>> {
        let (lsb, msb) = current.to_msb_lsb_bytes();
        // OTGCurrent is a 10-bit register (09/08h). Write to LSB (0x08) first.
        self.write_registers(Register::OTGCurrent, &[lsb, msb])
            .await
    }

    /// Reads the Input Voltage register and returns the value in mV.
    pub async fn read_input_voltage(&mut self) -> Result<InputVoltage, Error<E>> {
        // InputVoltage is a 9-bit register (0B/0Ah). Read from LSB address (0x0A).
        let raw_voltage = self.read_registers(Register::InputVoltage, 2).await?;
        Ok(InputVoltage::from_register_value(
            raw_voltage.as_ref()[0], // LSB
            raw_voltage.as_ref()[1], // MSB
        ))
    }
    /// Writes the Input Voltage register with the value in mV.
    pub async fn set_input_voltage(&mut self, voltage: InputVoltage) -> Result<(), Error<E>> {
        let (lsb, msb) = voltage.to_msb_lsb_bytes();
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
        self.write_register(Register::VsysMin, raw_value).await
    }

    /// Reads the IIN_HOST register and returns the value in mA.
    pub async fn read_iin_host(&mut self) -> Result<IinHost, Error<E>> {
        let raw_current = self.read_register(Register::IinHost).await?;
        Ok(IinHost::from_register_value(raw_current))
    }

    /// Writes the IIN_HOST register with the value in mA.
    pub async fn set_iin_host(&mut self, current: IinHost) -> Result<(), Error<E>> {
        self.write_register(Register::IinHost, current.to_register_value())
            .await
    }

    /// Reads the IIN_DPM register and returns the value in mA.
    pub async fn read_iin_dpm(&mut self) -> Result<IinDpm, Error<E>> {
        let raw_current = self.read_register(Register::IinDpm).await?;
        Ok(IinDpm::from_register_value(raw_current))
    }

    /// Writes the IIN_DPM register with the value in mA.
    pub async fn set_iin_dpm(&mut self, current: IinDpm) -> Result<(), Error<E>> {
        self.write_register(Register::IinDpm, current.to_register_value())
            .await
    }

    /// Sets the ChargeOption0 register.
    pub async fn set_charge_option0(&mut self, options: ChargeOption0) -> Result<(), Error<E>> {
        let (lsb, msb) = options.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption0, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption0 register.
    pub async fn read_charge_option0(&mut self) -> Result<ChargeOption0, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption0, 2).await?;
        Ok(ChargeOption0::from_register_value(
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ))
    }

    /// Sets the ChargeOption1 register.
    pub async fn set_charge_option1(&mut self, options: ChargeOption1) -> Result<(), Error<E>> {
        let (lsb, msb) = options.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption1, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption1 register.
    pub async fn read_charge_option1(&mut self) -> Result<ChargeOption1, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption1, 2).await?;
        Ok(ChargeOption1::from_register_value(
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ))
    }

    /// Sets the ChargeOption2 register.
    pub async fn set_charge_option2(&mut self, options: ChargeOption2) -> Result<(), Error<E>> {
        let (lsb, msb) = options.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption2, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption2 register.
    pub async fn read_charge_option2(&mut self) -> Result<ChargeOption2, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption2, 2).await?;
        Ok(ChargeOption2::from_register_value(
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ))
    }

    /// Sets the ChargeOption3 register.
    pub async fn set_charge_option3(&mut self, options: ChargeOption3) -> Result<(), Error<E>> {
        let (lsb, msb) = options.to_msb_lsb_bytes();
        self.write_registers(Register::ChargeOption3, &[lsb, msb])
            .await
    }

    /// Reads the ChargeOption3 register.
    pub async fn read_charge_option3(&mut self) -> Result<ChargeOption3, Error<E>> {
        let raw_options = self.read_registers(Register::ChargeOption3, 2).await?;
        Ok(ChargeOption3::from_register_value(
            raw_options.as_ref()[0],
            raw_options.as_ref()[1],
        ))
    }

    /// Enters ship mode.
    /// This function writes the required sequence to the Ship Mode register (0x40).
    pub async fn enter_ship_mode(&mut self) -> Result<(), Error<E>> {
        // Read current ChargeOption1 LSB and MSB
        let raw_options = self.read_registers(Register::ChargeOption1, 2).await?;
        let mut charge_option1_lsb = raw_options.as_ref()[0];
        let charge_option1_msb = raw_options.as_ref()[1];

        // Set EN_SHIP_DCHG bit (bit 1) in ChargeOption1 LSB
        charge_option1_lsb |= registers::CHARGE_OPTION1_EN_SHIP_DCHG;

        self.write_registers(
            Register::ChargeOption1,
            &[charge_option1_lsb, charge_option1_msb],
        )
        .await
    }
}
