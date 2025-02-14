/// Limiter trait.
pub(crate) trait Limiter {
    async fn limit(&mut self) -> ();
}
