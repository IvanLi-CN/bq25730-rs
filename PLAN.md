# BQ25730 CI 编译错误修复计划

## 目标

解决 CI 报告的编译错误，包括未解析的模块、未链接的 crate 以及 `info` 和 `error` 宏找不到的问题，并处理 `panic_handler` 缺失的错误。

## 步骤

### 1. 修改 `examples/stm32g031_bq25730/Cargo.toml`

* **更新 `embassy` 依赖的 `rev`：**
    由于 `architect` 模式无法执行 `git` 命令获取最新 `rev`，此步骤将暂时跳过。如果后续编译仍有问题，可能需要手动更新 `rev` 或切换到 `code` 模式获取。
* **调整 `cortex-m-rt` feature：**
    将 `cortex-m-rt` 的 feature 从 `"critical-section-single-core"` 更改为 `"critical-section-handlers"`，以确保 panic handler 能够正确链接。
* **配置 `defmt` 相关依赖：**
  * 将 `defmt-rtt` 和 `rtt-target` 标记为 `optional = true`。
  * 将 `panic-probe` 标记为 `optional = true`。
  * 在 `[features]` 部分添加一个新的 `defmt` feature，并将其依赖项设置为 `dep:defmt`, `dep:defmt-rtt`, `dep:rtt-target`, `dep:panic-probe`。这将允许通过 feature 统一管理 `defmt` 相关的依赖。

### 2. 修改 `examples/stm32g031_bq25730/src/main.rs`

* **删除未使用的导入：**
    移除 `use bq25730_async_rs::data_types::*` 和 `use bq25730_async_rs::registers::*`，因为它们在代码中未被直接使用，仅产生警告。
* **修改 `cortex_m_rt` 导入：**
    将 `use cortex_m_rt as _;` 更改为 `use cortex_m_rt::{entry, exception};`，以便正确使用 `entry` 和 `exception` 宏。
* **添加 `panic_handler` 和 `HardFault` 异常处理：**
    在文件末尾添加以下代码块，以提供 `no_std` 环境所需的 panic handler 和 HardFault 异常处理：

    ```rust
    #[defmt::panic_handler]
    fn panic() -> ! {
        cortex_m::asm::udf();
    }

    #[exception]
    unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
        cortex_m::asm::udf();
    }
    ```

## 流程图

```mermaid
graph TD
    A[开始] --> B{读取 examples/stm32g031_bq25730/Cargo.toml};
    B --> C{读取 Cargo.toml (根目录)};
    C --> D{读取 examples/stm32g031_bq25730/src/main.rs};
    D --> E{分析 CI 错误报告};
    E --> F{识别缺失依赖和配置问题};
    F --> G{向用户确认计划};
    G -- 用户同意 --> H{询问是否写入Markdown文件};
    H -- 用户同意 --> I{写入 PLAN.md};
    I --> J{切换到 Code 模式};
    J --> K{在 Code 模式下执行文件修改};
    K --> L{完成};
