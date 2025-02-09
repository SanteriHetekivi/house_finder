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
    let cottage_location: longitude::Location =
        longitude::Location::from(args.cottage_latitude, args.cottage_longitude);
    let cache: std::primitive::bool = args.cache;
    let open_route_service_token: std::string::String = args.open_route_service_token.clone();
    let telegram: std::option::Option<crate::telegram::Telegram> = telegram.clone();
    let price_max: std::primitive::u32 = args.price_max;
    let cities: std::vec::Vec<std::string::String> = args.cities.clone();
    handles.push(tokio::task::spawn(async move {
        etuovi(
            &args.publishing_time_search_criteria,
            cottage_location,
            cache,
            &open_route_service_token,
            telegram,
            price_max,
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
/// * `cottage_location` - Cottage location.
/// * `cache` - Cache data that can be changed?
/// * `open_route_service_token` - OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `telegram` - Optional Telegram bot.
/// * `price_max` - Maximum price.
/// * `cities` - Cities.
pub(self) async fn etuovi(
    publishing_time_search_criteria: &std::primitive::str,
    cottage_location: longitude::Location,
    cache: std::primitive::bool,
    open_route_service_token: &std::primitive::str,
    telegram: std::option::Option<crate::telegram::Telegram>,
    price_max: std::primitive::u32,
    cities: std::vec::Vec<std::string::String>,
) -> std::result::Result<std::primitive::u128, super::Error> {
    let mut handles: std::vec::Vec<
        tokio::task::JoinHandle<std::result::Result<std::primitive::bool, super::Error>>,
    > = std::vec::Vec::<
        tokio::task::JoinHandle<std::result::Result<std::primitive::bool, super::Error>>,
    >::new();
    for announcement in
        crate::etuovi::Etuovi::new(cache, publishing_time_search_criteria, price_max, cities)
            .announcements()
            .await?
            .iter()
    {
        let announcement: crate::etuovi::Announcement = announcement.clone();
        let cottage_location: longitude::Location = cottage_location.clone();
        let open_route_service_token: std::string::String = open_route_service_token.to_string();
        let telegram: std::option::Option<crate::telegram::Telegram> = telegram.clone();
        handles.push(tokio::task::spawn(async move {
            etuovi_announcement(
                announcement,
                cottage_location,
                cache,
                &open_route_service_token,
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
/// * `cottage_location` - Cottage location.
/// * `cache` - Cache data that can be changed?
/// * `open_route_service_token` - OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `telegram` - Optional Telegram bot.
pub(self) async fn etuovi_announcement(
    announcement: crate::etuovi::Announcement,
    cottage_location: longitude::Location,
    cache: std::primitive::bool,
    open_route_service_token: &std::primitive::str,
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
        cottage_location.clone(),
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
        telegram.send_message(&message).await?;
    }
    println!("{}\n", message);
    return Ok(true);
}
