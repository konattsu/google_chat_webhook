mod application;
mod domain;
mod infrastructure;

pub use application::ChatService;
pub use domain::DomainError;
pub use domain::ExternalApi;
pub use domain::Message;
pub use infrastructure::ExternalApiStruct;

pub fn post_google_chat_webhook(
    webhook_url: String,
    message: Message,
) -> Result<(), DomainError> {
    let api_impl = ExternalApiStruct::new(&webhook_url);
    let service = ChatService::new(api_impl);
    service.post_message_actual_call(message)
}
