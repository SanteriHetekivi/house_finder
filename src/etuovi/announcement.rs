#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Announcement {
    pub(self) friendly_id: std::string::String,
    pub(self) address_line1: std::string::String,
    pub(self) latitude: std::option::Option<std::primitive::f64>,
    pub(self) longitude: std::option::Option<std::primitive::f64>,
    pub(self) construction_finished_year: std::option::Option<std::primitive::u16>,
    pub(self) search_price: std::primitive::u32,
    pub(self) area: std::primitive::f64,
    pub(self) total_area: std::option::Option<std::primitive::f64>,
}

impl Announcement {
    /// URL for the announcement.
    pub(crate) fn url(&self) -> std::string::String {
        format!("https://www.etuovi.com/kohde/{}", self.friendly_id)
    }

    /// Location for the house.
    pub(crate) fn location(&self) -> std::option::Option<longitude::Location> {
        if let Some(latitude) = self.latitude {
            if let Some(longitude) = self.longitude {
                return Some(longitude::Location::from(latitude, longitude));
            }
        }
        return None;
    }

    /// Square meters for the house.
    pub(crate) fn square_meters_house(&self) -> std::primitive::f64 {
        self.area
    }

    /// Total square meters for the whole property.
    pub(crate) fn square_meters_total(&self) -> std::option::Option<std::primitive::f64> {
        self.total_area
    }

    /// Price in euros.
    pub(crate) fn euros(&self) -> std::primitive::u32 {
        self.search_price
    }

    /// Street address.
    pub(crate) fn street_address(&self) -> std::string::String {
        self.address_line1.clone()
    }

    /// Postal code.
    ///
    /// # Arguments
    /// * `cache` - Use cache?
    pub(crate) async fn postal_code(
        &self,
        cache: std::primitive::bool,
    ) -> std::result::Result<std::string::String, crate::client::RegexError> {
        crate::client::Client::new(if cache { Some("etuovi/kohde") } else { None }, 1000, None)?
            .get_regex(&self.url(), r#""postCode":"([0-9]{5})""#)
            .await
    }

    /// Construction year.
    pub(crate) fn year(&self) -> std::option::Option<std::primitive::u16> {
        self.construction_finished_year
    }
}
