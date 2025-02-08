/// Etuovi.com API.
pub(crate) struct Etuovi {
    pub(self) cache: std::primitive::bool,
    pub(self) publishing_time_search_criteria: std::string::String,
    pub(self) price_max: std::primitive::u32,
    pub(self) cities: std::vec::Vec<std::string::String>,
}

impl Etuovi {
    /// Create new Etuovi API instance.
    ///
    /// # Arguments
    /// * `cache` - Use cache?
    /// * `publishing_time_search_criteria` - Search criteria for publishing time. One of: ANY_DAY, WITHIN_ONE_DAY, WITHIN_TWO_DAYS, WITHIN_SEVEN_DAYS, WITHIN_TWO_WEEKS.
    /// * `price_max` - Max price in euros.
    /// * `cities` - Cities.
    pub(crate) fn new(
        cache: std::primitive::bool,
        publishing_time_search_criteria: &str,
        price_max: std::primitive::u32,
        cities: std::vec::Vec<std::string::String>,
    ) -> Self {
        Self {
            cache,
            publishing_time_search_criteria: publishing_time_search_criteria.to_string(),
            price_max,
            cities,
        }
    }

    /// Get announcements.
    pub(crate) async fn announcements(
        &self,
    ) -> std::result::Result<std::vec::Vec<super::Announcement>, crate::client::JSONError> {
        let classified_location_terms: Vec<_> = self
            .cities
            .iter()
            .map(|city| {
                serde_json::json!({
                    "type": "CITY",
                    "code": city
                })
            })
            .collect();
        Ok(crate::client::Client::new(if self.cache {
            Some("etuovi/announcements/search/listpage")
        } else {
            None
        })?
        .post_json::<super::Response>(
            "https://www.etuovi.com/api/v2/announcements/search/listpage",
            serde_json::json!({
                "propertyType": "RESIDENTIAL",
                "priceMax": self.price_max,
                "publishingTimeSearchCriteria": self.publishing_time_search_criteria,
                "ownershipTypes": ["OWN"],
                "plotHoldingTypes": ["OWN"],
                "residentialPropertyTypes": ["DETACHED_HOUSE"],
                "locationSearchCriteria": {
                    "classifiedLocationTerms": classified_location_terms
                }
            }),
            None,
        )
        .await?
        .announcements)
    }
}
