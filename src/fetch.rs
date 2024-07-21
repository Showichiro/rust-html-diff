use std::collections::HashMap;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

#[derive(Debug)]
pub enum FetchError {
    RequestFailed(reqwest::Error),
    StatusError(reqwest::StatusCode),
}

pub async fn fetch_url_content(
    url: &str,
    headers: &Option<HashMap<String, String>>,
) -> Result<String, FetchError> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);

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
