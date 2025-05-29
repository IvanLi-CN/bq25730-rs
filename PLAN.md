# BQ25730 驱动库输入电流计算优化计划

## 问题描述

在 `src/lib.rs` 文件的 `read_adc_measurements` 函数中，输入电流（IIN）的计算 (`AdcIin::from_u8(iin_raw[0])`) 没有考虑 BQ25730 芯片中 `RSNS_RAC` 寄存器设定的采样电阻值，而是使用了硬编码的计算参数。这可能导致在不同硬件配置下电流计算不准确。

## 目标

修改驱动库，使其能够根据 BQ25730 芯片中 `RSNS_RAC` 寄存器的值，动态地使用正确的 LSB 来计算输入电流（IIN），不使用 Offset。同时，优化读取 `RSNS_RAC` 的方式，在初始化时读取并保存，避免重复查询。

## 计划步骤

1.  **修改结构体以存储 `RSNS_RAC`：**
    *   在表示 BQ25730 设备的结构体中（很可能在 `src/lib.rs` 中），添加一个字段来存储 `RSNS_RAC` 的值（布尔类型）。

2.  **在初始化时读取并保存 `RSNS_RAC`：**
    *   在设备的初始化方法中，读取 ChargeOption0 寄存器 (0x01/00h)。
    *   从寄存器值中提取 `RSNS_RAC` 位（第 3 位）。
    *   将提取到的 `RSNS_RAC` 值保存到结构体的新字段中。

3.  **修改 `AdcIin::from_u8` 方法：**
    *   修改方法签名，使其能够接收 `rsns_rac: bool` 参数。
    *   在方法内部，根据传入的 `rsns_rac` 的值，使用以下 LSB 进行电流计算（没有 Offset）：
        *   如果 `rsns_rac` 为 true (5mΩ): `LSB_MA = 100`。
        *   如果 `rsns_rac` 为 false (10mΩ): `LSB_MA = 50`。
    *   计算公式变为：`实际电流 (mA) = raw_value * LSB_MA`。

4.  **在读取 ADC 值时使用保存的 `RSNS_RAC`：**
    *   在 `src/lib.rs` 的 `read_adc_measurements` 函数中，不再读取 ChargeOption0 寄存器。
    *   而是从设备结构体中获取初始化时保存的 `rsns_rac` 值。
    *   将获取到的 `rsns_rac` 值传递给 `AdcIin::from_u8` 方法。

5.  **更新 `AdcIin` 结构体和相关方法：**
    *   检查 `AdcIin` 结构体和相关的 `to_u16` 和 `to_msb_lsb_bytes` 方法是否需要根据 LSB 的变化进行调整。

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
```

---

# 移除 tests/common.rs 并重构测试代码计划

## 问题描述

`tests/common.rs` 文件包含一些辅助函数，被多个测试文件通过 `include!` 宏引用。这使得代码结构不够清晰，且不利于单独编译和测试。

## 目标

移除 `tests/common.rs` 文件，并将其中定义的辅助函数逻辑直接内联到使用它们的测试文件中，直接使用 `embedded_hal_mock::eh1::i2c` 提供的功能。

## 受影响文件

-   [`tests/init_and_basic.rs`](tests/init_and_basic.rs)
-   [`tests/charge_control.rs`](tests/charge_control.rs)
-   [`tests/otg_control.rs`](tests/otg_control.rs)
-   [`tests/status_and_adc.rs`](tests/status_and_adc.rs)

## 计划步骤

1.  **重构 [`tests/init_and_basic.rs`](tests/init_and_basic.rs)**:
    *   移除 `include!("common.rs");`。
    *   在文件顶部添加 `use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};`。
    *   将 `new_bq25730_with_mock` 的逻辑替换为直接创建 `I2cMock` 和 `Bq25730` 实例。
    *   将 `read_registers_transaction`, `write_registers_transaction`, `read_register_transaction`, `write_register_transaction` 的调用替换为直接使用 `I2cTransaction::write` 或 `I2cTransaction::write_read`。
    *   将 `assert_invalid_data_error` 的逻辑替换为直接的 `match` 语句或 `assert_eq!`。

2.  **重构 [`tests/charge_control.rs`](tests/charge_control.rs)**:
    *   移除 `include!("common.rs");`。
    *   在文件顶部添加 `use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};`。
    *   将 `new_bq25730_with_mock` 的逻辑替换为直接创建 `I2cMock` 和 `Bq25730` 实例。
    *   将 `write_registers_transaction` 的调用替换为直接使用 `I2cTransaction::write`。

3.  **重构 [`tests/otg_control.rs`](tests/otg_control.rs)**:
    *   移除 `include!("common.rs");`。
    *   在文件顶部添加 `use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};`。
    *   将 `new_bq25730_with_mock` 的逻辑替换为直接创建 `I2cMock` 和 `Bq25730` 实例。
    *   将 `write_registers_transaction` 和 `read_registers_transaction` 的调用替换为直接使用 `I2cTransaction::write` 或 `I2cTransaction::write_read`。

4.  **重构 [`tests/status_and_adc.rs`](tests/status_and_adc.rs)**:
    *   移除 `include!("common.rs");`。
    *   在文件顶部添加 `use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};`。
    *   将 `new_bq25730_with_mock` 的逻辑替换为直接创建 `I2cMock` 和 `Bq25730` 实例。
    *   将 `read_registers_transaction` 的调用替换为直接使用 `I2cTransaction::write_read`。
    *   将 `assert_i2c_error` 的逻辑替换为直接的 `match` 语句或 `assert_eq!`。

5.  **删除 [`tests/common.rs`](tests/common.rs) 文件。**

6.  **运行测试用例并修复可能出现的问题。**

## 重构流程示意图

```mermaid
graph TD
    A[开始] --> B{确定依赖 common.rs 的文件};
    B --> C[tests/init_and_basic.rs];
    B --> D[tests/charge_control.rs];
    B --> E[tests/otg_control.rs];
    B --> F[tests/status_and_adc.rs];
    C --> G[重构 tests/init_and_basic.rs];
    D --> H[重构 tests/charge_control.rs];
    E --> I[重构 tests/otg_control.rs];
    F --> J[重构 tests/status_and_adc.rs];
    G, H, I, J --> K[删除 tests/common.rs];
    K --> L[运行测试];
    L --> M{测试通过?};
    M -- 是 --> N[完成];
    M -- 否 --> O[修复问题];
    O --> L;
