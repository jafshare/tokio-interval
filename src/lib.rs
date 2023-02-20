//!
//! 基于 `tokio` 实现了类似于 `js` 的 `setInterval`、`setTimeout` 的功能(由于实现的逻辑，`set_interval` 受传入的 `fn` 运行时耗的影响)
//!
//! 目前提供了以下几个宏:
//! - `set_interval!(cb, ms)` 创建一个定时器，支持传入一个闭包
//! - `set_interval_async!(|| future, ms)` 创建一个定时器，支持传入一个返回 future 回调
//! - `set_timeout!(cb, ms)` 创建一个延时器，支持传入一个闭包
//! - `set_timeout_async!(future, ms)` 创建一个延时器，支持传入一个future
//! - `clear_timer!(timer_id)` 清除定时器
//!
//! 注意:
//! - 由于实现的逻辑，`set_interval` 、`set_interval_async` 受传入的 `fn` 运行时耗的影响，`ms` 不是一个固定的间隔
//!
mod micros;
mod timer;

pub use micros::*;
