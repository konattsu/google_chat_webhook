use std::env;

use chat_webhook::{ChatService, ExternalApiStruct, Message};

fn main() {
    dotenv::dotenv().ok();
    let url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");

    let api_impl = ExternalApiStruct::new(&url);
    let service = ChatService::new(api_impl);
    let message = Message::new("integration test".to_string(), None);
    let res = service.post_message_actual_call(message);

    print!("result: {:?}", res);
}
