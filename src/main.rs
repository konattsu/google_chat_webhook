use chat_webhook::{ChatService, ExternalApiImpl, Message};

fn main() {
    let api_impl: ExternalApiImpl = Default::default();
    let service = ChatService::new(api_impl);
    let message = Message::new("Hello from app layer".to_string(), None);
    let res = service.post_message(message);
    print!("result: {:?}", res);
}

// use std::env;

// use chat_webhook::PostMessWithThread;
// use reqwest::header;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dotenv::dotenv().ok();
//     let url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
//     const THREAD_URL: &str = "&messageReplyOption=REPLY_MESSAGE_OR_FAIL";
//     let url = format!("{}{}", url, THREAD_URL);

//     let thread =
//         env::var("WEBHOOK_THREAD_REPLY").expect("WEBHOOK_THREAD_REPLY must be set");

//     // let mess = PostMess::new("Hello webhook from rust!".into());
//     let mess =
//         PostMessWithThread::new("Reply to `Hello webhook from rust!`".into(), thread);

//     let client = reqwest::Client::new();
//     let res = client
//         .post(url)
//         .header(header::CONTENT_TYPE, "application/json")
//         .body(serde_json::to_string(&mess).unwrap())
//         .send()
//         .await?;

//     println!("res: {}", res.text().await?);
//     Ok(())
// }
