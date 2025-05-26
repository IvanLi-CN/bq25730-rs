#![no_std]
#![no_main]

use bq25730_async_rs::RegisterAccess; // Import the RegisterAccess trait
use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, I2c},
    peripherals,
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use cortex_m_rt::exception;

// Import the BQ25730 driver crate
use bq25730_async_rs::Bq25730;

// For sharing I2C bus
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

// Define the I2C interrupt handler
bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Removed rtt_init_print!(); to avoid duplicate symbol error
    info!("Starting BQ25730 STM32G031 example...");

    let config = embassy_stm32::Config::default();
    let p = embassy_stm32::init(config);

    info!("STM32 initialized.");

    // Configure I2C1 (PB6 SCL, PB7 SDA) with DMA
    let mut i2c_config = i2c::Config::default();
    i2c_config.scl_pullup = true;
    i2c_config.sda_pullup = true;

    // Create a static Mutex to share the I2C bus between multiple drivers
    static I2C_BUS_MUTEX_CELL: static_cell::StaticCell<
        Mutex<CriticalSectionRawMutex, I2c<'static, embassy_stm32::mode::Async>>,
    > = static_cell::StaticCell::new();

    let i2c_instance = i2c::I2c::new(
        p.I2C1,         // 1. peri
        p.PB6,          // 2. scl
        p.PB7,          // 3. sda
        Irqs,           // 4. _irq
        p.DMA1_CH1,     // 5. tx_dma (Assuming DMA1_CH1 for I2C1 TX)
        p.DMA1_CH2,     // 6. rx_dma (Assuming DMA1_CH2 for I2C1 RX)
        Hertz(100_000), // 7. freq
        i2c_config,     // 8. config
    );

    info!("I2C1 initialized on PB6/PB7 with DMA.");

    // Initialize the static Mutex with the I2C instance
    let i2c_bus_mutex = I2C_BUS_MUTEX_CELL.init(Mutex::new(i2c_instance));

    // BQ25730 I2C address (7-bit)
    let bq25730_address = 0x6B; // Confirmed from bq25730.pdf

    // Pass the I2C peripheral instance by value, wrapped in I2cAsynch
    let i2c_bus = I2cDevice::new(i2c_bus_mutex);
    let mut bq25730 = Bq25730::new(i2c_bus, bq25730_address);

    info!("BQ25730 driver instance created.");

    // Add this block to attempt reading ManufacturerID before full initialization
    info!("Attempting to read BQ25730 ManufacturerID...");
    match bq25730.read_register(bq25730_async_rs::registers::Register::ManufacturerID).await {
        Ok(id) => info!("Successfully read ManufacturerID: 0x{:X}", id),
        Err(e) => error!("Failed to read ManufacturerID: {:?}", e),
    }

    // --- BQ25730 Initialization Sequence ---
    info!("Initializing BQ25730...");
    if let Err(e) = bq25730.init().await {
        error!("Failed to initialize BQ25730: {:?}", e);
        core::panic!("Failed to initialize BQ25730: {:?}", e);
    }

    info!("BQ25730 initialization complete.");

    // --- Main Loop for Data Acquisition ---
    loop {
        info!("--- Reading BQ25730 Data ---");

        // Read Charger Status
        match bq25730.read_charger_status().await {
            Ok(status) => {
                info!("BQ25730 Charger Status:");
                info!("  Input Present: {}", status.stat_ac);
                info!("  ICO Complete: {}", status.ico_done);
                info!("  In VAP Mode: {}", status.in_vap);
                info!("  In VINDPM: {}", status.in_vindpm);
                info!("  In IIN_DPM: {}", status.in_iin_dpm);
                info!("  In Fast Charge: {}", status.in_fchrg);
                info!("  In Pre-Charge: {}", status.in_pchrg);
                info!("  In OTG Mode: {}", status.in_otg);
                info!("  Fault ACOV: {}", status.fault_acov);
                info!("  Fault BATOC: {}", status.fault_batoc);
                info!("  Fault ACOC: {}", status.fault_acoc);
                info!("  Fault SYSOVP: {}", status.fault_sysovp);
                info!("  Fault VSYS_UVP: {}", status.fault_vsys_uvp);
                info!(
                    "  Fault Force Converter Off: {}",
                    status.fault_force_converter_off
                );
                info!("  Fault OTG OVP: {}", status.fault_otg_ovp);
                info!("  Fault OTG UVP: {}", status.fault_otg_uvp);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Charger Status: {:?}", e);
            }
        }

        // Read Prochot Status
        match bq25730.read_prochot_status().await {
            Ok(status) => {
                info!("BQ25730 Prochot Status:");
                info!("  VINDPM Triggered: {}", status.stat_vindpm);
                info!("  Comparator Triggered: {}", status.stat_comp);
                info!("  ICRIT Triggered: {}", status.stat_icrit);
                info!("  INOM Triggered: {}", status.stat_inom);
                info!("  IDCHG1 Triggered: {}", status.stat_idchg1);
                info!("  VSYS Triggered: {}", status.stat_vsys);
                info!("  Battery Removal: {}", status.stat_bat_removal);
                info!("  Adapter Removal: {}", status.stat_adpt_removal);
                info!("  VAP Fail: {}", status.stat_vap_fail);
                info!("  Exit VAP: {}", status.stat_exit_vap);
                info!("  IDCHG2 Triggered: {}", status.stat_idchg2);
                info!("  PTM Operation: {}", status.stat_ptm);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Prochot Status: {:?}", e);
            }
        }

        // Read ADC Measurements
        match bq25730.read_adc_measurements().await {
            Ok(measurements) => {
                info!("BQ25730 ADC Measurements:");
                info!("  ADCPSYS: {} mV", measurements.psys.0);
                info!("  ADCVBUS: {} mV", measurements.vbus.0);
                info!("  ADCIDCHG: {} mA", measurements.idchg.0);
                info!("  ADCICHG: {} mA", measurements.ichg.0);
                info!("  ADCCMPIN: {} mV", measurements.cmpin.0);
                info!("  ADCIIN: {} mA", measurements.iin.0);
                info!("  ADCCMPIN: {} mV", measurements.cmpin.0);
                info!("  ADCIIN: {} mA", measurements.iin.0);
                info!("  ADCVBAT: {} mV", measurements.vbat.0);
                info!("  ADCVSYS: {} mV", measurements.vsys.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 ADC Measurements: {:?}", e);
            }
        }

        // Read Charge Current
        match bq25730.read_charge_current().await {
            Ok(current) => {
                info!("BQ25730 Charge Current: {} mA", current.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Charge Current: {:?}", e);
            }
        }

        // Read Charge Voltage
        match bq25730.read_charge_voltage().await {
            Ok(voltage) => {
                info!("BQ25730 Charge Voltage: {} mV", voltage.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Charge Voltage: {:?}", e);
            }
        }

        // Read OTG Voltage
        match bq25730.read_otg_voltage().await {
            Ok(voltage) => {
                info!("BQ25730 OTG Voltage: {} mV", voltage.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 OTG Voltage: {:?}", e);
            }
        }

        // Read OTG Current
        match bq25730.read_otg_current().await {
            Ok(current) => {
                info!("BQ25730 OTG Current: {} mA", current.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 OTG Current: {:?}", e);
            }
        }

        // Read Input Voltage
        match bq25730.read_input_voltage().await {
            Ok(voltage) => {
                info!("BQ25730 Input Voltage: {} mV", voltage.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Input Voltage: {:?}", e);
            }
        }

        // Read Minimum System Voltage
        match bq25730.read_vsys_min().await {
            Ok(voltage) => {
                info!("BQ25730 Minimum System Voltage: {} mV", voltage.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 Minimum System Voltage: {:?}", e);
            }
        }

        // Read IIN_HOST
        match bq25730.read_iin_host().await {
            Ok(current) => {
                info!("BQ25730 IIN_HOST: {} mA", current.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 IIN_HOST: {:?}", e);
            }
        }

        // Read IIN_DPM
        match bq25730.read_iin_dpm().await {
            Ok(current) => {
                info!("BQ25730 IIN_DPM: {} mA", current.0);
            }
            Err(e) => {
                error!("Failed to read BQ25730 IIN_DPM: {:?}", e);
            }
        }

        info!("----------------------------");

        // Wait for 1 second
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf();
}

#[exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    cortex_m::asm::udf();
}