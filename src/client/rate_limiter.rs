#![allow(dead_code)]

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimiterInner>>,
}

struct RateLimiterInner {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new(max_tokens: u32, window: Duration) -> Self {
        let refill_rate = max_tokens as f64 / window.as_secs_f64();
        Self {
            inner: Arc::new(Mutex::new(RateLimiterInner {
                tokens: max_tokens as f64,
                max_tokens: max_tokens as f64,
                refill_rate,
                last_refill: Instant::now(),
            })),
        }
    }

    /// Wait until a token is available, then consume it
    pub async fn acquire(&self) {
        loop {
            let wait_time = {
                let mut inner = self.inner.lock().await;
                inner.refill();
                if inner.tokens >= 1.0 {
                    inner.tokens -= 1.0;
                    return;
                }
                Duration::from_secs_f64(1.0 / inner.refill_rate)
            };
            tokio::time::sleep(wait_time).await;
        }
    }
}

impl RateLimiterInner {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        self.tokens = (self.tokens + elapsed.as_secs_f64() * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
    }
}
