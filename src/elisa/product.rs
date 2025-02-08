/// Elisa internet product.
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Product {
    pub(self) name: std::string::String,
    pub(self) r#type: std::string::String,
    pub(self) price: std::primitive::f32,
    pub(self) data_speed_in_kbps: std::primitive::u32,
}

impl Product {
    /// Include the internet product?
    pub(super) fn include(&self) -> std::primitive::bool {
        // Do not include mobile products.
        self.r#type != "fixedWirelessBroadband"
    }

    /// Name for the internet product.
    pub(crate) fn name(&self) -> std::string::String {
        format!("{} ({})", self.name, self.r#type)
    }

    /// Price in euros per month.
    pub(crate) fn euros_per_month(&self) -> std::primitive::f32 {
        self.price
    }

    /// Data speed in megabits per second.
    pub(crate) fn mbps(&self) -> std::primitive::u32 {
        self.data_speed_in_kbps / 1000
    }
}
