pub(super) static LIMITER: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<crate::client::BetweenCalls>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::Mutex::new(crate::client::BetweenCalls::new(
        5000,
    )))
});
