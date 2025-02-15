/// Elisa internet product.
#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Product {
    pub(super) name: std::string::String,
    pub(super) r#type: std::string::String,
    pub(super) price: std::primitive::f32,
    pub(super) data_speed_in_kbps: std::primitive::u32,
    pub(super) delivery_date: std::string::String,
}
