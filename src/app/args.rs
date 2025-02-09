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

    // Cottage latitude.
    #[arg(long, help = "Cottage latitude.")]
    pub(super) cottage_latitude: std::primitive::f64,

    // Cottage longitude.
    #[arg(long, help = "Cottage longitude.")]
    pub(super) cottage_longitude: std::primitive::f64,

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

    // Chache data that can be changed?
    #[arg(long, action, help = "Cache data that can be changed?")]
    pub(super) cache: bool,
}
