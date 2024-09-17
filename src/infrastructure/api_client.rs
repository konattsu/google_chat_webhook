use reqwest::{blocking::Client, header};

use crate::{DomainError, Message};

use super::{error_handling::DetailResponseError, mess_serialization::MessageWrapper};

/// 非同期 (blocking::Client)
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn post_message(&self, message: Message, url: &str) -> Result<(), DomainError> {
        let response = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(MessageWrapper::from(message).serialize_safe())
            .send()
            .map_err(|e| DomainError::NetworkError(e.to_string()))?;

        let status = response.status();

        if status.is_success() {
            Ok(())
        } else if status.is_client_error() {
            Err(DomainError::InvalidCall(response.text().detail_resp_err(
                "Client error",
                "Error reading response body",
            )))
        } else {
            Err(DomainError::ExternalServiceError(
                response
                    .text()
                    .detail_resp_err("Server error", "Error reading response body"),
            ))
        }
    }
}
