/// Store formatted result.
pub(super) struct Result {
    pub(self) url: std::string::String,
    pub(self) thousands_of_euros: std::option::Option<std::primitive::u32>,
    pub(self) square_meters_house: std::option::Option<std::primitive::u16>,
    pub(self) euros_per_square_meter_house: std::option::Option<std::primitive::u32>,
    pub(self) square_meters_total: std::option::Option<std::primitive::u16>,
    pub(self) euros_per_square_meter_total: std::option::Option<std::primitive::u32>,
    pub(self) km_to_location_straight: std::option::Option<std::primitive::u16>,
    pub(self) km_to_location_biking: std::option::Option<std::primitive::u16>,
    pub(self) year: std::option::Option<std::primitive::u16>,
    pub(self) internets: std::vec::Vec<super::Internet>,
}

/// Information about a field.
pub(self) struct FieldInfo {
    pub(super) title: &'static std::primitive::str,
    pub(super) unit: std::option::Option<&'static std::primitive::str>,
}

/// Map for field to it's information.
pub(self) struct FieldToInfo {
    pub(self) url: FieldInfo,
    pub(self) thousands_of_euros: FieldInfo,
    pub(self) square_meters_house: FieldInfo,
    pub(self) euros_per_square_meter_house: FieldInfo,
    pub(self) square_meters_total: FieldInfo,
    pub(self) euros_per_square_meter_total: FieldInfo,
    pub(self) km_to_location_straight: FieldInfo,
    pub(self) km_to_location_biking: FieldInfo,
    pub(self) year: FieldInfo,
    pub(self) internets: FieldInfo,
}

/// Field to information map.
const FIELD_TO_INFO: FieldToInfo = FieldToInfo {
    url: FieldInfo {
        title: "URL",
        unit: None,
    },
    thousands_of_euros: FieldInfo {
        title: "Price",
        unit: Some("k€"),
    },
    square_meters_house: FieldInfo {
        title: "Area (house)",
        unit: Some("m²"),
    },
    euros_per_square_meter_house: FieldInfo {
        title: "Price/Area (house)",
        unit: Some("€/m²"),
    },
    square_meters_total: FieldInfo {
        title: "Area (total)",
        unit: Some("m²"),
    },
    euros_per_square_meter_total: FieldInfo {
        title: "Price/Area (total)",
        unit: Some("€/m²"),
    },
    km_to_location_straight: FieldInfo {
        title: "Straight to location",
        unit: Some("km"),
    },
    km_to_location_biking: FieldInfo {
        title: "Biking to location",
        unit: Some("km"),
    },
    year: FieldInfo {
        title: "Year",
        unit: None,
    },
    internets: FieldInfo {
        title: "Internet",
        unit: None,
    },
};

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
        square_meters_house: std::option::Option<std::primitive::u16>,
        euros_per_square_meter_house: std::option::Option<std::primitive::u32>,
        square_meters_total: std::option::Option<std::primitive::u16>,
        euros_per_square_meter_total: std::option::Option<std::primitive::u32>,
        km_to_location_straight: std::option::Option<std::primitive::u16>,
        km_to_location_biking: std::option::Option<std::primitive::u16>,
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

    /// Generate message line.
    ///
    /// # Arguments
    /// * `info` - Field information.
    /// * `value` - Field value.
    pub(self) fn message_line(info: FieldInfo, value: std::string::String) -> std::string::String {
        format!(
            "\n\t{}: {}{}",
            info.title,
            value,
            match info.unit {
                Some(unit) => format!(" {}", unit),
                None => "".to_string(),
            }
        )
    }

    /// Generate message.
    pub(super) fn message(&self) -> std::string::String {
        let mut message: std::string::String = std::string::String::new();
        message.push_str(&self.url);
        message.push_str(":");

        if let Some(thousands_of_euros) = self.thousands_of_euros {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.thousands_of_euros,
                thousands_of_euros.to_string(),
            ));
        }

        if let Some(square_meters_house) = self.square_meters_house {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.square_meters_house,
                square_meters_house.to_string(),
            ));
        }

        if let Some(euros_per_square_meter_house) = self.euros_per_square_meter_house {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.euros_per_square_meter_house,
                euros_per_square_meter_house.to_string(),
            ));
        }

        if let Some(square_meters_total) = self.square_meters_total {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.square_meters_total,
                square_meters_total.to_string(),
            ));
        }

        if let Some(euros_per_square_meter_total) = self.euros_per_square_meter_total {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.euros_per_square_meter_total,
                euros_per_square_meter_total.to_string(),
            ));
        }

        if let Some(km_to_location_straight) = self.km_to_location_straight {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.km_to_location_straight,
                km_to_location_straight.to_string(),
            ));
        }

        if let Some(km_to_location_biking) = self.km_to_location_biking {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.km_to_location_biking,
                km_to_location_biking.to_string(),
            ));
        }

        if let Some(year) = self.year {
            message.push_str(&Self::message_line(FIELD_TO_INFO.year, year.to_string()));
        }

        if !self.internets.is_empty() {
            message.push_str(&Self::message_line(
                FIELD_TO_INFO.internets,
                self.internets
                    .iter()
                    .map(|internet| format!("\n\t- {}", internet.to_str()))
                    .collect::<std::string::String>(),
            ));
        }

        return message;
    }

    /// Generate CSV title row cell.
    ///
    /// # Arguments
    /// * `info` - Field information.
    pub(self) fn csv_title_row_cell(info: FieldInfo) -> std::string::String {
        format!(
            "{}{}",
            info.title,
            match info.unit {
                Some(unit) => format!(" {}", unit),
                None => "".to_string(),
            }
        )
    }

    /// Generate CSV title row.
    pub(super) fn csv_title_row() -> [std::string::String; 10] {
        [
            Self::csv_title_row_cell(FIELD_TO_INFO.url),
            Self::csv_title_row_cell(FIELD_TO_INFO.thousands_of_euros),
            Self::csv_title_row_cell(FIELD_TO_INFO.square_meters_house),
            Self::csv_title_row_cell(FIELD_TO_INFO.euros_per_square_meter_house),
            Self::csv_title_row_cell(FIELD_TO_INFO.square_meters_total),
            Self::csv_title_row_cell(FIELD_TO_INFO.euros_per_square_meter_total),
            Self::csv_title_row_cell(FIELD_TO_INFO.km_to_location_straight),
            Self::csv_title_row_cell(FIELD_TO_INFO.km_to_location_biking),
            Self::csv_title_row_cell(FIELD_TO_INFO.year),
            Self::csv_title_row_cell(FIELD_TO_INFO.internets),
        ]
    }

    /// Generate CSV row.
    pub(super) fn csv_row(&self) -> [std::string::String; 10] {
        [
            self.url.clone(),
            match self.thousands_of_euros {
                Some(thousands_of_euros) => thousands_of_euros.to_string(),
                None => "".to_string(),
            },
            match self.square_meters_house {
                Some(square_meters_house) => square_meters_house.to_string(),
                None => "".to_string(),
            },
            match self.euros_per_square_meter_house {
                Some(euros_per_square_meter_house) => euros_per_square_meter_house.to_string(),
                None => "".to_string(),
            },
            match self.square_meters_total {
                Some(square_meters_total) => square_meters_total.to_string(),
                None => "".to_string(),
            },
            match self.euros_per_square_meter_total {
                Some(euros_per_square_meter_total) => euros_per_square_meter_total.to_string(),
                None => "".to_string(),
            },
            match self.km_to_location_straight {
                Some(km_to_location_straight) => km_to_location_straight.to_string(),
                None => "".to_string(),
            },
            match self.km_to_location_biking {
                Some(km_to_location_biking) => km_to_location_biking.to_string(),
                None => "".to_string(),
            },
            match self.year {
                Some(year) => year.to_string(),
                None => "".to_string(),
            },
            self.internets
                .iter()
                .map(|internet| format!("\n{}", internet.to_str()))
                .collect::<std::string::String>(),
        ]
    }

    /// Write CSV file.
    ///
    /// # Arguments
    /// * `results` - Results.
    ///
    /// # Returns
    /// Path to the CSV file.
    pub(super) fn write_csv(
        results: &std::vec::Vec<Self>,
    ) -> std::result::Result<std::string::String, std::io::Error> {
        let mut exe_dir: std::path::PathBuf = std::env::current_exe()?;
        let _: bool = exe_dir.pop();
        let path: std::string::String = exe_dir
            .join(format!(
                "{}.csv",
                format!(
                    "results_{}_{}",
                    chrono::Local::now().format("%Y%m%d_%H%M%S").to_string(),
                    rand::RngCore::next_u64(&mut rand::rng())
                )
            ))
            .to_str()
            .unwrap()
            .to_string();

        let mut writer: csv::Writer<std::fs::File> = csv::Writer::from_writer(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(false)
                .create_new(true)
                .open(&path)?,
        );
        writer.write_record(&Self::csv_title_row())?;
        for result in results {
            writer.write_record(&result.csv_row())?;
        }
        writer.flush()?;

        return Ok(path);
    }

    /// Generate a key for sorting.
    pub(super) fn sort_key(&self) -> std::primitive::u32 {
        self.euros_per_square_meter_house.unwrap_or(0)
    }
}
