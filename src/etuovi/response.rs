// Structure for response.
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Response {
    pub(super) announcements: std::vec::Vec<super::AnnouncementRaw>,
    pub(super) count_of_all_results: std::primitive::u16,
}
