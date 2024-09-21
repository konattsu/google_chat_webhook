use reqwest::{header, StatusCode};

use crate::{WebhookError, Message};

use super::{error_handling::DetailResponseError, mess_serialization::MessageWrapper};

// for async
#[derive(Debug)]
pub struct ApiClientAsync {
    client: reqwest::Client,
}
impl ApiClientAsync {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn post_message(
        &self,
        message: Message,
        url: &str,
    ) -> Result<(), WebhookError> {
        let response = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(MessageWrapper::from(message).serialize_safe())
            .send()
            .await
            .map_err(|e| WebhookError::NetworkError(e.to_string()))?;

        let status = response.status();
        let text = response.text().await;
        parse_response(status, text)
    }
}

// for sync
#[cfg(feature = "blocking")]
#[derive(Debug)]
pub struct ApiClientSync {
    client: reqwest::blocking::Client,
}
#[cfg(feature = "blocking")]
impl ApiClientSync {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn post_message(&self, message: Message, url: &str) -> Result<(), WebhookError> {
        let response = self
            .client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(MessageWrapper::from(message).serialize_safe())
            .send()
            .map_err(|e| WebhookError::NetworkError(e.to_string()))?;

        let status = response.status();
        let text = response.text();
        parse_response(status, text)
    }
}

// common process
type ResponseText = reqwest::Result<String>;

fn parse_response(status: StatusCode, text: ResponseText) -> Result<(), WebhookError> {
    if status.is_success() {
        Ok(())
    } else if status.is_client_error() {
        Err(invalid_call(text))
    } else {
        Err(external_service_error(text))
    }
}

fn invalid_call(text: ResponseText) -> WebhookError {
    WebhookError::InvalidCall(
        text.detail_resp_err("Client error", "Error reading response body"),
    )
}

fn external_service_error(text: ResponseText) -> WebhookError {
    WebhookError::ExternalServiceError(
        text.detail_resp_err("Server error", "Error reading response body"),
    )
}
