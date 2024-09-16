use std::env;

use reqwest::header;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::json;

use crate::{DomainError, ExternalApi, Message};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ExternalApiImpl {
    //
}

impl ExternalApiImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for ExternalApiImpl {
    fn default() -> Self {
        Self::new()
    }
}

trait DetailResponseError {
    fn detail_resp_err(self, ok_prefix: &str, err_prefix: &str) -> String;
}
impl<T, E> DetailResponseError for Result<T, E>
where
    T: ToString,
    E: ToString,
{
    fn detail_resp_err(self, ok_prefix: &str, err_prefix: &str) -> String {
        match self {
            Ok(v) => format!("{}: {}", ok_prefix, v.to_string()),
            Err(e) => format!("{}: {}", err_prefix, e.to_string()),
        }
    }
}

/// Message構造体をシリアライズするため
#[derive(Debug, Clone)]
struct MessageWrapper(Message);

impl Serialize for MessageWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // thread_idがNoneのとき 2 でもいいっぽい
        let mut state = serializer.serialize_struct("MessageWrapper", 2)?;
        state.serialize_field("text", &self.0.get_text())?;

        if let Some(thread_id) = self.0.get_thread() {
            state.serialize_field("thread", &json!({"name": thread_id}))?;
        }

        state.end()
    }
}

impl From<Message> for MessageWrapper {
    fn from(value: Message) -> Self {
        Self(value)
    }
}

impl ExternalApi for ExternalApiImpl {
    fn post_data(&self, message: Message) -> Result<(), crate::DomainError> {
        dotenv::dotenv().ok();
        let mut url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");

        if message.has_thread() {
            const THREAD_URL: &str = "&messageReplyOption=REPLY_MESSAGE_OR_FAIL";
            url = format!("{}{}", url, THREAD_URL);
        }

        let client = reqwest::blocking::Client::new();
        let response = client
            .post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&MessageWrapper::from(message)).unwrap())
            .send()
            .map_err(|e| DomainError::NetworkError(e.to_string()))?;

        // println!("res: {}", response.text().unwrap());

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
