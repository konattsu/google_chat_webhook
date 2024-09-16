use crate::{DomainError, ExternalApi, Message};

pub struct ChatService<T: ExternalApi> {
    api: T,
}

impl<T> ChatService<T>
where
    T: ExternalApi,
{
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub fn post_message(&self, message: Message) -> Result<(), DomainError> {
        self.api.post_data(message)
    }
}
