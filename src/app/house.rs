/// House to buy.
pub(crate) struct House {
    pub(self) url: std::string::String,
    pub(self) location_house: std::option::Option<longitude::Location>,
    pub(self) square_meters_house: std::option::Option<std::primitive::u16>,
    pub(self) square_meters_total: std::option::Option<std::primitive::u16>,
    pub(self) euros: std::option::Option<std::primitive::u32>,
    pub(self) street_address: std::string::String,
    pub(self) year: std::option::Option<std::primitive::u16>,
    pub(self) location_comparison: std::option::Option<longitude::Location>,
    pub(self) open_route_service_token: std::option::Option<std::string::String>,
    pub(self) cache_elisa_fixed_broadband_products: std::primitive::bool,
    pub(self) biking_km_to_location: std::option::Option<std::primitive::u16>,
    pub(self) house_min_square_meters: std::option::Option<std::primitive::u16>,
    pub(super) max_distance_km: std::option::Option<std::primitive::u16>,
}

impl House {
    /// Initilizes house.
    ///
    /// # Arguments
    /// * `url` - URL for the house.
    /// * `location` - Optional location for the house.
    /// * `square_meters_house` - Optional square meters for the house.
    /// * `square_meters_total` - Optional total square meters for the whole property.
    /// * `euros` - Price in euros.
    /// * `street_address` - Street address.
    /// * `year` - Optional construction year.
    /// * `location_comparison` - Location for the location.
    /// * `open_route_service_token` - Open Route Service key.
    /// * `cache_elisa_fixed_broadband_products` - Use cache when getting Elisa fixed broadband products?
    /// * `house_min_square_meters` - Optional minimum area in square meters of the house.
    /// * `max_distance_km` - Optional maximum distance in kilometers to the location.
    pub(super) fn new(
        url: &std::primitive::str,
        location_house: std::option::Option<longitude::Location>,
        square_meters_house: std::option::Option<std::primitive::u16>,
        square_meters_total: std::option::Option<std::primitive::u16>,
        euros: std::option::Option<std::primitive::u32>,
        street_address: &std::primitive::str,
        year: std::option::Option<std::primitive::u16>,
        location_comparison: std::option::Option<longitude::Location>,
        open_route_service_token: std::option::Option<std::string::String>,
        cache_elisa_fixed_broadband_products: std::primitive::bool,
        house_min_square_meters: std::option::Option<std::primitive::u16>,
        max_distance_km: std::option::Option<std::primitive::u16>,
    ) -> Self {
        Self {
            url: url.to_string(),
            location_house,
            square_meters_house,
            square_meters_total,
            euros,
            street_address: street_address.to_string(),
            year,
            location_comparison,
            open_route_service_token,
            cache_elisa_fixed_broadband_products,
            biking_km_to_location: None,
            house_min_square_meters,
            max_distance_km,
        }
    }

    /// Distance to location directly.
    pub(self) fn distance_to_location(&self) -> std::option::Option<longitude::Distance> {
        if let Some(location_comparison) = &self.location_comparison {
            if let Some(location_house) = &self.location_house {
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
            &self.street_address,
            self.cache_elisa_fixed_broadband_products,
        )
        .await?
        .products()
        {
            let mbps: std::primitive::u32 = elisa_product.mbps();
            if mbps == 0 || 100 < mbps {
                internets.push(super::Internet {
                    name: elisa_product.name(),
                    euros_per_month: elisa_product.euros_per_month(),
                    mbps: elisa_product.mbps(),
                });
            }
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
                    if let Some(location) = &self.location_house {
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
    pub(super) async fn include(
        &mut self,
    ) -> std::result::Result<std::primitive::bool, crate::open_route_service::Error> {
        // Check area.
        if let Some(house_min_square_meters) = self.house_min_square_meters {
            if let Some(square_meters_house) = self.square_meters_house {
                if square_meters_house < house_min_square_meters {
                    return Ok(false);
                }
            } else if let Some(square_meters_total) = self.square_meters_total {
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

        return Ok(true);
    }

    /// Result for the house.
    ///
    /// # Arguments
    /// * `postal_code` - Postal code for the house.
    pub(super) async fn result(
        &mut self,
        postal_code: &std::primitive::str,
    ) -> std::result::Result<super::Result, crate::open_route_service::Error> {
        Ok(super::Result::new(
            self.url.clone(),
            match self.euros {
                Some(euros) => Some(euros / 1000),
                None => None,
            },
            self.square_meters_house,
            match self.euros {
                Some(euros) => match self.square_meters_house {
                    Some(square_meters_house) => {
                        Some(euros / square_meters_house as std::primitive::u32)
                    }
                    None => None,
                },
                None => None,
            },
            self.square_meters_total,
            match self.euros {
                Some(euros) => match self.square_meters_total {
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
            self.year,
            self.internets(postal_code).await?,
        ))
    }
}
