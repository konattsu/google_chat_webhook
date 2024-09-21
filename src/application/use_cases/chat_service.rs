use crate::ExternalApiAsync;
#[cfg(feature = "blocking")]
use crate::ExternalApiSync;

use crate::{WebhookError, Message};

pub struct ChatServiceAsync<T: ExternalApiAsync> {
    api: T,
}

impl<T> ChatServiceAsync<T>
where
    T: ExternalApiAsync,
{
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub async fn post_message_actual_call(
        &self,
        message: Message,
    ) -> Result<(), WebhookError> {
        self.api.post_message(message).await
    }
}

#[cfg(feature = "blocking")]
pub struct ChatServiceSync<T: ExternalApiSync> {
    api: T,
}

#[cfg(feature = "blocking")]
impl<T> ChatServiceSync<T>
where
    T: ExternalApiSync,
{
    // この引数にinfrastructure層とかで定義したもの(traitはdomain層の)を
    // 入れることで依存性の逆転
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub fn post_message_actual_call(
        &self,
        message: Message,
    ) -> Result<(), WebhookError> {
        self.api.post_message(message)
    }
}
