/// House to buy.
pub(crate) struct House {
    pub(self) url: std::string::String,
    pub(self) location: std::option::Option<longitude::Location>,
    pub(self) square_meters_house: std::option::Option<std::primitive::f64>,
    pub(self) square_meters_total: std::option::Option<std::primitive::f64>,
    pub(self) euros: std::primitive::u32,
    pub(self) street_address: std::string::String,
    pub(self) year: std::option::Option<std::primitive::u16>,
    pub(self) cottage_location: longitude::Location,
    pub(self) open_route_service_token: std::option::Option<std::string::String>,
    pub(self) cache: std::primitive::bool,
    pub(self) biking_km_to_cottage: std::option::Option<std::primitive::f64>,
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
    /// * `cottage_location` - Location for the cottage.
    /// * `open_route_service_token` - Open Route Service key.
    /// * `cache` - Use cache?
    pub(super) fn new(
        url: &std::primitive::str,
        location: std::option::Option<longitude::Location>,
        square_meters_house: std::option::Option<std::primitive::f64>,
        square_meters_total: std::option::Option<std::primitive::f64>,
        euros: std::primitive::u32,
        street_address: &std::primitive::str,
        year: std::option::Option<std::primitive::u16>,
        cottage_location: longitude::Location,
        open_route_service_token: std::option::Option<std::string::String>,
        cache: std::primitive::bool,
    ) -> Self {
        Self {
            url: url.to_string(),
            location,
            square_meters_house,
            square_meters_total,
            euros,
            street_address: street_address.to_string(),
            year,
            cottage_location,
            open_route_service_token,
            cache,
            biking_km_to_cottage: None,
        }
    }

    /// Distance to cottage directly.
    pub(self) fn distance_to_cottage(&self) -> std::option::Option<longitude::Distance> {
        if let Some(location) = &self.location {
            return Some(location.distance(&self.cottage_location));
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
        for elisa_product in crate::elisa::Elisa::new(postal_code, &self.street_address, self.cache)
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

    /// Biking distance in kilometers to cottage.
    pub(self) async fn biking_km_to_cottage(
        &mut self,
    ) -> std::result::Result<
        std::option::Option<std::primitive::f64>,
        crate::open_route_service::Error,
    > {
        if self.biking_km_to_cottage.is_none() {
            if let Some(open_route_service_token) = &self.open_route_service_token {
                if let Some(location) = &self.location {
                    self.biking_km_to_cottage = Some(
                        crate::open_route_service::OpenRouteService::new(open_route_service_token)?
                            .biking_km(location.clone(), self.cottage_location.clone())
                            .await?,
                    );
                }
            }
        }
        return Ok(self.biking_km_to_cottage);
    }

    /// Include house as one of the options?
    pub(super) async fn include(
        &mut self,
    ) -> std::result::Result<std::primitive::bool, crate::open_route_service::Error> {
        // Check area.
        if let Some(square_meters_house) = self.square_meters_house {
            if square_meters_house < 40.0 {
                return Ok(false);
            }
        } else if let Some(square_meters_total) = self.square_meters_total {
            if square_meters_total < 40.0 {
                return Ok(false);
            }
        }

        // Check distance.
        if let Some(distance_to_cottage) = self.distance_to_cottage() {
            if 35.0 < distance_to_cottage.kilometers() {
                return Ok(false);
            }
        }
        if let Some(biking_km_to_cottage) = self.biking_km_to_cottage().await? {
            if 35.0 < biking_km_to_cottage {
                return Ok(false);
            }
        }

        return Ok(true);
    }

    /// Message about house.
    ///
    /// # Arguments
    /// * `postal_code` - Postal code for the house.
    pub(super) async fn message(
        &mut self,
        postal_code: &std::primitive::str,
    ) -> Result<std::string::String, crate::open_route_service::Error> {
        let mut message: std::string::String = std::string::String::new();
        message.push_str(&self.url);
        message.push_str(":");

        message.push_str("\n\tPrice: ");
        message.push_str(&(self.euros / 1000).to_string());
        message.push_str(" k€");

        let euros: std::primitive::f64 = self.euros as std::primitive::f64;

        if let Some(square_meters_house) = self.square_meters_house {
            message.push_str("\n\tArea (house): ");
            message.push_str(&square_meters_house.floor().to_string());
            message.push_str(" m²");

            message.push_str("\n\tPrice/Area (house): ");
            message.push_str(&((euros / square_meters_house).ceil()).to_string());
            message.push_str(" €/m²");
        }

        if let Some(square_meters_total) = self.square_meters_total {
            message.push_str("\n\tArea (total): ");
            message.push_str(&square_meters_total.floor().to_string());
            message.push_str(" m²");

            message.push_str("\n\tPrice/Area (total): ");
            message.push_str(&((euros / square_meters_total).ceil()).to_string());
            message.push_str(" €/m²");
        }

        if let Some(biking_km_to_cottage) = self.biking_km_to_cottage().await? {
            message.push_str("\n\tBiking to cottage: ");
            message.push_str(&biking_km_to_cottage.ceil().to_string());
            message.push_str(" km");
        }

        if let Some(year) = self.year {
            message.push_str("\n\tYear: ");
            message.push_str(&year.to_string());
        }

        let internets: std::vec::Vec<super::Internet> = self.internets(postal_code).await?;
        if !internets.is_empty() {
            message.push_str("\n\tInternet:");
            for internet in internets {
                message.push_str("\n\t- ");
                message.push_str(&internet.to_str());
            }
        }
        Ok(message)
    }
}
