/// 创建一个定时器，功能类似于js的setInterval
/// # Example
/// ```rust
/// use tokio::time::{sleep, Duration};
/// use tokio_interval::set_interval;
///
/// #[tokio::main]
/// async fn main() {
///     set_interval!(|| { println!("literal") }, 1000);
///     let interval = 1000;
///     // ms 支持表达式
///     set_interval!(|| { println!("expr") }, interval);
///     let closure = || println!("closure");
///     // 支持一个闭包
///     set_interval!(closure, 1000);
///     let move_value = 1;
///     let move_closure = move || println!("move_value: {move_value}");
///     set_interval!(move_closure, 1000);
///     fn func() {
///         println!("function")
///     }
///     // 支持函数
///     set_interval!(func, 1000);
///     sleep(Duration::from_millis(1300)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_interval {
    ($fn:expr, $ms:expr) => {
        $crate::timer::_set_interval($fn, $ms)
    };
}
/// 创建一个支持传入异步的定时器
/// # Example
/// ```rust
/// use tokio::time::{sleep, Duration};
/// use tokio_interval::set_interval_async;
///
/// #[tokio::main]
/// async fn main() {
///     set_interval_async!(|| async { println!("literal") }, 1000);
///     let interval = 1000;
///     // ms 支持表达式
///     set_interval_async!(|| async { println!("expr") }, interval);
///     let closure = || async { println!("closure") };
///     // 支持传入一个闭包
///     set_interval_async!(closure, 1000);
///     let move_value = 1;
///     let move_closure = move || async move { println!("move_value: {move_value}") };
///     set_interval_async!(move_closure, 1000);
///     sleep(Duration::from_millis(1300)).await;
/// }
#[macro_export]
macro_rules! set_interval_async {
    ($fn:expr, $ms:expr) => {
        $crate::timer::_set_interval_async($fn, $ms)
    };
}

/// 创建一个延时器，功能类似于js的setTimeout
/// # Example
/// ```rust
/// use tokio::time::{sleep, Duration};
/// use tokio_interval::set_timeout;
///
/// #[tokio::main]
/// async fn main() {
///     set_timeout!(|| { println!("literal") }, 1000);
///     let timeout = 1000;
///     // ms 支持表达式
///     set_timeout!(|| { println!("expr") }, timeout);
///     let closure = || println!("closure");
///     // 支持一个闭包
///     set_timeout!(closure, 1000);
///     let move_value = 1;
///     let move_closure = move || println!("move_value: {move_value}");
///     set_timeout!(move_closure, 1000);
///     fn func() {
///         println!("function")
///     }
///     // 支持函数
///     set_timeout!(func, 1000);
///     sleep(Duration::from_millis(1300)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_timeout {
    ($fn:expr, $ms:expr) => {
        $crate::timer::_set_timeout($fn, $ms)
    };
}

/// 创建一个支持传入异步的延时器
/// # Example
/// ```rust
/// use tokio::time::{sleep, Duration};
/// use tokio_interval::set_timeout_async;
///
/// #[tokio::main]
/// async fn main() {
///     // 支持直接传入一个 Future
///     set_timeout_async!(async { println!("future") }, 1000);
///     set_timeout_async!(|| async { println!("literal") }, 1000);
///     let interval = 1000;
///     // ms 支持表达式
///     set_timeout_async!(|| async { println!("expr") }, interval);
///     let closure = || async { println!("closure") };
///     // 支持传入一个闭包
///     set_timeout_async!(closure, 1000);
///     let move_value = 1;
///     let move_closure = move || async move { println!("move_value: {move_value}") };
///     set_timeout_async!(move_closure, 1000);
///     sleep(Duration::from_millis(1300)).await;
/// }
/// ```
#[macro_export]
macro_rules! set_timeout_async {
    (async $future:block, $ms:expr) => {
        $crate::timer::_set_timeout_async(|| async { $future }, $ms)
    };
    ($cb:expr, $ms:expr) => {
        $crate::timer::_set_timeout_async($cb, $ms)
    };
}

/// 手动清除一个定时器
/// # Example
/// ```rust
/// use tokio::time::{sleep, Duration};
/// use tokio_interval::set_interval;
///
/// #[tokio::main]
/// async fn main(){
///     // 调用 set_interval 会返回 timer_id
///     let id = set_interval!(|| { println!("clear_timer") }, 500);
///     sleep(Duration::from_millis(1300)).await;
///     clear_timer!(id);
///     sleep(Duration::from_millis(1300)).await;
/// }
/// ```
#[macro_export]
macro_rules! clear_timer {
    ($id:expr) => {
        $crate::timer::_clear_timer($id)
    };
}
#[cfg(test)]
mod tests {
    use tokio::time::{sleep, Duration};
    #[tokio::test]
    async fn test_set_interval_micro() {
        set_interval!(|| println!("literal"), 1000);
        let interval = 1000;
        // ms 支持表达式
        set_interval!(|| { println!("expr") }, interval);
        let closure = || println!("closure");
        // 支持传入一个闭包
        set_interval!(closure, 1000);
        let move_value = 1;
        let move_closure = move || println!("move_value: {move_value}");
        set_interval!(move_closure, 1000);
        fn func() {
            println!("function")
        }
        // 支持传入一个函数
        set_interval!(func, 1000);
        sleep(Duration::from_millis(1300)).await;
    }
    #[tokio::test]
    async fn test_set_interval_async_micro() {
        set_interval_async!(|| async { println!("literal") }, 1000);
        let interval = 1000;
        // ms 支持表达式
        set_interval_async!(|| async { println!("expr") }, interval);
        let closure = || async { println!("closure") };
        // 支持传入一个闭包
        set_interval_async!(closure, 1000);
        let move_value = 1;
        let move_closure = move || async move { println!("move_value: {move_value}") };
        set_interval_async!(move_closure, 1000);
        sleep(Duration::from_millis(1300)).await;
    }
    #[tokio::test]
    async fn test_set_timeout_micro() {
        set_timeout!(|| { println!("literal") }, 1000);
        let timeout = 1000;
        // ms 支持表达式
        set_timeout!(|| { println!("expr") }, timeout);
        let closure = || println!("closure");
        // 支持传入一个闭包
        set_timeout!(closure, 1000);
        let move_value = 1;
        let move_closure = move || println!("move_value: {move_value}");
        set_timeout!(move_closure, 1000);
        fn func() {
            println!("function")
        }
        // 支持传入一个函数
        set_timeout!(func, 1000);
        sleep(Duration::from_millis(1300)).await;
    }
    #[tokio::test]
    async fn test_set_timeout_async_micro() {
        // 支持直接传入一个 Future
        set_timeout_async!(async { println!("future") }, 1000);
        set_timeout_async!(|| async { println!("literal") }, 1000);
        let interval = 1000;
        // ms 支持表达式
        set_timeout_async!(|| async { println!("expr") }, interval);
        let closure = || async { println!("closure") };
        // 支持传入一个闭包
        set_timeout_async!(closure, 1000);
        let move_value = 1;
        let move_closure = move || async move { println!("move_value: {move_value}") };
        set_timeout_async!(move_closure, 1000);
        sleep(Duration::from_millis(1300)).await;
    }
    #[tokio::test]
    async fn test_clear_timer_micro() {
        let id = set_interval!(|| { println!("clear_timer") }, 500);
        let id1 = set_timeout!(|| { println!("clear_timer1") }, 1500);
        sleep(Duration::from_millis(1300)).await;
        clear_timer!(id);
        clear_timer!(id1);
        sleep(Duration::from_millis(1300)).await;
    }
}
