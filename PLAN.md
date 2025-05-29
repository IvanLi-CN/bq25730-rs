# BQ25730 驱动库输入电流计算优化计划

## 问题描述

在 `src/lib.rs` 文件的 `read_adc_measurements` 函数中，输入电流（IIN）的计算 (`AdcIin::from_u8(iin_raw[0])`) 没有考虑 BQ25730 芯片中 `RSNS_RAC` 寄存器设定的采样电阻值，而是使用了硬编码的计算参数。这可能导致在不同硬件配置下电流计算不准确。

## 目标

修改驱动库，使其能够根据 BQ25730 芯片中 `RSNS_RAC` 寄存器的值，动态地使用正确的 LSB 来计算输入电流（IIN），不使用 Offset。同时，优化读取 `RSNS_RAC` 的方式，在初始化时读取并保存，避免重复查询。

## 计划步骤

1. **修改结构体以存储 `RSNS_RAC`：**
    * 在表示 BQ25730 设备的结构体中（很可能在 `src/lib.rs` 中），添加一个字段来存储 `RSNS_RAC` 的值（布尔类型）。

2. **在初始化时读取并保存 `RSNS_RAC`：**
    * 在设备的初始化方法中，读取 ChargeOption0 寄存器 (0x01/00h)。
    * 从寄存器值中提取 `RSNS_RAC` 位（第 3 位）。
    * 将提取到的 `RSNS_RAC` 值保存到结构体的新字段中。

3. **修改 `AdcIin::from_u8` 方法：**
    * 修改方法签名，使其能够接收 `rsns_rac: bool` 参数。
    * 在方法内部，根据传入的 `rsns_rac` 的值，使用以下 LSB 进行电流计算（没有 Offset）：
        * 如果 `rsns_rac` 为 true (5mΩ): `LSB_MA = 100`。
        * 如果 `rsns_rac` 为 false (10mΩ): `LSB_MA = 50`。
    * 计算公式变为：`实际电流 (mA) = raw_value * LSB_MA`。

4. **在读取 ADC 值时使用保存的 `RSNS_RAC`：**
    * 在 `src/lib.rs` 的 `read_adc_measurements` 函数中，不再读取 ChargeOption0 寄存器。
    * 而是从设备结构体中获取初始化时保存的 `rsns_rac` 值。
    * 将获取到的 `rsns_rac` 值传递给 `AdcIin::from_u8` 方法。

5. **更新 `AdcIin` 结构体和相关方法：**
    * 检查 `AdcIin` 结构体和相关的 `to_u16` 和 `to_msb_lsb_bytes` 方法是否需要根据 LSB 的变化进行调整。

## 修改思路示意图

```mermaid
graph TD
    A[初始化设备] --> B{读取 ChargeOption0 寄存器};
    B --> C{提取 RSNS_RAC 位};
    C --> D[保存 RSNS_RAC 到结构体];
    E[调用 read_adc_measurements] --> F[从结构体获取保存的 RSNS_RAC];
    E --> G[获取 ADCIIN 原始值];
    F, G --> H{调用 AdcIin::from_u8(raw_value, rsns_rac)};
    H --> I{根据 rsns_rac 选择 LSB};
    I --> J[计算实际电流];
    J --> K[返回 AdcIin 结构体];
