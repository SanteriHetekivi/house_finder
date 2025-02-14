/// OpenRouteService API client.
pub(crate) struct OpenRouteService {
    headers: reqwest::header::HeaderMap,
}

static LIMITER: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<crate::client::CallsPerMinute>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::Mutex::new(crate::client::CallsPerMinute::new(
        // OpenRouteServices has a rate limit of 40 requests per minute for free users.
        40,
    )))
});

impl OpenRouteService {
    /// Create new OpenRouteService API client.
    ///
    /// # Arguments
    /// * `token` - OpenRouteService authorization token: https://openrouteservice.org/sign-up/
    pub(crate) fn new(
        token: &std::primitive::str,
    ) -> Result<Self, reqwest::header::InvalidHeaderValue> {
        let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
        let _: bool = headers.append(
            "Authorization",
            reqwest::header::HeaderValue::from_str(token)?,
        );
        Ok(Self { headers })
    }

    /// Get biking distance in kilometers between two locations.
    ///
    /// # Arguments
    /// * `from` - Starting location.
    /// * `to` - Destination location.
    pub(crate) async fn biking_km(
        &self,
        from: longitude::Location,
        to: longitude::Location,
    ) -> std::result::Result<std::primitive::u16, crate::client::JSONError> {
        Ok(crate::client::Client::new(
            // Always caching cycling directions, because the API is rate limited and they should not change.
            Some("open_route_service/directions/cycling-regular"),
            Some(std::sync::Arc::clone(&LIMITER)),
        )?
        .post_json::<super::Response>(
            "https://api.openrouteservice.org/v2/directions/cycling-regular/json",
            serde_json::json!(
                {
                    "coordinates": [
                        [from.longitude, from.latitude],
                        [to.longitude, to.latitude]
                    ],
                    "preference": "recommended",
                    "language": "en",
                    "units": "km",
                    // Do not include any additional information in the response.
                    "instructions": false,
                    "maneuvers": false,
                    "geometry": false,
                    "elevation": false,
                }
            ),
            Some(self.headers.clone()),
        )
        .await?
        .routes[0]
            .summary
            .distance
            .ceil() as std::primitive::u16)
    }
}
