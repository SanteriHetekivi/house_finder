#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Response {
    pub(super) fbb_products: std::vec::Vec<super::Product>,
}
