#![no_std]
#![no_main]
#![allow(unused_imports)] // 允许未使用的导入，以消除 AdcMeasurements 的警告

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, Config, I2c},
    peripherals::I2C1,
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use bq25730_async_rs::{
    Bq25730,
    data_types::{AdcMeasurements, ChargeCurrentSetting, ChargeVoltageSetting, SenseResistorValue},
    registers::{ChargeOption0MsbFlags, ChargeOption1MsbFlags, WatchdogTimerAdjust},
};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello from STM32G431CB!");

    let mut config = Config::default();
    config.scl_pullup = true;
    config.sda_pullup = true;

    let i2c = I2c::new(
        p.I2C1,
        p.PA15,
        p.PB7,
        Irqs,
        p.DMA1_CH5,
        p.DMA1_CH6,
        Hertz(100_000),
        config,
    );

    let mut config = bq25730_async_rs::data_types::Config::new(
        4,
        SenseResistorValue::R5mOhm,
        SenseResistorValue::R10mOhm,
    );
    config
        .charge_option0
        .msb_flags
        .remove(ChargeOption0MsbFlags::EN_LWPWR);
    config
        .charge_option0
        .msb_flags
        .set_watchdog_timer(WatchdogTimerAdjust::Disabled);
    config
        .charge_option1
        .msb_flags
        .insert(ChargeOption1MsbFlags::EN_IBAT);

    let mut bq = Bq25730::new(i2c, 0x6B, config); // BQ25730 I2C 地址是 0x6B，电池节数 4

    // 1. 初始化 BQ25730
    info!("Initializing BQ25730...");
    if let Err(e) = bq.init().await {
        error!("Failed to initialize BQ25730: {:?}", e);
        // core::panic!("Failed to initialize BQ25730: {:?}", e); // 在示例中不直接panic
    }
    info!("BQ25730 initialization complete.");

    // 验证 ChargeOption0 寄存器设置
    match bq.read_charge_option0().await {
        Ok(options) => {
            let (lsb, msb) = options.to_msb_lsb_bytes();
            info!(
                "Verified Charge Option 0 (raw): LSB=0x{:02X}, MSB=0x{:02X}",
                lsb, msb
            );
        }
        Err(e) => {
            error!("Failed to read Charge Option 0 for verification: {:?}", e);
        }
    }

    // ADCOption 相关的代码已移除，因为 ADCOption 结构体已从 data_types.rs 中移除。
    // 如果需要 ADC 配置，请参考数据手册并重新实现。

    // 2. 充电控制示例
    info!("--- Charging Control Example ---");
    // 设置充电电流为 512 mA (4 * 128mA LSB)
    let charge_current = ChargeCurrentSetting::from_milliamps(512, bq.config().rsns_bat);
    if let Err(e) = bq.set_charge_current_setting(charge_current).await {
        error!("Failed to set charge current: {:?}", e);
    } else {
        info!("Charge current set to {} mA.", charge_current.to_milliamps());
    }

    // 设置充电电压为 18000 mV (5 节磷酸铁锂电池，每节 3.6V)
    let charge_voltage = ChargeVoltageSetting::from_millivolts(18000);
    if let Err(e) = bq.set_charge_voltage_setting(charge_voltage).await {
        error!("Failed to set charge voltage: {:?}", e);
    } else {
        info!("Charge voltage set to {} mV.", charge_voltage.to_millivolts());
    }
    info!("Charging control example complete.");

    // 3. 配置并启用 ADC 进行连续转换
    info!("--- Configuring and Enabling ADC ---");
    let adc_option = bq25730_async_rs::data_types::AdcOption {
        msb_flags: bq25730_async_rs::registers::AdcOptionMsbFlags::ADC_CONV
            | bq25730_async_rs::registers::AdcOptionMsbFlags::ADC_START
            | bq25730_async_rs::registers::AdcOptionMsbFlags::ADC_FULLSCALE,
        lsb_flags: bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_CMPIN
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_VBUS
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_PSYS
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_IIN
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_IDCHG
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_ICHG
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_VSYS
            | bq25730_async_rs::registers::AdcOptionFlags::EN_ADC_VBAT,
    };

    if let Err(e) = bq.set_adc_option(adc_option).await {
        error!("Failed to set ADC option: {:?}", e);
    } else {
        info!("ADC configured for continuous conversion.");
    }

    // 4. 读取电池电压和电流示例 (循环读取)
    info!("--- Reading Battery Data Example (Loop) ---");

    loop {
        info!("--- Reading BQ25730 Data ---");

        // 读取 ChargerStatus 寄存器
        match bq.read_charger_status().await {
            Ok(status) => {
                info!("Charger Status: {:?}", status);
            }
            Err(e) => {
                error!("Failed to read Charger Status: {:?}", e);
            }
        }

        // 读取 ChargeOption0 寄存器
        match bq.read_charge_option0().await {
            Ok(options) => {
                let (lsb, msb) = options.to_msb_lsb_bytes();
                info!(
                    "Charge Option 0 (raw): LSB=0x{:02X}, MSB=0x{:02X}",
                    lsb, msb
                );
            }
            Err(e) => {
                error!("Failed to read Charge Option 0: {:?}", e);
            }
        }

        // 读取 ADCOption 寄存器 (已移除，因为 ADCOption 结构体已从 data_types.rs 中移除)

        // 读取 ADC 测量值
        match bq.read_adc_measurements().await {
            Ok(adc_measurements) => {
                info!("ADC Measurements:");
                info!("  VSYS: {} mV", adc_measurements.vsys.0);
                info!("  VBUS: {} mV", adc_measurements.vbus.0);
                info!("  ICHG: {} mA", adc_measurements.ichg.milliamps);
                info!("  IDCHG: {} mA", adc_measurements.idchg.milliamps);
                info!("  CMPIN: {} mV", adc_measurements.cmpin.0);
                info!("  IIN: {} mA", adc_measurements.iin.milliamps);
                info!("  VBAT: {} mV", adc_measurements.vbat.0);
                info!("  PSYS: {} mW", adc_measurements.psys.0);
            }
            Err(e) => {
                error!("Failed to read ADC measurements: {:?}", e);
            }
        }

        info!("----------------------------");
        Timer::after(Duration::from_secs(1)).await;
    }
}
