#[cfg(feature = "blocking")]
use crate::ExternalApiSync;
use crate::{ExternalApiAsync, Message, WebhookError};

use super::api_client::ApiClientAsync;
#[cfg(feature = "blocking")]
use super::api_client::ApiClientSync;

#[derive(Debug)]
pub struct ExternalApiStructAsync {
    client: ApiClientAsync,
    webhook_url: String,
}

impl ExternalApiStructAsync {
    pub fn new(url: &str) -> Self {
        Self {
            client: ApiClientAsync::new(),
            webhook_url: url.to_owned(),
        }
    }
}

#[async_trait::async_trait]
impl ExternalApiAsync for ExternalApiStructAsync {
    async fn post_message(&self, message: Message) -> Result<(), WebhookError> {
        let url = convert_url(&message, &self.webhook_url);
        self.client.post_message(message, &url).await
    }
}

#[cfg(feature = "blocking")]
#[derive(Debug)]
pub struct ExternalApiStructSync {
    client: ApiClientSync,
    webhook_url: String,
}

#[cfg(feature = "blocking")]
impl ExternalApiStructSync {
    pub fn new(url: &str) -> Self {
        Self {
            client: ApiClientSync::new(),
            webhook_url: url.to_owned(),
        }
    }
}

#[cfg(feature = "blocking")]
impl ExternalApiSync for ExternalApiStructSync {
    fn post_message(&self, message: crate::Message) -> Result<(), WebhookError> {
        let url = convert_url(&message, &self.webhook_url);
        self.client.post_message(message, &url)
    }
}

// https://developers.google.com/workspace/chat/api/reference/
// rest/v1/spaces.messages/create
fn convert_url(message: &Message, url: &str) -> String {
    if !message.has_thread() {
        return url.to_owned();
    }

    format!(
        "{}{}",
        url,
        if message.reply_only().unwrap() {
            "&messageReplyOption=REPLY_MESSAGE_OR_FAIL"
        } else {
            "&messageReplyOption=REPLY_MESSAGE_FALLBACK_TO_NEW_THREAD"
        }
    )
}
