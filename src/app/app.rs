/// Run the application.
///
/// # Arguments
/// * `args` - Application arguments.
pub(crate) async fn run(
    args: super::Args,
    telegram: std::option::Option<crate::telegram::Telegram>,
) -> std::result::Result<std::primitive::u128, super::Error> {
    let mut handles: std::vec::Vec<
        tokio::task::JoinHandle<std::result::Result<std::primitive::u128, super::Error>>,
    > = std::vec::Vec::<
        tokio::task::JoinHandle<std::result::Result<std::primitive::u128, super::Error>>,
    >::new();
    let location_comparison: std::option::Option<longitude::Location> =
        match args.location_latitude.clone() {
            Some(location_latitude) => match args.location_longitude.clone() {
                Some(location_longitude) => Some(longitude::Location::from(
                    location_latitude,
                    location_longitude,
                )),
                // Should not happen if clap is configured and working correctly.
                None => std::panic!("--location-latitude was given but not --location-longitude!"),
            },
            None => {
                if args.telegram_user_id.is_some() {
                    // Should not happen if clap is configured and working correctly.
                    std::panic!("--location-longitude was given but not --location-latitude!");
                }
                None
            }
        };
    let cache: std::primitive::bool = args.cache;
    let open_route_service_token: std::option::Option<std::string::String> =
        args.open_route_service_token.clone();
    let telegram: std::option::Option<crate::telegram::Telegram> = telegram.clone();
    let cities: std::vec::Vec<std::string::String> = args.cities.clone();
    handles.push(tokio::task::spawn(async move {
        etuovi(
            &args.publishing_time_search_criteria,
            location_comparison,
            cache,
            open_route_service_token,
            telegram,
            args.price_max,
            cities,
        )
        .await
    }));

    let mut count: std::primitive::u128 = 0;
    for handle in handles {
        count += handle.await??;
    }

    return Ok(count);
}

/// Handle Etuovi announcements.
///
/// # Arguments
/// * `publishing_time_search_criteria` - Search criteria for publishing time.
/// * `location_comparison` - Optional location to compare against.
/// * `cache` - Cache data that can be changed?
/// * `open_route_service_token` - Optional OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `telegram` - Optional Telegram bot.
/// * `price_max` - Optional maximum price.
/// * `cities` - Cities.
pub(self) async fn etuovi(
    publishing_time_search_criteria: &std::primitive::str,
    location_comparison: std::option::Option<longitude::Location>,
    cache: std::primitive::bool,
    open_route_service_token: std::option::Option<std::string::String>,
    telegram: std::option::Option<crate::telegram::Telegram>,
    price_max: std::option::Option<std::primitive::u32>,
    cities: std::vec::Vec<std::string::String>,
) -> std::result::Result<std::primitive::u128, super::Error> {
    let mut handles: std::vec::Vec<
        tokio::task::JoinHandle<std::result::Result<std::primitive::bool, super::Error>>,
    > = std::vec::Vec::<
        tokio::task::JoinHandle<std::result::Result<std::primitive::bool, super::Error>>,
    >::new();
    for announcement in
        crate::etuovi::Etuovi::new(cache, publishing_time_search_criteria, price_max, cities)?
            .announcements()
            .await?
            .iter()
    {
        let announcement: crate::etuovi::Announcement = announcement.clone();
        let location_comparison: std::option::Option<longitude::Location> =
            location_comparison.clone();
        let open_route_service_token: std::option::Option<std::string::String> =
            open_route_service_token.clone();
        let telegram: std::option::Option<crate::telegram::Telegram> = telegram.clone();
        handles.push(tokio::task::spawn(async move {
            etuovi_announcement(
                announcement,
                location_comparison,
                cache,
                open_route_service_token,
                telegram,
            )
            .await
        }));
    }

    let mut count: std::primitive::u128 = 0;

    for handle in handles {
        if handle.await?? {
            count += 1;
        }
    }
    return Ok(count);
}

/// Handle Etuovi announcement.
///
/// # Arguments
/// * `announcement` - Etuovi announcement.
/// * `location_comparison` - Optional location_comparison to compare against.
/// * `cache` - Cache data that can be changed?
/// * `open_route_service_token` - OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `telegram` - Optional Telegram bot.
pub(self) async fn etuovi_announcement(
    announcement: crate::etuovi::Announcement,
    location_comparison: std::option::Option<longitude::Location>,
    cache: std::primitive::bool,
    open_route_service_token: std::option::Option<std::string::String>,
    telegram: std::option::Option<crate::telegram::Telegram>,
) -> std::result::Result<std::primitive::bool, super::Error> {
    let mut house: crate::app::house::House = crate::app::House::new(
        &announcement.url(),
        announcement.location(),
        announcement.square_meters_house(),
        announcement.square_meters_total(),
        announcement.euros(),
        &announcement.street_address(),
        announcement.year(),
        location_comparison.clone(),
        open_route_service_token,
        cache,
    );

    if !house.include().await? {
        return Ok(false);
    }

    let message: std::string::String = house
        .message(&announcement.postal_code(cache).await?)
        .await?;
    if let Some(telegram) = &telegram {
        let _: teloxide::prelude::Message = telegram.send_message(&message).await?;
    }
    println!("{}\n", message);
    return Ok(true);
}
