use crate::{DomainError, ExternalApi};

use super::api_client::ApiClient;

pub struct ExternalApiStruct {
    client: ApiClient,
    webhook_url: String,
}

impl ExternalApiStruct {
    pub fn new(url: &str) -> Self {
        Self {
            client: ApiClient::new(),
            webhook_url: url.to_owned(),
        }
    }
}

impl ExternalApi for ExternalApiStruct {
    fn post_message(&self, message: crate::Message) -> Result<(), DomainError> {
        let mut url = self.webhook_url.clone();

        if message.has_thread() {
            const THREAD_URL: &str = "&messageReplyOption=REPLY_MESSAGE_OR_FAIL";
            url = format!("{}{}", url, THREAD_URL);
        }

        self.client.post_message(message, &url)
    }
}
