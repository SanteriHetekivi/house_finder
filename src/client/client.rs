/// HTTP client for making requests to the APIs.

pub(self) struct CallsPerMinute {
    pub(self) count: std::primitive::u8,
    pub(self) zeroed: tokio::time::Instant,
}

struct DomainRateLimit {
    pub(self) last_call_time: std::option::Option<tokio::time::Instant>,
    pub(self) calls_per_minute: CallsPerMinute,
}

static RATE_LIMIT_DATA: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, DomainRateLimit>>>,
> = once_cell::sync::Lazy::new(|| {
    std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()))
});

pub(crate) struct Client {
    pub(self) client: reqwest::Client,
    pub(self) cache: std::option::Option<crate::cache::Cache>,
    pub(self) max_every_ms: std::primitive::u16,
    pub(self) max_per_minute: std::option::Option<std::primitive::u8>,
}

impl Client {
    /// Initilizes a new client.
    ///
    /// # Arguments
    /// * `cache_name` - If provided, the client will cache the responses with the given name.
    /// * `max_every_ms` - The client will rate limit the requests to domain to every this many milliseconds, set to 0 to turn off.
    /// * `max_per_minute` - If provided, the client will rate limit the requests to the given amount per minute.
    pub(crate) fn new(
        cache_name: std::option::Option<&std::primitive::str>,
        max_every_ms: std::primitive::u16,
        max_per_minute: std::option::Option<std::primitive::u8>,
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
            max_every_ms,
            max_per_minute,
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

    /// Get the main domain from the given URL.
    ///
    /// # Arguments
    /// * `url` - URL to get the main domain from.
    pub(self) fn get_main_domain(url: &str) -> std::result::Result<String, url::ParseError> {
        let parsed_url: url::Url = url::Url::parse(url)?;
        let parts: Vec<&str> = parsed_url
            .host_str()
            .ok_or(url::ParseError::EmptyHost)?
            .split('.')
            .collect();
        if parts.len() < 2 {
            return Err(url::ParseError::EmptyHost);
        }
        Ok(format!(
            "{}.{}",
            parts[parts.len() - 2],
            parts[parts.len() - 1]
        ))
    }

    /// Rate limit the requests.
    /// The rate limit is per main domain (www.test.com => test.com).
    /// The rate limit is 1 request per second per domain.
    /// If client has custom calls per minute rate limit, also use it.
    ///
    /// # Arguments
    /// * `url` - URL to get main domain from.
    pub(self) async fn rate_limit(&self, url: &std::primitive::str) -> Result<(), url::ParseError> {
        // No rate limit.
        if self.max_every_ms == 0 && self.max_per_minute.is_none() {
            return Ok(());
        }

        // Get data.
        let mut rate_limit_data: tokio::sync::MutexGuard<
            '_,
            std::collections::HashMap<String, DomainRateLimit>,
        > = RATE_LIMIT_DATA.lock().await;
        let domain_rate_limit: &mut DomainRateLimit = rate_limit_data
            .entry(Self::get_main_domain(url)?.to_string())
            .or_insert(DomainRateLimit {
                last_call_time: None,
                calls_per_minute: CallsPerMinute {
                    count: 0,
                    zeroed: tokio::time::Instant::now(),
                },
            });

        // Call every domain only once per second.
        if self.max_every_ms != 0 {
            if let Some(last_call_time) = domain_rate_limit.last_call_time {
                let duration: tokio::time::Duration =
                    tokio::time::Duration::from_millis(self.max_every_ms.into());
                if tokio::time::Instant::now().duration_since(last_call_time) < duration {
                    tokio::time::sleep(
                        duration - tokio::time::Instant::now().duration_since(last_call_time),
                    )
                    .await;
                }
            }
        }

        // If client has custom calls per minute rate limit, also use it.
        if let Some(max_per_minute) = self.max_per_minute {
            let minute: tokio::time::Duration = tokio::time::Duration::from_secs(60);
            // Zero the call count every minute.
            if minute
                <= tokio::time::Instant::now()
                    .duration_since(domain_rate_limit.calls_per_minute.zeroed)
            {
                domain_rate_limit.calls_per_minute.count = 0;
                domain_rate_limit.calls_per_minute.zeroed = tokio::time::Instant::now();
            }

            // Sleep if the rate limit per minute is reached.
            if max_per_minute <= domain_rate_limit.calls_per_minute.count {
                tokio::time::sleep(
                    minute
                        - tokio::time::Instant::now()
                            .duration_since(domain_rate_limit.calls_per_minute.zeroed),
                )
                .await;
                domain_rate_limit.calls_per_minute.count = 0;
                domain_rate_limit.calls_per_minute.zeroed = tokio::time::Instant::now();
            }

            // Increase the call count.
            domain_rate_limit.calls_per_minute.count += 1;
        }

        domain_rate_limit.last_call_time = Some(tokio::time::Instant::now());
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
        self.rate_limit(url).await?;

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

    /// Get regular expression first capture group from the given URL.
    /// If cache is enabled, the text is cached and returned from cache.
    ///
    /// # Arguments
    /// * `method` - Method for the request.
    /// * `url` - URL for the request.
    /// * `regex` - Regex to extract the value with.
    /// * `json` - Optional JSON data for the request.
    /// * `headers` - Optional headers for the request.
    pub(self) async fn regex(
        &self,
        method: reqwest::Method,
        url: &std::primitive::str,
        regex: &std::primitive::str,
        json: std::option::Option<serde_json::Value>,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<std::string::String, super::RegexError> {
        Ok(regex::Regex::new(regex)?
            .captures(&self.text(method, url, json, headers).await?)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string())
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
    pub(crate) async fn get_regex(
        &self,
        url: &std::primitive::str,
        regex: &std::primitive::str,
    ) -> std::result::Result<std::string::String, super::RegexError> {
        self.regex(reqwest::Method::GET, url, regex, None, None)
            .await
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
