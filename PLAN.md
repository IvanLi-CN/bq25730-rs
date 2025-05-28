# 为所有数据类型支持 `binrw` 的计划

**任务目标：** 为所有数据类型支持 `binrw`，版本 `0.15.0`，作为可选依赖通过 `feature` 启用，并为该特性增加测试用例。

## 计划步骤：

1.  **修改 `Cargo.toml` 文件：**
    *   添加 `binrw` 作为可选依赖，版本为 `0.15.0`。
    *   在 `[features]` 部分添加 `binrw` 特性，并将其与 `dep:binrw` 关联。

2.  **修改 `src/data_types.rs` 文件：**
    *   为所有需要支持 `binrw` 的结构体添加 `#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]` 宏。
    *   根据 `binrw` 的要求，可能需要调整结构体字段的类型或添加 `#[br(map = ...)]` 或 `#[bw(map = ...)]` 属性来处理位字段或特殊转换。特别是对于那些通过 `from_register_value` 和 `to_msb_lsb_bytes` 方法进行转换的类型，需要仔细考虑如何映射到 `binrw` 的读写操作。
    *   对于 `ChargerStatus` 和 `ProchotStatus` 这种包含多个布尔字段的结构体，需要考虑如何将它们打包成字节进行读写。`binrw` 提供了 `#[br(bits_u8)]` 等属性来处理位字段。

3.  **添加测试用例：**
    *   在 `tests/` 目录下创建一个新的测试文件，例如 `tests/binrw_support.rs`。
    *   编写测试用例，确保在启用 `binrw` 特性时，所有数据类型都能正确地进行二进制读写。
    *   测试应包括：
        *   创建结构体实例。
        *   使用 `binrw` 将结构体写入字节数组。
        *   使用 `binrw` 从字节数组读取结构体。
        *   验证读取的结构体与原始结构体是否一致。

## 详细步骤分解和考虑：

### 步骤 1: 修改 `Cargo.toml`

*   在 `[dependencies]` 部分添加：
    ```toml
    binrw = { version = "0.15.0", optional = true }
    ```
*   在 `[features]` 部分添加：
    ```toml
    binrw = ["dep:binrw"]
    ```

### 步骤 2: 修改 `src/data_types.rs`

*   **导入 `binrw` 宏：**
    在文件顶部添加：
    ```rust
    #[cfg(feature = "binrw")]
    use binrw::{BinRead, BinWrite};
    ```
*   **为结构体添加 `derive` 宏：**
    对于每个结构体，在 `#[derive(...)]` 后面添加 `#[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]`。
    例如：
    ```rust
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "defmt", derive(Format))]
    #[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]
    pub struct ChargerStatus {
        // ... 结构体字段，例如：pub stat_ac: bool, pub ico_done: bool, ...
    }
    ```
*   **处理位字段和自定义转换：**
    对于 `ChargerStatus` 和 `ProchotStatus`，它们包含多个布尔字段，需要将它们打包成一个或多个字节。`binrw` 的 `bits` 属性可以帮助实现这一点。
    对于 `ChargeCurrent`, `ChargeVoltage`, `OtgVoltage`, `OtgCurrent`, `InputVoltage`, `VsysMin`, `IinHost`, `IinDpm`, `AdcPsys`, `AdcVbus`, `AdcIdchg`, `AdcIchg`, `AdcCmpin`, `AdcIin`, `AdcVbat`, `AdcVsys` 这些包装了 `u16` 或 `u8` 的结构体，它们有 `from_register_value` 和 `to_msb_lsb_bytes` 方法。这意味着它们内部的 `u16` 或 `u8` 并不是直接的二进制表示，而是需要经过转换。

    我倾向于先尝试使用 `binrw` 的 `derive` 宏和 `bits` 属性来直接处理这些结构体的原始字节表示，尽可能利用 `derive` 宏。如果遇到无法解决的复杂性，我将考虑手动实现 `BinRead` 和 `BinWrite`。

### 步骤 3: 添加测试用例

*   在 `tests/binrw_support.rs` 中，使用 `#[cfg(feature = "binrw")]` 宏来条件编译测试代码。
*   对于每个结构体，编写一个测试函数，确保在启用 `binrw` 特性时，所有数据类型都能正确地进行二进制读写。
*   测试应包括：
    *   创建结构体实例。
    *   使用 `binrw` 将结构体写入字节数组。
    *   使用 `binrw` 从字节数组读取结构体。
    *   验证读取的结构体与原始结构体是否一致。

## 流程图：

```mermaid
graph TD
    A[开始] --> B{读取 Cargo.toml 和 src/data_types.rs};
    B --> C{分析数据类型和现有转换方法};
    C --> D[制定计划];
    D --> E[修改 Cargo.toml];
    E --> F[修改 src/data_types.rs];
    F --> G{处理位字段和复杂转换};
    G -- 尝试 derive 宏和 bits 属性 --> H[添加 #[cfg_attr(feature = "binrw", derive(BinRead, BinWrite))]];
    G -- 如果复杂，手动实现 BinRead/BinWrite 或辅助结构体 --> I[手动实现 BinRead/BinWrite];
    H --> J[添加测试用例];
    I --> J;
    J --> K[将计划写入 PLAN.md];
    K --> L[请求用户确认计划];
    L -- 用户确认 --> M[切换到 Code 模式执行];
    L -- 用户修改 --> D;
