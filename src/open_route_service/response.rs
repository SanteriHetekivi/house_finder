#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(super) struct Response {
    pub(super) routes: std::vec::Vec<super::Route>,
}
