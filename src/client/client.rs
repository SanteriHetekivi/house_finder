/// HTTP client for making requests to the APIs.
pub(crate) struct Client<L: super::Limiter> {
    pub(self) client: reqwest::Client,
    pub(self) cache: std::option::Option<crate::cache::Cache>,
    pub(self) limiter: std::option::Option<std::sync::Arc<tokio::sync::Mutex<L>>>,
}

impl<L: super::Limiter> Client<L> {
    /// Initilizes a new client.
    ///
    /// # Arguments
    /// * `cache_name` - If provided, the client will cache the responses with the given name.
    /// * `max_every_ms` - The client will rate limit the requests to domain to every this many milliseconds, set to 0 to turn off.
    /// * `max_per_minute` - If provided, the client will rate limit the requests to the given amount per minute.
    pub(crate) fn new(
        cache_name: std::option::Option<&std::primitive::str>,
        limiter: std::option::Option<std::sync::Arc<tokio::sync::Mutex<L>>>,
    ) -> std::result::Result<Self, super::RequestError> {
        Ok(Self {
            client: reqwest::Client::builder().cookie_store(true).build()?,
            cache: match cache_name {
                Some(cache_name) => Some(crate::cache::Cache::new(
                    &format!("client/{}", cache_name),
                    "json",
                )?),
                None => None,
            },
            limiter,
        })
    }

    /// Generate cache from given data.
    ///
    /// # Arguments
    /// * `url` - URL of the request.
    /// * `json` - Optional JSON data of the request.
    pub(self) fn cache_key(
        url: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
    ) -> std::string::String {
        let mut key: std::string::String = url.to_string();
        match json {
            Some(json) => {
                key.push_str(&json.to_string());
            }
            None => {}
        }
        return key;
    }

    /// Convert response to text and cache it, if cache is enabled.
    ///
    /// # Arguments
    /// * `response` - Response to get text from.
    /// * `json` - Optional JSON data of the request to use as part of cache key.
    pub(self) async fn response_to_text(
        &self,
        url: &std::primitive::str,
        response: reqwest::Response,
        json: std::option::Option<serde_json::Value>,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        let text: std::string::String = response.text().await?;
        match self.cache.as_ref() {
            Some(cache) => {
                cache.write(&Self::cache_key(&url, json), &text)?;
            }
            None => {}
        }
        return Ok(text);
    }

    /// Get text from cache if cache is on and the data exists.
    ///
    /// # Arguments
    /// * `url` - URL of the request to use as part of cache key.
    /// * `json` - Optional JSON data of the request to use as part of cache key.
    pub(self) fn text_from_cache(
        &self,
        url: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
    ) -> std::result::Result<Option<std::string::String>, std::io::Error> {
        let cache_key: std::string::String = Self::cache_key(url, json);
        match self.cache.as_ref() {
            Some(cache) => {
                if cache.exists(&cache_key) {
                    return Ok(Some(cache.read(&cache_key)?));
                }
                return Ok(None);
            }
            None => return Ok(None),
        }
    }

    /// Rate limit the requests.
    pub(self) async fn rate_limit(&self) -> Result<(), url::ParseError> {
        if let Some(limiter) = &self.limiter {
            let mut limiter: tokio::sync::MutexGuard<'_, L> = limiter.lock().await;
            limiter.limit().await;
        }
        return Ok(());
    }

    /// Make a request.
    ///
    /// # Arguments
    /// * `method` - Method for the request.
    /// * `url` - URL for the request.
    /// * `json` - Optional JSON data for the request.
    /// * `headers` - Optional headers for the request.
    pub(self) async fn request(
        &self,
        method: reqwest::Method,
        url: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<reqwest::Response, super::RequestError> {
        self.rate_limit().await?;

        let mut builder: reqwest::RequestBuilder = self.client.request(method, url);
        if let Some(headers) = headers {
            builder = builder.headers(headers);
        }
        if let Some(json) = json {
            builder = builder.json(&json);
        }

        return Ok(builder.send().await?.error_for_status()?);
    }

    /// Get text from the given URL.
    /// If cache is enabled, the text is cached and returned from cache.
    ///
    /// # Arguments
    /// * `method` - Method for the request.
    /// * `url` - URL for the request.
    /// * `json` - Optional JSON data for the request.
    /// * `headers` - Optional headers for the request.
    pub(self) async fn text(
        &self,
        method: reqwest::Method,
        url: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        match self.text_from_cache(url, json.clone())? {
            Some(text) => return Ok(text),
            None => {
                return Ok(self
                    .response_to_text(
                        url,
                        self.request(method, url, json.clone(), headers).await?,
                        json,
                    )
                    .await?);
            }
        }
    }

    /// Get JSON from the given URL.
    /// If cache is enabled, the JSON is cached and returned from cache.
    ///
    /// # Arguments
    /// * `method` - Method for the request.
    /// * `url` - URL for the request.
    /// * `json` - Optional JSON data for the request.
    /// * `headers` - Optional headers for the request.
    pub(self) async fn json<T>(
        &self,
        method: reqwest::Method,
        url: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<T, super::JSONError>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut headers_real: reqwest::header::HeaderMap =
            headers.unwrap_or(reqwest::header::HeaderMap::new());
        let _: std::option::Option<reqwest::header::HeaderValue> = headers_real.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        return Ok(serde_json::from_str::<T>(
            &self.text(method, url, json, Some(headers_real)).await?,
        )?);
    }

    /// Get JSON from the given URL.
    /// If cache is enabled, the JSON is cached and returned from cache.
    /// The JSON is deserialized to the given type.
    ///
    /// # Arguments
    /// * `T` - Type to deserialize JSON to.
    /// * `url` - URL to get JSON from.
    pub(crate) async fn get_json<T>(
        &self,
        url: &std::primitive::str,
    ) -> std::result::Result<T, super::JSONError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.json(reqwest::Method::GET, url, None, None).await
    }

    /// Get text from the given URL and extract a value using regex.
    /// If cache is enabled, the text is cached and returned from cache.
    ///
    /// # Arguments
    /// * `url` - URL to get text from.
    /// * `regex` - Regex to extract the value with.
    pub(crate) async fn get_text(
        &self,
        url: &std::primitive::str,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        self.text(reqwest::Method::GET, url, None, None).await
    }

    /// Make POST request and get JSON.
    /// If cache is enabled, the JSON is cached and returned from cache.
    /// The JSON is deserialized to the given type.
    ///
    /// # Arguments
    /// * `T` - Type to deserialize JSON to.
    /// * `url` - URL to post to.
    /// * `json` - JSON data to send.
    /// * `headers` - Optional headers to send in addition to Content-Type: application/json.
    pub(crate) async fn post_json<T>(
        &self,
        url: &std::primitive::str,
        json: serde_json::Value,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<T, super::JSONError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.json(reqwest::Method::POST, url, Some(json), headers)
            .await
    }
}
