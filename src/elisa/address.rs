#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Address {
    pub(super) address_id: std::primitive::u64,
}
