/// Internet trait.
pub(crate) trait Internet {
    /// Name of the internet product.
    fn name(&self) -> std::string::String;

    /// Price in euros per month.
    fn euros_per_month(&self) -> std::primitive::f32;

    /// Speed in megabits per second.
    fn mbps(&self) -> std::primitive::u32;

    /// When the internet is delivered.
    fn delivery_date(&self) -> std::string::String;

    /// Check if mbps passes the minimum requirement.
    /// 0 mbps always passes.
    ///
    /// # Arguments
    /// * `min_mbps` - Optional minimum megabits per second.
    fn check_mbps(
        &self,
        min_mbps: std::option::Option<std::primitive::u32>,
    ) -> std::primitive::bool {
        return match min_mbps {
            Some(min_mbps) => {
                let mbps: std::primitive::u32 = self.mbps();
                return mbps == 0 || min_mbps <= mbps;
            }
            None => true,
        };
    }

    /// Transform to string.
    fn to_str(&self) -> std::string::String {
        format!(
            "{} ({}-): {:.2} €/kk, {:.0} Mbit/s",
            self.name(),
            self.delivery_date(),
            self.euros_per_month(),
            self.mbps(),
        )
    }
}
