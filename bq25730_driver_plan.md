# BQ25730 驱动 CRC 功能移除计划

## 目标：

1.  删除文件 [`src/crc.rs`](src/crc.rs)。
2.  从文件 [`src/lib.rs`](src/lib.rs) 中移除所有与 CRC 相关的内容，包括模块导入、结构体泛型参数、CRC 模式相关的 `impl` 块以及 CRC 计算和验证逻辑。

## 详细计划步骤：

1.  **删除 `src/crc.rs` 文件。**
2.  **修改 `src/lib.rs` 文件：**
    *   **移除模块导入：** 删除 `mod crc;` 和 `pub use crc::{calculate_crc, CrcMode, Disabled, Enabled};` 两行。
    *   **更新 `Bq25730` 结构体定义：**
        *   将 `pub struct Bq25730<I2C, M: CrcMode>` 修改为 `pub struct Bq25730<I2C>`。
        *   删除字段 `_crc_mode: core::marker::PhantomData<M>,`。
    *   **更新 `Bq25730` 构造函数：**
        *   将 `impl<I2C, E> Bq25730<I2C, Disabled>` 修改为 `impl<I2C, E> Bq25730<I2C>`。
        *   将 `pub fn new_without_crc(i2c: I2C, address: u8) -> Self` 修改为 `pub fn new(i2c: I2C, address: u8) -> Self`。
    *   **更新 `RegisterAccess` 实现块：**
        *   将 `impl<I2C, E> RegisterAccess<E> for Bq25730<I2C, Disabled>` 修改为 `impl<I2C, E> RegisterAccess<E> for Bq25730<I2C>`。
    *   **删除 `Enabled` CRC 模式相关实现：** 移除整个 `impl<I2C, E> Bq25730<I2C, Enabled>` 块及其对应的 `impl<I2C, E> RegisterAccess<E> for Bq25730<I2C, Enabled>` 块。
    *   **更新通用 `Bq25730` 实现块：** 将 `impl<I2C, M, E> Bq25730<I2C, M>` 修改为 `impl<I2C, E> Bq25730<I2C>`。

## 流程图：

```mermaid
graph TD
    A[开始] --> B{删除 src/crc.rs};
    B --> C[修改 src/lib.rs];
    C --> C1[移除 CRC 模块导入];
    C1 --> C2[更新 Bq25730 结构体定义];
    C2 --> C3[更新 Bq25730 构造函数];
    C3 --> C4[更新 RegisterAccess 实现块];
    C4 --> C5[删除 Enabled CRC 模式实现];
    C5 --> C6[更新通用 Bq25730 实现块];
    C6 --> D[完成];