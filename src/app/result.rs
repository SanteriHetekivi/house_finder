/// Store formatted result.
pub(super) struct Result {
    pub(self) url: std::string::String,
    pub(self) thousands_of_euros: std::option::Option<std::primitive::u32>,
    pub(self) square_meters_house: std::option::Option<std::primitive::f64>,
    pub(self) euros_per_square_meter_house: std::option::Option<std::primitive::f64>,
    pub(self) square_meters_total: std::option::Option<std::primitive::f64>,
    pub(self) euros_per_square_meter_total: std::option::Option<std::primitive::f64>,
    pub(self) km_to_location_straight: std::option::Option<std::primitive::f64>,
    pub(self) km_to_location_biking: std::option::Option<std::primitive::f64>,
    pub(self) year: std::option::Option<std::primitive::u16>,
    pub(self) internets: std::vec::Vec<super::Internet>,
}

impl Result {
    /// Create a new result.
    ///
    /// # Arguments
    /// * `url` - URL.
    /// * `thousands_of_euros` - Optional price in thousands of euros.
    /// * `square_meters_house` - Optional square meters for the house.
    /// * `euros_per_square_meter_house` - Optional price per square meter for the house.
    /// * `square_meters_total` - Optional total square meters.
    /// * `euros_per_square_meter_total` - Optional price per square meter for the total.
    /// * `km_to_location_straight` - Optional distance to location straight.
    /// * `km_to_location_biking` - Optional distance to location biking.
    /// * `year` - Optional construction year.
    /// * `internets` - Internet products.
    pub(super) fn new(
        url: std::string::String,
        thousands_of_euros: std::option::Option<std::primitive::u32>,
        square_meters_house: std::option::Option<std::primitive::f64>,
        euros_per_square_meter_house: std::option::Option<std::primitive::f64>,
        square_meters_total: std::option::Option<std::primitive::f64>,
        euros_per_square_meter_total: std::option::Option<std::primitive::f64>,
        km_to_location_straight: std::option::Option<std::primitive::f64>,
        km_to_location_biking: std::option::Option<std::primitive::f64>,
        year: std::option::Option<std::primitive::u16>,
        internets: std::vec::Vec<super::Internet>,
    ) -> Self {
        Self {
            url,
            thousands_of_euros,
            square_meters_house,
            euros_per_square_meter_house,
            square_meters_total,
            euros_per_square_meter_total,
            km_to_location_straight,
            km_to_location_biking,
            year,
            internets,
        }
    }

    /// Generate message.
    pub(super) fn message(&self) -> std::string::String {
        let mut message: std::string::String = std::string::String::new();
        message.push_str(&self.url);
        message.push_str(":");

        if let Some(thousands_of_euros) = self.thousands_of_euros {
            message.push_str("\n\tPrice: ");
            message.push_str(&thousands_of_euros.to_string());
            message.push_str(" k€");
        }

        if let Some(square_meters_house) = self.square_meters_house {
            message.push_str("\n\tArea (house): ");
            message.push_str(&square_meters_house.floor().to_string());
            message.push_str(" m²");
        }

        if let Some(euros_per_square_meter_house) = self.euros_per_square_meter_house {
            message.push_str("\n\tPrice/Area (house): ");
            message.push_str(&(euros_per_square_meter_house.ceil()).to_string());
            message.push_str(" €/m²");
        }

        if let Some(square_meters_total) = self.square_meters_total {
            message.push_str("\n\tArea (total): ");
            message.push_str(&square_meters_total.floor().to_string());
            message.push_str(" m²");
        }

        if let Some(euros_per_square_meter_total) = self.euros_per_square_meter_total {
            message.push_str("\n\tPrice/Area (total): ");
            message.push_str(&(euros_per_square_meter_total.ceil()).to_string());
            message.push_str(" €/m²");
        }

        if let Some(km_to_location_straight) = self.km_to_location_straight {
            message.push_str("\n\tStraight to location: ");
            message.push_str(&km_to_location_straight.ceil().to_string());
            message.push_str(" km");
        }

        if let Some(km_to_location_biking) = self.km_to_location_biking {
            message.push_str("\n\tBiking to location: ");
            message.push_str(&km_to_location_biking.ceil().to_string());
            message.push_str(" km");
        }

        if let Some(year) = self.year {
            message.push_str("\n\tYear: ");
            message.push_str(&year.to_string());
        }

        if !self.internets.is_empty() {
            message.push_str("\n\tInternet:");
            for internet in self.internets.iter() {
                message.push_str("\n\t- ");
                message.push_str(&internet.to_str());
            }
        }

        return message;
    }
}
