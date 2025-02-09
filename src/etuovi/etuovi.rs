/// Etuovi.com API.
pub(crate) struct Etuovi {
    pub(self) client: crate::client::Client,
    pub(self) publishing_time_search_criteria: std::string::String,
    pub(self) price_max: std::option::Option<std::primitive::u32>,
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
        publishing_time_search_criteria: &std::primitive::str,
        price_max: std::option::Option<std::primitive::u32>,
        cities: std::vec::Vec<std::string::String>,
    ) -> std::result::Result<Self, crate::client::RequestError> {
        Ok(Self {
            client: crate::client::Client::new(
                if cache {
                    Some("etuovi/announcements/search/listpage")
                } else {
                    None
                },
                1000,
                None,
            )?,
            publishing_time_search_criteria: publishing_time_search_criteria.to_string(),
            price_max,
            cities,
        })
    }

    /// Get one page of announcements.
    ///
    /// # Arguments
    /// * `classified_location_terms` - Classified location terms.
    /// * `page` - Page number.
    pub(super) async fn announcements_page(
        &self,
        classified_location_terms: std::vec::Vec<serde_json::Value>,
        page: u16,
    ) -> std::result::Result<super::Response, crate::client::JSONError> {
        Ok(self
            .client
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
                    },
                    "pagination": {
                        "firstResult": 30*(page-1),
                        "maxResults": 30,
                        "page": page,
                    },
                }),
                None,
            )
            .await?)
    }

    /// Get announcements.
    pub(crate) async fn announcements(
        &self,
    ) -> std::result::Result<std::vec::Vec<super::Announcement>, crate::client::JSONError> {
        let mut announcements: std::vec::Vec<super::Announcement> = std::vec::Vec::new();
        let classified_location_terms: std::vec::Vec<serde_json::Value> = self
            .cities
            .iter()
            .map(|city| {
                serde_json::json!({
                    "type": "CITY",
                    "code": city
                })
            })
            .collect();
        let mut page: std::primitive::u16 = 1;

        // Loop every page.
        loop {
            let response: super::Response = self
                .announcements_page(classified_location_terms.clone(), page)
                .await?;
            let mut added: std::primitive::bool = false;
            for announcement in response.announcements {
                announcements.push(announcement);
                if !added {
                    added = true;
                }
            }
            if !added || response.count_of_all_results as usize <= announcements.len() {
                break;
            }
            page += 1;
        }

        return Ok(announcements);
    }
}
