## 创建 STM32G4 示例项目计划 (最终修订版 - 针对 bq25730-rs)

**目标**: 在 `examples` 目录下创建一个全新的 `stm32g4` 项目，作为使用 `stm32g431cbu6` MCU 调用 `bq25730-rs` 库的示例代码，清晰展示充电控制、放电控制和电池数据读取功能。

### 计划步骤

#### 1. 准备工作 (由 Architect 模式完成)

*   **确认 `example-stm32g4` 的完整结构**: 确认了 `example-stm32g4` 包含 `Cargo.toml`、`src/main.rs`、`.cargo/config.toml` 和 `Cargo.lock`。未找到 `memory.x` 文件，假设其由依赖项处理或自动生成。
*   **获取必要文件内容**:
    *   已读取 `example-stm32g4/.cargo/config.toml` 的内容。
    *   已读取 `example-stm32g4/Cargo.lock` 的内容。
*   **确认 `bq25730-rs` 库的 API**: 已通过 `list_code_definition_names` 确认了 `bq25730-rs` 库的 API，包括 `Bq25730` 结构体及其相关方法，以及 `ChargeCurrent`、`ChargeVoltage`、`AdcMeasurements` 等数据类型。

#### 2. 项目初始化 (由 Code 模式完成)

*   **确认源目录内容**: 使用 `ls -a example-stm32g4` 命令，确认 `example-stm32g4` 目录的完整内容，包括隐藏文件。
*   **清理旧目录**: 确保 `examples/stm32g4` 目录是空的或不存在。如果存在，先删除。
*   **创建目录结构**:
    *   在 `examples` 目录下创建 `stm32g4` 目录。
    *   在 `examples/stm32g4` 目录下创建 `src` 目录。
    *   在 `examples/stm32g4` 目录下创建 `.cargo` 目录。
*   **创建 `Cargo.toml`**:
    *   创建一个全新的 `examples/stm32g4/Cargo.toml` 文件。
    *   `package.name` 设置为 `bq25730_stm32g431cbu6_example`。
    *   `edition` 设置为 `2024`。
    *   `dependencies` 部分只包含 `embassy-stm32` (features: `defmt`, `time-driver-any`, `stm32g431cb`, `memory-x`, `unstable-pac`, `exti`)、`embassy-embedded-hal`、`embassy-executor` (features: `arch-cortex-m`, `executor-thread`, `defmt`)、`embassy-time` (features: `defmt`, `defmt-timestamp-uptime`, `tick-hz-32_768`)、`embassy-sync` (features: `defmt`)、`defmt`、`defmt-rtt`、`cortex-m` (features: `inline-asm`, `critical-section-single-core`)、`cortex-m-rt`、`embedded-hal`、`panic-probe` (features: `print-defmt`)、`heapless` (default-features: `false`)、`portable-atomic` (features: `critical-section`)、`static_cell`、`libm`、`uom` (default-features: `false`, features: `si`)。
    *   **关键修改**: `bq25730-rs` 依赖的 `path` 设置为 `../../`。
    *   `profile.dev` 和 `profile.release` 部分与 `example-stm32g4/Cargo.toml` 保持一致。
*   **创建 `.cargo/config.toml`**:
    *   将 `example-stm32g4/.cargo/config.toml` 的内容写入 `examples/stm32g4/.cargo/config.toml`。
*   **创建 `Cargo.lock`**:
    *   将 `example-stm32g4/Cargo.lock` 的内容写入 `examples/stm32g4/Cargo.lock`。
*   **创建 `src/main.rs`**:
    *   创建一个全新的 `examples/stm32g4/src/main.rs` 文件。
    *   包含 `#![no_std]` 和 `#![no_main]`。
    *   包含必要的 `use` 声明，特别是 `use bq25730_rs::{...};`。
    *   `main` 函数中包含以下逻辑：
        *   MCU 初始化。
        *   I2C 初始化。
        *   `Bq25730` 实例创建和初始化。
        *   **充电控制示例**: 调用 `bq.set_charge_current()` 和 `bq.set_charge_voltage()`。
        *   **放电控制示例**: 调用 `bq.enter_ship_mode()` 或其他相关方法（如果 `ChargeOption1` 中有更合适的放电控制位）。
        *   **读取电池电压和电流示例**: 在一个循环中持续读取并打印电芯电压、总电压和电流（使用 `read_adc_measurements` 和 `read_charge_current` 等）。
        *   添加清晰的 `info!` 消息和错误处理。

#### 3. 验证与构建 (由 Code 模式完成)

*   **构建项目**: 在 `examples/stm32g4` 目录下运行 `cargo build` 命令，验证项目是否能成功构建。

### 流程图

```mermaid
graph TD
    A[用户请求创建STM32G4示例项目] --> B{Architect 模式};
    B --> C{重新制定详细计划};
    C --> D{向用户展示修订后的计划并征求同意};
    D -- 用户同意 --> E{询问是否写入Markdown文件};
    E -- 用户同意 --> F{写入PLAN.md};
    F --> G{切换到Code模式};
    G --> H{Code模式执行计划};
    H --> H1{确认源目录内容 (ls -a)};
    H1 --> I{清理旧目录};
    I --> J{创建目录结构};
    J --> K{创建Cargo.toml};
    K --> L{创建.cargo/config.toml};
    L --> M{创建Cargo.lock};
    M --> N{创建src/main.rs并实现示例功能};
    N --> O{构建项目};
    O --> P[完成];
