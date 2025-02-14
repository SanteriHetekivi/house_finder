/// Limiter for waiting between calls.
pub(crate) struct BetweenCalls {
    pub(self) duration: tokio::time::Duration,
    pub(self) last_call_time: std::option::Option<tokio::time::Instant>,
}

impl BetweenCalls {
    /// Create a new `BetweenCalls` limiter.
    ///
    /// # Arguments
    /// * `ms` - Milliseconds to wait between calls.
    pub(crate) fn new(ms: std::primitive::u16) -> Self {
        Self {
            duration: tokio::time::Duration::from_millis(ms.into()),
            last_call_time: None,
        }
    }
}

impl super::Limiter for BetweenCalls {
    /// Limiting logic.
    async fn limit(&mut self) -> () {
        if let Some(last_call_time) = self.last_call_time {
            if tokio::time::Instant::now().duration_since(last_call_time) < self.duration {
                tokio::time::sleep(
                    self.duration - tokio::time::Instant::now().duration_since(last_call_time),
                )
                .await;
            }
        }
        self.last_call_time = Some(tokio::time::Instant::now());
    }
}
