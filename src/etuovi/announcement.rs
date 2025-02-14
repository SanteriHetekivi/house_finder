#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Announcement {
    pub(self) raw: super::AnnouncementRaw,
    pub(self) cache_html: std::primitive::bool,
    pub(self) html: std::option::Option<std::string::String>,
}

impl Announcement {
    /// Create new announcement.
    ///
    /// # Arguments
    /// * `raw` - Raw announcement.
    /// * `cache_html` - Cache HTML request result?
    pub(super) fn new(raw: super::AnnouncementRaw, cache_html: bool) -> Self {
        Self {
            raw,
            cache_html,
            html: None,
        }
    }

    /// Get HTML for this announcement.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    pub(self) async fn html(
        &mut self,
    ) -> std::result::Result<std::string::String, crate::client::RequestError> {
        if let Some(html) = &self.html {
            return Ok(html.clone());
        }
        let html = crate::client::Client::new(
            if self.cache_html {
                Some("etuovi/kohde")
            } else {
                None
            },
            Some(std::sync::Arc::clone(&super::LIMITER)),
        )?
        .get_text(&crate::app::Announcement::url(self))
        .await?;
        self.html = Some(html.clone());
        return Ok(html);
    }
}

impl crate::app::Announcement for Announcement {
    /// URL for the announcement.
    fn url(&self) -> std::string::String {
        format!("https://www.etuovi.com/kohde/{}", self.raw.friendly_id)
    }

    /// Location for the house.
    fn location(&self) -> std::option::Option<longitude::Location> {
        if let Some(latitude) = self.raw.latitude {
            if let Some(longitude) = self.raw.longitude {
                return Some(longitude::Location::from(latitude, longitude));
            }
        }
        return None;
    }

    /// Square meters for the house.
    fn square_meters_house(&self) -> std::option::Option<std::primitive::u16> {
        match self.raw.area {
            Some(area) => Some(area.floor() as std::primitive::u16),
            None => None,
        }
    }

    /// Total square meters for the whole property.
    fn square_meters_total(&self) -> std::option::Option<std::primitive::u16> {
        match self.raw.total_area {
            Some(total_area) => Some(total_area.floor() as std::primitive::u16),
            None => None,
        }
    }

    /// Price in euros.
    fn euros(&self) -> std::option::Option<std::primitive::u32> {
        self.raw.search_price
    }

    /// Street address.
    fn street_address(&self) -> std::string::String {
        self.raw.address_line1.clone()
    }

    /// Postal code.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    async fn postal_code(&mut self) -> std::result::Result<std::string::String, super::RegexError> {
        Ok(regex::Regex::new(r#""postCode":"([0-9]{5})""#)?
            .captures(&self.html().await?)
            .ok_or(regex::Error::Syntax(
                "No postal code capture group".to_string(),
            ))?
            .get(1)
            .ok_or(regex::Error::Syntax(
                "No capture group 1 for postal code".to_string(),
            ))?
            .as_str()
            .to_string())
    }

    /// Construction year.
    fn year(&self) -> std::option::Option<std::primitive::u16> {
        self.raw.construction_finished_year
    }

    /// Number of floors.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    async fn floors(
        &mut self,
    ) -> std::result::Result<std::option::Option<std::primitive::u8>, super::RegexError> {
        let html: std::string::String = self.html().await?;
        let capture: std::option::Option<regex::Captures<'_>> =
            regex::Regex::new(r#""floorCount":([0-9]),"#)?.captures(&html);
        match capture {
            None => Ok(None),
            Some(captures) => Ok(Some(
                captures
                    .get(1)
                    .ok_or(regex::Error::Syntax(
                        "No capture group 1 for floorCount".to_string(),
                    ))?
                    .as_str()
                    .parse::<std::primitive::u8>()?,
            )),
        }
    }

    /// Text for the announcement.
    ///
    /// # Arguments
    /// * `cache` - Use cache for HTTP request?
    async fn text(&mut self) -> std::result::Result<std::string::String, super::RegexError> {
        Ok(regex::Regex::new(r#""text":"(.*?)","#)?
            .captures(&self.html().await?)
            .ok_or(regex::Error::Syntax("No text capture group".to_string()))?
            .get(1)
            .ok_or(regex::Error::Syntax(
                "No capture group 1 for text".to_string(),
            ))?
            .as_str()
            .to_string())
    }
}
