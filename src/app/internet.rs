/// Internet product.
pub(super) struct Internet {
    pub(super) name: std::string::String,
    pub(super) euros_per_month: std::primitive::f32,
    pub(super) mbps: std::primitive::u32,
}

impl Internet {
    /// Transform to string.
    pub(super) fn to_str(&self) -> std::string::String {
        format!(
            "{}: {:.2} €/kk {:.0} Mbit/s",
            self.name, self.euros_per_month, self.mbps
        )
    }
}
