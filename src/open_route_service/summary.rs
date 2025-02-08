#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Summary {
    pub(super) distance: std::primitive::f64,
}
