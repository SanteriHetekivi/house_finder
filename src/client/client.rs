/// HTTP client for making requests to the APIs.
pub(crate) struct Client {
    pub(self) client: reqwest::Client,
    pub(self) cache: std::option::Option<crate::cache::Cache>,
}

impl Client {
    /// Initilizes a new client.
    ///
    /// # Arguments
    /// * `cache_name` - If provided, the client will cache the responses with the given name.
    pub(crate) fn new(
        cache_name: std::option::Option<&std::primitive::str>,
    ) -> std::result::Result<Self, super::RequestError> {
        return Ok(Self {
            client: reqwest::Client::builder().cookie_store(true).build()?,
            cache: match cache_name {
                Some(cache_name) => Some(crate::cache::Cache::new(
                    &format!("client/{}", cache_name),
                    "json",
                )?),
                None => None,
            },
        });
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
        response: reqwest::Response,
        json: std::option::Option<serde_json::Value>,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        let url: std::string::String = response.url().to_string();
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

    /// Make a GET request.
    ///
    /// # Arguments
    /// * `url` - To this URL.
    pub(self) async fn get(
        &self,
        url: &std::primitive::str,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        self.client.get(url).send().await?.error_for_status()
    }

    /// Get text from the given URL.
    /// If cache is enabled, the text is cached and returned from cache.
    ///
    /// # Arguments
    /// * `url` - URL to get text from.
    pub(self) async fn get_text(
        &self,
        url: &std::primitive::str,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        match self.text_from_cache(url, None)? {
            Some(text) => return Ok(text),
            None => return Ok(self.response_to_text(self.get(url).await?, None).await?),
        }
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
        Ok(serde_json::from_str::<T>(&self.get_text(url).await?)?)
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
        Ok(regex::Regex::new(regex)?
            .captures(&self.get_text(url).await?)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string())
    }

    /// Make a POST request.
    ///
    /// # Arguments
    /// * `url` - To this URL.
    /// * `json` - JSON data to send.
    /// * `headers` - Optional headers to send in addition to Content-Type: application/json.
    pub(self) async fn post(
        &self,
        url: &std::primitive::str,
        json: serde_json::Value,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(url)
            .headers(headers.unwrap_or(reqwest::header::HeaderMap::new()))
            .header("Content-Type", "application/json")
            .json(&json)
            .send()
            .await?
            .error_for_status()
    }

    /// Make POST request and get text.
    /// If cache is enabled, the text is cached and returned from cache.
    ///
    /// # Arguments
    /// * `url` - URL to post to.
    /// * `json` - JSON data to send.
    /// * `headers` - Optional headers to send in addition to Content-Type: application/json.
    pub(self) async fn post_text(
        &self,
        url: &std::primitive::str,
        json: serde_json::Value,
        headers: std::option::Option<reqwest::header::HeaderMap>,
    ) -> std::result::Result<std::string::String, super::RequestError> {
        match self.text_from_cache(url, Some(json.clone()))? {
            Some(text) => return Ok(text),
            None => {
                return self
                    .response_to_text(self.post(url, json.clone(), headers).await?, Some(json))
                    .await
            }
        }
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
        Ok(serde_json::from_str::<T>(
            &self.post_text(url, json, headers).await?,
        )?)
    }
}
