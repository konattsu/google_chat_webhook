use crate::{domain::DomainError, Message};

pub trait ExternalApi {
    fn post_message(&self, message: Message) -> Result<(), DomainError>;

    // asyncと迷ったけど今のところ軽いのでsyncに
    // fn post_data(
    //     &self,
    //     message: Message,
    // ) -> Pin<Box<dyn Future<Output = Result<(), DomainError>> + Send>>;
}
