/// Run the application.
///
/// # Arguments
/// * `args` - Application arguments.
pub(crate) async fn run(
    args: super::Args,
    telegram: std::option::Option<crate::telegram::Telegram>,
) -> std::result::Result<(), super::Error> {
    let mut handles: std::vec::Vec<
        tokio::task::JoinHandle<std::result::Result<std::vec::Vec<super::Result>, super::Error>>,
    > = std::vec::Vec::<
        tokio::task::JoinHandle<std::result::Result<std::vec::Vec<super::Result>, super::Error>>,
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
    let cache_etuovi_announcements: std::primitive::bool = args.cache_etuovi_announcements;
    let cache_etuovi_html: std::primitive::bool = args.cache_etuovi_html;
    let cache_elisa_fixed_broadband_products: std::primitive::bool =
        args.cache_elisa_fixed_broadband_products;
    let open_route_service_token: std::option::Option<std::string::String> =
        args.open_route_service_token.clone();
    let cities: std::vec::Vec<std::string::String> = args.cities.clone();
    let house_min_square_meters: std::option::Option<std::primitive::u16> =
        args.house_min_square_meters;
    let max_distance_km: std::option::Option<std::primitive::u16> = args.max_distance_km;
    let min_mbps: std::option::Option<std::primitive::u32> = args.min_mbps;
    let exclude_texts: std::vec::Vec<std::string::String> = args
        .exclude_texts
        .iter()
        .map(|text| text.to_lowercase())
        .collect();
    handles.push(tokio::task::spawn(async move {
        etuovi(
            &args.publishing_time_search_criteria,
            location_comparison,
            cache_etuovi_announcements,
            cache_etuovi_html,
            cache_elisa_fixed_broadband_products,
            open_route_service_token,
            args.price_max,
            cities,
            house_min_square_meters,
            max_distance_km,
            min_mbps,
            exclude_texts,
        )
        .await
    }));

    let mut results: std::vec::Vec<super::Result> = std::vec::Vec::<super::Result>::new();
    for handle in handles {
        for result in handle.await?? {
            results.push(result);
        }
    }

    let telegram: std::option::Option<crate::telegram::Telegram> = telegram.clone();
    if !results.is_empty() {
        results.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        for result in &results {
            let message: std::string::String = result.message();
            println!("{}", &message);
            if let Some(telegram) = &telegram {
                let _: teloxide::prelude::Message = telegram.send_message(&message).await?;
            }
        }
        println!("Wrote file: {}", super::Result::write_csv(&results)?);
    }

    let message: std::string::String = format!("Found {}!", results.len());
    println!("{}", message);
    if let Some(telegram) = &telegram {
        let _: teloxide::prelude::Message = telegram.send_message(&message).await?;
    }

    return Ok(());
}

/// Handle Etuovi announcements.
///
/// # Arguments
/// * `publishing_time_search_criteria` - Search criteria for publishing time.
/// * `location_comparison` - Optional location to compare against.
/// * `cache` - Cache data that can be changed?
/// * `open_route_service_token` - Optional OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `price_max` - Optional maximum price.
/// * `cities` - Cities.
/// * `house_min_square_meters` - Optional minimum square meters for the house.
/// * `max_distance_km` - Optional maximum distance in kilometers.
/// * `min_mbps` - Optional minimum megabits per second.
/// * `exclude_texts` - Exclude house if it's text data has one of these texts.
pub(self) async fn etuovi(
    publishing_time_search_criteria: &std::primitive::str,
    location_comparison: std::option::Option<longitude::Location>,
    cache_etuovi_announcements: std::primitive::bool,
    cache_etuovi_html: std::primitive::bool,
    cache_elisa_fixed_broadband_products: std::primitive::bool,
    open_route_service_token: std::option::Option<std::string::String>,
    price_max: std::option::Option<std::primitive::u32>,
    cities: std::vec::Vec<std::string::String>,
    house_min_square_meters: std::option::Option<std::primitive::u16>,
    max_distance_km: std::option::Option<std::primitive::u16>,
    min_mbps: std::option::Option<std::primitive::u32>,
    exclude_texts: std::vec::Vec<std::string::String>,
) -> std::result::Result<std::vec::Vec<super::Result>, super::Error> {
    let mut handles: std::vec::Vec<
        tokio::task::JoinHandle<std::result::Result<Option<super::Result>, super::Error>>,
    > = std::vec::Vec::<
        tokio::task::JoinHandle<std::result::Result<Option<super::Result>, super::Error>>,
    >::new();
    for announcement in crate::etuovi::Etuovi::new(
        cache_etuovi_announcements,
        cache_etuovi_html,
        publishing_time_search_criteria,
        price_max,
        cities,
    )?
    .announcements()
    .await?
    .iter()
    {
        let announcement: crate::etuovi::Announcement = announcement.clone();
        let location_comparison: std::option::Option<longitude::Location> =
            location_comparison.clone();
        let open_route_service_token: std::option::Option<std::string::String> =
            open_route_service_token.clone();
        let exclude_texts: std::vec::Vec<std::string::String> = exclude_texts.clone();
        handles.push(tokio::task::spawn(async move {
            etuovi_announcement(
                announcement,
                location_comparison,
                cache_elisa_fixed_broadband_products,
                open_route_service_token,
                house_min_square_meters,
                max_distance_km,
                min_mbps,
                exclude_texts,
            )
            .await
        }));
    }

    let mut results: std::vec::Vec<super::Result> = std::vec::Vec::<super::Result>::new();
    for handle in handles {
        if let Some(result) = handle.await?? {
            results.push(result);
        }
    }
    return Ok(results);
}

/// Handle Etuovi announcement.
///
/// # Arguments
/// * `announcement` - Etuovi announcement.
/// * `location_comparison` - Optional location_comparison to compare against.
/// * `cache_elisa_fixed_broadband_products` - Cache Elisa fixed broadband products?
/// * `open_route_service_token` - OpenRouteService authorization token: https://openrouteservice.org/sign-up/
/// * `house_min_square_meters` - Optional minimum square meters for the house.
/// * `max_distance_km` - Optional maximum distance in kilometers.
/// * `min_mbps` - Optional minimum megabits per second.
/// * `exclude_texts` - Exclude house if it's text data has one of these texts.
pub(self) async fn etuovi_announcement(
    announcement: crate::etuovi::Announcement,
    location_comparison: std::option::Option<longitude::Location>,
    cache_elisa_fixed_broadband_products: std::primitive::bool,
    open_route_service_token: std::option::Option<std::string::String>,
    house_min_square_meters: std::option::Option<std::primitive::u16>,
    max_distance_km: std::option::Option<std::primitive::u16>,
    min_mbps: std::option::Option<std::primitive::u32>,
    exclude_texts: std::vec::Vec<std::string::String>,
) -> std::result::Result<Option<super::Result>, super::Error> {
    return Ok(crate::app::House::<crate::etuovi::Announcement>::new(
        announcement,
        location_comparison.clone(),
        open_route_service_token,
        cache_elisa_fixed_broadband_products,
        house_min_square_meters,
        max_distance_km,
        min_mbps,
        exclude_texts,
    )
    .result()
    .await?);
}
