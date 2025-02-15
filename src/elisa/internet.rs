/// Elisa internet implementation.
#[derive(Clone)]
pub(crate) struct Internet {
    pub(self) product: super::Product,
}

impl self::Internet {
    /// Create a new internet instance.
    ///
    /// # Arguments
    /// * `product` - Elisa product.
    pub(super) fn new(product: super::Product) -> Self {
        Self { product }
    }

    /// Include the internet product?
    pub(super) fn include(&self) -> std::primitive::bool {
        // Do not include mobile products.
        self.product.r#type != "fixedWirelessBroadband"
    }
}

impl crate::app::Internet for self::Internet {
    /// Name for the internet product.
    fn name(&self) -> std::string::String {
        format!("{} ({})", self.product.name, self.product.r#type)
    }

    /// Price in euros per month.
    fn euros_per_month(&self) -> std::primitive::f32 {
        self.product.price
    }

    /// Data speed in megabits per second.
    fn mbps(&self) -> std::primitive::u32 {
        self.product.data_speed_in_kbps / 1000
    }

    /// When the internet is delivered.
    fn delivery_date(&self) -> std::string::String {
        self.product.delivery_date.clone()
    }
}
