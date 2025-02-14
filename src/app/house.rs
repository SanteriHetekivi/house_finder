/// House to buy.
pub(crate) struct House<A: super::Announcement> {
    pub(self) announcement: A,
    pub(self) location_comparison: std::option::Option<longitude::Location>,
    pub(self) open_route_service_token: std::option::Option<std::string::String>,
    pub(self) cache_elisa_fixed_broadband_products: std::primitive::bool,
    pub(self) biking_km_to_location: std::option::Option<std::primitive::u16>,
    pub(self) house_min_square_meters: std::option::Option<std::primitive::u16>,
    pub(self) max_distance_km: std::option::Option<std::primitive::u16>,
    pub(self) min_mbps: std::option::Option<std::primitive::u32>,
    pub(self) exclude_texts: std::vec::Vec<std::string::String>,
}

impl<A: super::Announcement> House<A> {
    /// Initilizes house.
    ///
    /// # Arguments
    /// * `announcement` - Announcement for the house.
    /// * `location_comparison` - Location for the location.
    /// * `open_route_service_token` - Open Route Service key.
    /// * `cache_elisa_fixed_broadband_products` - Use cache when getting Elisa fixed broadband products?
    /// * `house_min_square_meters` - Optional minimum area in square meters of the house.
    /// * `max_distance_km` - Optional maximum distance in kilometers to the location.
    /// * `min_mbps` - Optional minimum megabits per second for the internet.
    /// * `exclude_texts` - Exclude house if it's text data has one of these texts.
    pub(super) fn new(
        announcement: A,
        location_comparison: std::option::Option<longitude::Location>,
        open_route_service_token: std::option::Option<std::string::String>,
        cache_elisa_fixed_broadband_products: std::primitive::bool,
        house_min_square_meters: std::option::Option<std::primitive::u16>,
        max_distance_km: std::option::Option<std::primitive::u16>,
        min_mbps: std::option::Option<std::primitive::u32>,
        exclude_texts: std::vec::Vec<std::string::String>,
    ) -> Self {
        Self {
            announcement,
            location_comparison,
            open_route_service_token,
            cache_elisa_fixed_broadband_products,
            biking_km_to_location: None,
            house_min_square_meters,
            max_distance_km,
            min_mbps,
            exclude_texts,
        }
    }

    /// Distance to location directly.
    pub(self) fn distance_to_location(&self) -> std::option::Option<longitude::Distance> {
        if let Some(location_comparison) = &self.location_comparison {
            if let Some(location_house) = &self.announcement.location() {
                return Some(location_house.distance(location_comparison));
            }
        }

        return None;
    }

    /// Internet products that meet my requirements.
    ///
    /// # Arguments
    /// * `postal_code` - Postal code for the house.
    pub(self) async fn internets(
        &self,
        postal_code: &std::primitive::str,
    ) -> std::result::Result<std::vec::Vec<super::Internet>, crate::client::JSONError> {
        let mut internets: std::vec::Vec<super::Internet> = std::vec::Vec::<super::Internet>::new();
        for elisa_product in crate::elisa::Elisa::new(
            postal_code,
            &self.announcement.street_address(),
            self.cache_elisa_fixed_broadband_products,
        )
        .await?
        .products()
        {
            let mbps: std::primitive::u32 = elisa_product.mbps();
            if let Some(min_mbps) = self.min_mbps {
                if mbps != 0 && mbps < min_mbps {
                    continue;
                }
            }
            internets.push(super::Internet {
                name: elisa_product.name(),
                euros_per_month: elisa_product.euros_per_month(),
                mbps,
            });
        }
        return Ok(internets);
    }

    /// Biking distance in kilometers to location.
    pub(self) async fn biking_km_to_location(
        &mut self,
    ) -> std::result::Result<
        std::option::Option<std::primitive::u16>,
        crate::open_route_service::Error,
    > {
        if self.biking_km_to_location.is_none() {
            if let Some(location_comparison) = &self.location_comparison {
                if let Some(open_route_service_token) = &self.open_route_service_token {
                    if let Some(location) = &self.announcement.location() {
                        self.biking_km_to_location = Some(
                            crate::open_route_service::OpenRouteService::new(
                                open_route_service_token,
                            )?
                            .biking_km(location.clone(), location_comparison.clone())
                            .await?,
                        );
                    }
                }
            }
        }
        return Ok(self.biking_km_to_location);
    }

    /// Include house as one of the options?
    pub(self) async fn include(
        &mut self,
    ) -> std::result::Result<std::primitive::bool, super::Error> {
        // Check area.
        if let Some(house_min_square_meters) = self.house_min_square_meters {
            if let Some(square_meters_house) = self.announcement.square_meters_house() {
                if square_meters_house < house_min_square_meters {
                    return Ok(false);
                }
            } else if let Some(square_meters_total) = self.announcement.square_meters_total() {
                if square_meters_total < house_min_square_meters {
                    return Ok(false);
                }
            }
        }

        // Check distance.
        if let Some(max_distance_km) = self.max_distance_km {
            if let Some(distance_to_location) = self.distance_to_location() {
                if max_distance_km < distance_to_location.kilometers().ceil() as std::primitive::u16
                {
                    return Ok(false);
                }
            }
            if let Some(biking_km_to_location) = self.biking_km_to_location().await? {
                if max_distance_km < biking_km_to_location {
                    return Ok(false);
                }
            }
        }

        // Check texts.
        if !self.exclude_texts.is_empty() {
            let text_lowercase: std::string::String =
                self.announcement.text().await?.to_lowercase();
            for invalid_text in &self.exclude_texts {
                if text_lowercase.contains(invalid_text) {
                    return Ok(false);
                }
            }
        }

        return Ok(true);
    }

    /// Result for the house.
    ///
    /// # Arguments
    /// * `postal_code` - Postal code for the house.
    pub(super) async fn result(
        &mut self,
    ) -> std::result::Result<std::option::Option<super::Result>, super::Error> {
        if !self.include().await? {
            return Ok(None);
        }
        let euros: std::option::Option<u32> = self.announcement.euros();
        let square_meters_house: std::option::Option<u16> = self.announcement.square_meters_house();
        let square_meters_total: std::option::Option<u16> = self.announcement.square_meters_total();
        let postal_code: std::string::String = self.announcement.postal_code().await?;
        Ok(Some(super::Result::new(
            self.announcement.url().clone(),
            match euros {
                Some(euros) => Some(euros / 1000),
                None => None,
            },
            self.announcement.floors().await?,
            square_meters_house,
            match euros {
                Some(euros) => match square_meters_house {
                    Some(square_meters_house) => {
                        Some(euros / square_meters_house as std::primitive::u32)
                    }
                    None => None,
                },
                None => None,
            },
            square_meters_total,
            match euros {
                Some(euros) => match square_meters_total {
                    Some(square_meters_total) => {
                        Some(euros / square_meters_total as std::primitive::u32)
                    }
                    None => None,
                },
                None => None,
            },
            match self.distance_to_location() {
                Some(distance_to_location) => {
                    Some(distance_to_location.kilometers().ceil() as std::primitive::u16)
                }
                None => None,
            },
            self.biking_km_to_location().await?,
            self.announcement.year(),
            self.internets(&postal_code).await?,
        )))
    }
}
