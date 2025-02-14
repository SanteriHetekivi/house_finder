#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AnnouncementRaw {
    pub(super) friendly_id: std::string::String,
    pub(super) address_line1: std::string::String,
    pub(super) latitude: std::option::Option<std::primitive::f64>,
    pub(super) longitude: std::option::Option<std::primitive::f64>,
    pub(super) construction_finished_year: std::option::Option<std::primitive::u16>,
    pub(super) search_price: std::option::Option<std::primitive::u32>,
    pub(super) area: std::option::Option<std::primitive::f64>,
    pub(super) total_area: std::option::Option<std::primitive::f64>,
}
