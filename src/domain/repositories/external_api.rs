use crate::{domain::WebhookError, Message};

#[async_trait::async_trait]
pub trait ExternalApiAsync {
    async fn post_message(&self, message: Message) -> Result<(), WebhookError>;
}

#[cfg(feature = "blocking")]
pub trait ExternalApiSync {
    fn post_message(&self, message: Message) -> Result<(), WebhookError>;
}
