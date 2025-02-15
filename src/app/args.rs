// Arguments:
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    // Search criteria for publishing time.
    #[arg(
        long,
        default_value="ANY_DAY",
        value_parser = clap::builder::PossibleValuesParser::new(
            ["ANY_DAY", "WITHIN_ONE_DAY", "WITHIN_TWO_DAYS", "WITHIN_SEVEN_DAYS", "WITHIN_TWO_WEEKS"]
        ),
        help = "Search criteria for publishing time. One of: ANY_DAY, WITHIN_ONE_DAY, WITHIN_TWO_DAYS, WITHIN_SEVEN_DAYS or WITHIN_TWO_WEEKS"
    )]
    pub(super) publishing_time_search_criteria: std::string::String,

    // Maximum price.
    #[arg(long, help = "Max price in euros.")]
    pub(super) price_max: Option<std::primitive::u32>,

    // Cities.
    #[arg(long, help = "Cities.")]
    pub(super) cities: Vec<std::string::String>,

    // Location latitude.
    #[arg(long, help = "Location latitude.", requires = "location_longitude")]
    pub(super) location_latitude: Option<std::primitive::f64>,

    // Location longitude.
    #[arg(long, help = "Location longitude.", requires = "location_latitude")]
    pub(super) location_longitude: Option<std::primitive::f64>,

    // OpenRouteService authorization token: https://openrouteservice.org/sign-up/
    #[arg(long, help = "OpenRouteService authorization token.")]
    pub(super) open_route_service_token: Option<std::string::String>,

    // Telegram bot token: https://core.telegram.org/bots#botfather
    #[arg(long, help = "Telegram bot token.", requires = "telegram_user_id")]
    pub(crate) telegram_bot_token: Option<std::string::String>,

    // Send Telegram message to this user id: https://core.telegram.org/bots/api#user
    #[arg(
        long,
        help = "Send Telegram message to this user id.",
        requires = "telegram_bot_token"
    )]
    pub(crate) telegram_user_id: Option<std::primitive::u64>,

    // Cache Elisa fixedBroadbandProducts result?
    #[arg(long, action, help = "Cache Elisa fixedBroadbandProducts result?")]
    pub(super) cache_elisa_fixed_broadband_products: bool,

    // Cache Etuovi announcements search result?
    #[arg(long, action, help = "Cache Etuovi announcements search result?")]
    pub(super) cache_etuovi_announcements: bool,

    // Cache Etuovi HTML?
    #[arg(long, action, help = "Cache Etuovi HTML?")]
    pub(super) cache_etuovi_html: bool,

    // Minimun area (m²) of the house.
    #[arg(long, action, help = "Minimum area (m²) of the house.")]
    pub(super) house_min_square_meters: Option<std::primitive::u16>,

    // Maximum distance to house from location in kilometers.
    #[arg(
        long,
        action,
        help = "Maximum distance to house from location in kilometers.",
        requires = "location_longitude",
        requires = "location_latitude"
    )]
    pub(super) max_distance_km: Option<std::primitive::u16>,

    // Minimum megabits per second for the internet.
    #[arg(long, action, help = "Minimum megabits per second for the internet.")]
    pub(super) min_mbps: Option<std::primitive::u32>,

    // Exclude house if it's text data has one of these texts.
    #[arg(long, help = "Exclude house if it's text data has one of these texts.")]
    pub(super) exclude_texts: Vec<std::string::String>,
}
