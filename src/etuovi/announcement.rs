#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Announcement {
    pub(self) friendly_id: std::string::String,
    pub(self) address_line1: std::string::String,
    pub(self) latitude: std::option::Option<std::primitive::f64>,
    pub(self) longitude: std::option::Option<std::primitive::f64>,
    pub(self) construction_finished_year: std::option::Option<std::primitive::u16>,
    pub(self) search_price: std::option::Option<std::primitive::u32>,
    pub(self) area: std::option::Option<std::primitive::f64>,
    pub(self) total_area: std::option::Option<std::primitive::f64>,
    pub(self) html: std::option::Option<std::string::String>,
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
    pub(crate) fn square_meters_house(&self) -> std::option::Option<std::primitive::u16> {
        match self.area {
            Some(area) => Some(area.floor() as std::primitive::u16),
            None => None,
        }
    }

    /// Total square meters for the whole property.
    pub(crate) fn square_meters_total(&self) -> std::option::Option<std::primitive::u16> {
        match self.total_area {
            Some(total_area) => Some(total_area.floor() as std::primitive::u16),
            None => None,
        }
    }

    /// Price in euros.
    pub(crate) fn euros(&self) -> std::option::Option<std::primitive::u32> {
        self.search_price
    }

    /// Street address.
    pub(crate) fn street_address(&self) -> std::string::String {
        self.address_line1.clone()
    }

    /// Postal code.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    pub(crate) async fn postal_code(
        &mut self,
        cache: std::primitive::bool,
    ) -> std::result::Result<std::string::String, super::RegexError> {
        Ok(regex::Regex::new(r#""postCode":"([0-9]{5})""#)?
            .captures(&self.html(cache).await?)
            .ok_or(regex::Error::Syntax(
                "No postal code capture group".to_string(),
            ))?
            .get(1)
            .ok_or(regex::Error::Syntax("No capture group 1".to_string()))?
            .as_str()
            .to_string())
    }

    /// Construction year.
    pub(crate) fn year(&self) -> std::option::Option<std::primitive::u16> {
        self.construction_finished_year
    }

    /// Get HTML for this announcement.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    pub(self) async fn html(
        &mut self,
        cache: std::primitive::bool,
    ) -> std::result::Result<std::string::String, crate::client::RequestError> {
        if let Some(html) = &self.html {
            return Ok(html.clone());
        }
        let html = crate::client::Client::new(
            if cache { Some("etuovi/kohde") } else { None },
            Some(std::sync::Arc::clone(&super::LIMITER)),
        )?
        .get_text(&self.url())
        .await?;
        self.html = Some(html.clone());
        return Ok(html);
    }

    /// Number of floors.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    pub(crate) async fn floors(
        &mut self,
        cache: std::primitive::bool,
    ) -> std::result::Result<std::option::Option<std::primitive::u8>, super::RegexError> {
        let html: std::string::String = self.html(cache).await?;
        let capture: std::option::Option<regex::Captures<'_>> =
            regex::Regex::new(r#""floorCount":([0-9]),"#)?.captures(&html);
        match capture {
            None => Ok(None),
            Some(captures) => Ok(Some(
                captures
                    .get(1)
                    .ok_or(regex::Error::Syntax("No capture group 1".to_string()))?
                    .as_str()
                    .parse::<std::primitive::u8>()?,
            )),
        }
    }
}
