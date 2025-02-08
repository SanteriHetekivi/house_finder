// Structure for response.
#[derive(serde::Deserialize, Debug)]
pub(super) struct Response {
    pub(super) announcements: std::vec::Vec<super::Announcement>,
}
