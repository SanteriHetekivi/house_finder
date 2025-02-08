#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Route {
    pub(super) summary: super::Summary,
}
