/// 手动清除一个定时器
#[macro_export]
macro_rules! clear_timer {
    ($id:expr) => {
        $crate::timer::clear_timer($id)
    };
}
/// 创建一个定时器，功能类似于js的setInterval
#[macro_export]
macro_rules! set_interval {
    ($fn:expr, $ms:expr) => {
        $crate::timer::set_interval($fn, $ms)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};
    fn func() {
        println!("function")
    }
    #[tokio::test]
    async fn test_set_interval_micro() {
        let interval = 1000;
        set_interval!(|| { println!("literal") }, 1000);
        set_interval!(|| { println!("expr") }, interval);
        let closure = || println!("closure");
        set_interval!(closure, 1000);
        let move_value = 1;
        let move_closure = move || println!("move_value: {move_value}");
        set_interval!(move_closure, 1000);
        set_interval!(func, 1000);
        sleep(Duration::from_millis(1300)).await;
    }
    #[tokio::test]
    async fn test_clear_timer_micro() {
        let id = set_interval!(|| { println!("clear_timer") }, 500);
        sleep(Duration::from_millis(1300)).await;
        clear_timer!(id);
        sleep(Duration::from_millis(1300)).await;
    }
}
