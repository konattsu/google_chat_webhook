mod application;
mod domain;
mod infrastructure;

pub use domain::Message;
pub use domain::Reply;
pub use domain::WebhookError;

use application::use_cases::chat_service::ChatServiceAsync;
#[cfg(feature = "blocking")]
use application::use_cases::chat_service::ChatServiceSync;
use domain::repositories::external_api::ExternalApiAsync;
#[cfg(feature = "blocking")]
use domain::repositories::external_api::ExternalApiSync;
use infrastructure::post::ExternalApiStructAsync;
#[cfg(feature = "blocking")]
use infrastructure::post::ExternalApiStructSync;

pub async fn post_google_chat_webhook(
    webhook_url: &str,
    message: Message,
) -> Result<(), WebhookError> {
    let api_impl = ExternalApiStructAsync::new(webhook_url);
    let service = ChatServiceAsync::new(api_impl);
    service.post_message_actual_call(message).await
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use crate::{ChatServiceSync, ExternalApiStructSync};
    use crate::{Message, WebhookError};

    pub fn post_google_chat_webhook(
        webhook_url: &str,
        message: Message,
    ) -> Result<(), WebhookError> {
        let api_impl = ExternalApiStructSync::new(webhook_url);
        let service = ChatServiceSync::new(api_impl);
        service.post_message_actual_call(message)
    }
}
