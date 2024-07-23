use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};

#[derive(Debug)]
pub enum FetchError {
    RequestFailed(reqwest::Error),
    StatusError(reqwest::StatusCode),
}
pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_url_content(
        &self,
        url: &str,
        headers: &Option<HashMap<String, String>>,
    ) -> Result<String, FetchError> {
        let mut request = self.client.get(url);

        if let Some(headers) = headers {
            let header_map: HeaderMap = headers
                .iter()
                .map(|(k, v)| {
                    (
                        HeaderName::from_bytes(k.as_bytes()).unwrap(),
                        HeaderValue::from_str(v).unwrap(),
                    )
                })
                .collect();
            request = request.headers(header_map);
        }

        let response = request.send().await.map_err(FetchError::RequestFailed)?;

        if !response.status().is_success() {
            return Err(FetchError::StatusError(response.status()));
        }

        response.text().await.map_err(FetchError::RequestFailed)
    }
}

impl Default for Fetcher {
    fn default() -> Self {
        Self {
            client: Client::new(),
        }
    }
}
