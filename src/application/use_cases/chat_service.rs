use crate::{DomainError, ExternalApi, Message};

pub struct ChatService<T: ExternalApi> {
    api: T,
}

impl<T> ChatService<T>
where
    T: ExternalApi,
{
    // この引数にinfrastructure層とかで定義したもの(traitはdomain層の)を
    // 入れることで依存性の逆転
    pub fn new(api: T) -> Self {
        Self { api }
    }

    pub fn post_message_actual_call(
        &self,
        message: Message,
    ) -> Result<(), DomainError> {
        self.api.post_message(message)
    }
}
