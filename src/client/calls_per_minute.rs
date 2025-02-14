/// A limiter that limits the number of calls per minute.
pub(crate) struct CallsPerMinute {
    pub(self) max: std::primitive::u8,
    pub(self) count: std::primitive::u8,
    pub(self) zeroed: std::option::Option<tokio::time::Instant>,
    pub(self) last_call: std::option::Option<tokio::time::Instant>,
}

impl CallsPerMinute {
    /// Create a new `CallsPerMinute` limiter.
    ///
    /// # Arguments
    /// * `max` - Maximum number of calls per minute.
    pub(crate) fn new(max: std::primitive::u8) -> Self {
        Self {
            max,
            count: 0,
            zeroed: None,
            last_call: None,
        }
    }
}

const MINUTE: tokio::time::Duration = tokio::time::Duration::from_secs(60);

impl super::Limiter for CallsPerMinute {
    /// Limiting logic.
    async fn limit(&mut self) -> () {
        if let Some(zeroed) = self.zeroed {
            if MINUTE <= tokio::time::Instant::now().duration_since(zeroed) {
                self.count = 0;
                self.zeroed = Some(tokio::time::Instant::now());
            } else if self.max <= self.count {
                // Needs to be minute from last call, not exactly X calls in a minute.
                // At least for the OpenRouteService API.
                tokio::time::sleep(
                    MINUTE
                        - tokio::time::Instant::now()
                            .duration_since(self.last_call.unwrap_or(tokio::time::Instant::now())),
                )
                .await;
                self.count = 0;
                self.zeroed = Some(tokio::time::Instant::now());
            }
        } else {
            self.zeroed = Some(tokio::time::Instant::now());
            self.count = 0;
        }
        self.count += 1;
        self.last_call = Some(tokio::time::Instant::now());
    }
}
