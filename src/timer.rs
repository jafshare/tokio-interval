use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::{interval_at, sleep, Duration, Instant};
#[allow(dead_code)]
static TIMER_ID: Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(0));
#[allow(dead_code)]
static TIMERS: Lazy<Mutex<HashMap<u64, JoinHandle<()>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
/// 清除指定的定时器
pub(crate) fn _clear_timer(id: u64) {
    let mut timer_map = TIMERS.lock().unwrap();
    // 如果定时器不存在，则提前释放锁
    if !timer_map.contains_key(&id) {
        return;
    }
    let handler = timer_map.get(&id).unwrap();
    // 如果未完成，则停止
    if !handler.is_finished() {
        handler.abort();
    }
    // 从map中删除对应的数据
    timer_map.remove(&id);
}
/// 开启定时器，指定ms后,执行传入的回调函数
pub(crate) fn _set_interval<F: Fn() + Send + 'static>(f: F, ms: u64) -> u64 {
    let start = Instant::now() + Duration::from_millis(ms);
    let period = Duration::from_millis(ms);
    let handler: JoinHandle<()> = tokio::spawn(async move {
        let mut int = interval_at(start, period);
        loop {
            int.tick().await;
            f();
        }
    });
    // 保存timer数据
    let id = TIMER_ID.fetch_add(1, Ordering::SeqCst);
    TIMERS.lock().unwrap().insert(id, handler);
    id
}
/// 定时器的异步版本,指定ms后，执行传入的Future
pub(crate) fn _set_interval_async<
    F: (Fn() -> Fut) + Sync + Send + 'static,
    Fut: Future + Sync + Send + 'static,
>(
    f: F,
    ms: u64,
) -> u64 {
    let start = Instant::now() + Duration::from_millis(ms);
    let period = Duration::from_millis(ms);
    let handler: JoinHandle<()> = tokio::spawn(async move {
        let mut int = interval_at(start, period);
        loop {
            int.tick().await;
            f().await;
        }
    });
    // 保存timer数据
    let id = TIMER_ID.fetch_add(1, Ordering::SeqCst);
    TIMERS.lock().unwrap().insert(id, handler);
    id
}
/// 延时器
pub(crate) fn _set_timeout<F: Fn() + Send + 'static>(f: F, ms: u64) -> u64 {
    let delay = Duration::from_millis(ms);
    let id = TIMER_ID.fetch_add(1, Ordering::SeqCst);
    let handler = tokio::spawn(async move {
        sleep(delay).await;
        f();
        // 执行完毕后自动清除定时器
        _clear_timer(id);
    });
    // 保存timer数据
    TIMERS.lock().unwrap().insert(id, handler);
    id
}
/// 延时器的异步版本
pub(crate) fn _set_timeout_async<
    F: (Fn() -> Fut) + Send + Sync + 'static,
    Fut: Future + Send + Sync + 'static,
>(
    f: F,
    ms: u64,
) -> u64 {
    let delay = Duration::from_millis(ms);
    let id = TIMER_ID.fetch_add(1, Ordering::SeqCst);
    let handler = tokio::spawn(async move {
        sleep(delay).await;
        f().await;
        // 执行完毕后，自动清除定时器
        _clear_timer(id);
    });
    // 保存timer数据
    TIMERS.lock().unwrap().insert(id, handler);
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    #[tokio::test]
    async fn test_set_interval() {
        let times = 3;
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            _set_interval(
                move || {
                    counter.clone().fetch_add(1, Ordering::SeqCst);
                },
                1 * 1000,
            );
        }
        {
            let counter = counter.clone();
            _set_interval(
                move || {
                    counter.clone().fetch_add(1, Ordering::SeqCst);
                },
                1 * 1000,
            );
        }
        assert_eq!(TIMERS.lock().unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(times * 1100)).await;
        assert_eq!(counter.load(Ordering::SeqCst), times * 2);
    }
    #[tokio::test]
    async fn test_set_interval_async() {
        let times = 3;
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            _set_interval_async(closure_async, 1 * 1000);
        }
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            _set_interval_async(closure_async, 1 * 1000);
        }
        assert_eq!(TIMERS.lock().unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(times * 1100)).await;
        assert_eq!(counter.load(Ordering::SeqCst), times * 2);
    }
    #[tokio::test]
    async fn test_set_timeout() {
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            _set_timeout(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                1000,
            );
        }
        {
            let counter = counter.clone();
            _set_timeout(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                1000,
            );
        }
        assert_eq!(TIMERS.lock().unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(TIMERS.lock().unwrap().len(), 0);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
    #[tokio::test]
    async fn test_set_timeout_async() {
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            _set_timeout_async(closure_async, 1000);
        }
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            _set_timeout_async(closure_async, 1000);
        }
        assert_eq!(TIMERS.lock().unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(1100)).await;
        assert_eq!(TIMERS.lock().unwrap().len(), 0);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
    #[tokio::test]
    async fn test_clear_timer() {
        let times = 3;
        let counter = Arc::new(AtomicU64::new(0));
        {
            let counter = counter.clone();
            _set_interval(
                move || {
                    counter.clone().fetch_add(1, Ordering::SeqCst);
                },
                1 * 1000,
            );
        }
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            let id = _set_interval_async(closure_async, 1 * 1000);
            _clear_timer(id);
        }
        {
            let counter = counter.clone();
            let id = _set_timeout(
                move || {
                    counter.fetch_add(1, Ordering::SeqCst);
                },
                1000,
            );
            _clear_timer(id);
        }
        {
            let counter = counter.clone();
            let closure_async = move || {
                let counter_inner = counter.clone();
                async move {
                    counter_inner.fetch_add(1, Ordering::SeqCst);
                }
            };
            _set_timeout_async(closure_async, 1000);
        }
        assert_eq!(TIMERS.lock().unwrap().len(), 2);
        tokio::time::sleep(Duration::from_millis(times * 1100)).await;
        assert_eq!(TIMERS.lock().unwrap().len(), 1);
        assert_eq!(counter.load(Ordering::SeqCst), times * 1 + 1);
    }
}
