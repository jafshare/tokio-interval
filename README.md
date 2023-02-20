# Tokio-Timer

基于 `tokio` 实现了类似于 `js` 的 `setInterval`、`setTimeout` 的功能

目前提供了以下几个宏:

- `set_interval!(cb, ms)` 创建一个定时器，支持传入一个闭包
- `set_interval_async!(|| future, ms)` 创建一个定时器，支持传入一个返回 future 回调
- `set_timeout!(cb, ms)` 创建一个延时器，支持传入一个闭包
- `set_timeout_async!(future, ms)` 创建一个延时器，支持传入一个 future
- `clear_timer!(timer_id)` 清除定时器

注意: 
- 由于实现的逻辑，`set_interval` 受传入的 `fn` 运行时耗的影响，`ms` 不是一个固定的间隔

使用方法:

**Cargo.toml**

```toml
[dependencies]
tokio_interval = "<latest-version>"
```

**main.rs**

```rust
use tokio::time::{sleep,Duration};
use tokio_interval::{set_interval, set_timeout, clear_timer};

#[tokio::main]
async fn main() {
    set_timeout!(|| println!("timeout"), 500);
    set_interval!(|| println!("interval"), 500);
    // 保存id，以便手动删除
    let id = set_interval!(|| println!("clear_interval"), 500);
    sleep(Duration::from_millis(600)).await;
    // 删除定时器
    clear_interval(id);
    // 保证定时器可以继续执行
    sleep(Duration::from_millis(1200)).await;
}
```
