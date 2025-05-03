use std::{collections::HashMap, sync::LazyLock};

use axum::{
    Json,
    http::{HeaderMap, HeaderName, HeaderValue},
    response::Result,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::errors::ProxyApiError;
type Payload = HashMap<String, String>;
type Headers = HashMap<String, String>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum HttpMethod {
    GET { headers: Headers },
    POST { headers: Headers, body: Payload },
}

#[derive(Deserialize, Debug)]
pub struct ProxyRequest {
    url: String,
    method: HttpMethod,
}

// the output to our `create_user` handler
#[derive(Serialize, Debug)]
pub struct ProxyResponse {
    status_code: u16,
    text: String,
}

fn get_headers(map: &HashMap<String, String>) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    for (key, value) in map.iter() {
        if let (Ok(name), Ok(val)) = (
            HeaderName::try_from(key.as_str()),
            HeaderValue::from_str(value.as_str()),
        ) {
            headers.insert(name, val);
        } else {
            return Err(format!("Invalid header: {}: {}", key, value).into());
        }
    }

    Ok(headers)
}
static CLIENT: LazyLock<Client> = std::sync::LazyLock::new(reqwest::Client::new);

#[axum::debug_handler]
pub async fn post_proxy(
    Json(payload): Json<ProxyRequest>,
) -> Result<Json<ProxyResponse>, ProxyApiError> {
    info!("Received request to proxy: {:?}", payload);

    match payload.method {
        HttpMethod::GET { headers } => {
            // Handle GET request
            let headers = get_headers(&headers)
                .inspect_err(|e| {
                    error!("Error parsing headers: {:?}", e);
                })
                .map_err(|_| ProxyApiError::BadRequest)?;

            debug!("Handling GET request to {}", payload.url);
            debug!("Headers: {:?}", headers);

            let response = CLIENT
                .get(&payload.url)
                .headers(headers)
                .send()
                .await
                .unwrap();

            let status_code = response.status().as_u16();
            let text = response.text().await.unwrap();

            Ok(Json(ProxyResponse { status_code, text }))
        }
        HttpMethod::POST { headers, body } => {
            // Handle POST request
            debug!("Handling POST request to {}", payload.url);
            debug!("Headers: {:?}", headers);
            debug!("Payload: {:?}", body);

            let headers = get_headers(&headers)
                .inspect_err(|e| {
                    error!("Error parsing headers: {:?}", e);
                })
                .map_err(|_| ProxyApiError::BadRequest)?;

            let response = CLIENT
                .post(&payload.url)
                .json(&body)
                .headers(headers)
                .send()
                .await
                .unwrap();

            let status_code = response.status().as_u16();
            let text = response.text().await.unwrap();
            Ok(Json(ProxyResponse { status_code, text }))
        }
    }
}
