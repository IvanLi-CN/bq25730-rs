#![no_std]
#![no_main]
#![allow(unused_imports)] // 允许未使用的导入，以消除 AdcMeasurements 的警告

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, Config, I2c},
    peripherals,
    time::Hertz,
};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

use bq25730_async_rs::{
    Bq25730,
    data_types::{AdcMeasurements, ChargeCurrent, ChargeVoltage},
};
use uom::si::{
    Quantity, SI,
    electrical_resistance::{Dimension, ElectricalResistance, milliohm},
};

bind_interrupts!(struct Irqs {
    I2C1 => i2c::EventInterruptHandler<peripherals::I2C1>, i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello from STM32G031!");

    let mut config = Config::default();
    config.scl_pullup = true;
    config.sda_pullup = true;

    let i2c = I2c::new(
        p.I2C1,
        p.PB6, // Keep PB6 for STM32G0
        p.PB7, // Keep PB7 for STM32G0
        Irqs,
        p.DMA1_CH1, // Keep DMA1_CH1 for STM32G0
        p.DMA1_CH2, // Keep DMA1_CH2 for STM32G0
        Hertz(100_000),
        config,
    );

    let mut bq = Bq25730::new(i2c, 0x6B, 4); // BQ25730 I2C 地址是 0x6B，电池节数 4

    // 1. 初始化 BQ25730
    info!("Initializing BQ25730...");
    if let Err(e) = bq.init().await {
        error!("Failed to initialize BQ25730: {:?}", e);
        // core::panic!("Failed to initialize BQ25730: {:?}", e); // 在示例中不直接panic
    }
    info!("BQ25730 initialization complete.");

    // 设置检流电阻：VBUS侧 10mΩ (RSNS_RAC = 0b), VBAT侧 5mΩ (RSNS_RSR = 1b)
    // ChargeOption1 MSB (0x31) 默认值 0x3F (0b00111111)
    // RSNS_RAC (bit 3) = 0, RSNS_RSR (bit 2) = 1
    // 0b00111111 & ~(1 << 3) | (1 << 2) = 0b00110111 (0x37)
    let charge_option1_msb: u8 = 0x37;
    let charge_option1_lsb: u8 = 0x00; // LSB 保持默认
    if let Err(e) = bq
        .set_charge_option1(
            bq25730_async_rs::data_types::ChargeOption1::from_register_value(
                charge_option1_lsb,
                charge_option1_msb,
            ),
        )
        .await
    {
        error!("Failed to set charge option 1: {:?}", e);
    } else {
        info!("Charge option 1 set for sense resistors.");
    }

    // 禁用低功耗模式 (EN_LWPWR = 0b)，启用性能模式以使 ADC 可用
    // ChargeOption0 MSB (0x01) 默认值 0xE7 (0b11100111)
    // EN_LWPWR (bit 7) = 0
    // WDTMR_ADJ (bit 6:5) = 00b (禁用看门狗定时器)
    let charge_option0_msb: u8 = 0x27; // 0b00100111 (EN_LWPWR=0, WDTMR_ADJ=00, 其他保持默认)
    let charge_option0_lsb: u8 = 0x0E; // 0b00001110 (保持 IBAT_GAIN, EN_LDO, EN_IIN_DPM 默认启用)
    if let Err(e) = bq
        .set_charge_option0(
            bq25730_async_rs::data_types::ChargeOption0::from_register_value(
                charge_option0_lsb,
                charge_option0_msb,
            ),
        )
        .await
    {
        error!("Failed to set charge option 0: {:?}", e);
    } else {
        info!("Charge option 0 set for performance mode and watchdog disabled.");
    }

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
    let charge_current = ChargeCurrent(512); // Directly set the raw value in mA
    if let Err(e) = bq.set_charge_current(charge_current).await {
        error!("Failed to set charge current: {:?}", e);
    } else {
        info!("Charge current set to {} mA.", charge_current.0);
    }

    // 设置充电电压为 18000 mV (5 节磷酸铁锂电池，每节 3.6V)
    let charge_voltage = ChargeVoltage(18000); // Directly set the raw value in mV
    if let Err(e) = bq.set_charge_voltage(charge_voltage).await {
        error!("Failed to set charge voltage: {:?}", e);
    } else {
        info!("Charge voltage set to {} mV.", charge_voltage.0);
    }
    info!("Charging control example complete.");

    // 3. 配置并启用 ADC 进行连续转换
    info!("--- Configuring and Enabling ADC ---");
    let adc_option = bq25730_async_rs::data_types::AdcOption {
        adc_conv: true,      // 连续转换
        adc_start: true,     // 启动转换
        adc_fullscale: true, // 3.06V 满量程
        en_adc_cmpin: true,  // 启用所有通道
        en_adc_vbus: true,
        en_adc_psys: true,
        en_adc_iin: true,
        en_adc_idchg: true,
        en_adc_ichg: true,
        en_adc_vsys: true,
        en_adc_vbat: true,
    };

    if let Err(e) = bq.set_adc_option(adc_option).await {
        error!("Failed to set ADC option: {:?}", e);
    } else {
        info!("ADC configured for continuous conversion.");
    }

    // 4. 读取电池电压和电流示例 (循环读取)
    info!("--- Reading Battery Data Example (Loop) ---");
    // 检流电阻配置：VBUS侧 10mΩ，VBAT侧 5mΩ
    let _sense_resistor_vbus: Quantity<Dimension, SI<f32>, f32> =
        ElectricalResistance::new::<milliohm>(10.0);
    let _sense_resistor_vbat: Quantity<Dimension, SI<f32>, f32> =
        ElectricalResistance::new::<milliohm>(5.0);

    loop {
        info!("--- Reading BQ25730 Data ---");

        // 读取 ChargerStatus 寄存器
        match bq.read_charger_status().await {
            Ok(status) => {
                info!("Charger Status:");
                info!("  STAT_AC: {}", status.stat_ac);
                info!("  IN_OTG: {}", status.in_otg);
                info!("  IN_VAP: {}", status.in_vap);
                info!("  IN_FCHRG: {}", status.in_fchrg);
                info!("  IN_PCHRG: {}", status.in_pchrg);
                info!("  Fault VSYS_UVP: {}", status.fault_vsys_uvp);
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
                info!("  ICHG: {} mA", adc_measurements.ichg.0);
                info!("  IDCHG: {} mA", adc_measurements.idchg.0);
                info!("  CMPIN: {} mV", adc_measurements.cmpin.0);
                info!("  IIN: {} mA", adc_measurements.iin.0);
                info!("  VBAT: {} mV", adc_measurements.vbat.0);
                info!("  PSYS: {} mW", adc_measurements.psys.0);
            }
            Err(e) => {
                error!("Failed to read ADC measurements: {:?}", e);
            }
        }

        // 读取充电电流 (如果需要单独读取)
        /*
        match bq.read_charge_current().await {
            Ok(current) => {
                info!("Charge Current Register: {} mA", current.0);
            }
            Err(e) => {
                error!("Failed to read charge current register: {:?}", e);
            }
        }
        */

        // 读取充电电压 (如果需要单独读取)
        /*
        match bq.read_charge_voltage().await {
            Ok(voltage) => {
                info!("Charge Voltage Register: {} mV", voltage.0);
            }
            Err(e) => {
                error!("Failed to read charge voltage register: {:?}", e);
            }
        }
        */

        info!("----------------------------");
        Timer::after(Duration::from_secs(1)).await;
    }
}
