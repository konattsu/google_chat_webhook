use std::env;

use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct PostMess {
    text: String,
}
impl PostMess {
    fn new(text: String) -> Self {
        Self { text }
    }
}
impl Message for PostMess {}

#[derive(Deserialize, Serialize, Debug)]
struct PostMessWithThread {
    text: String,
    thread: Thread,
}
#[derive(Deserialize, Serialize, Debug)]
struct Thread {
    name: String,
}
impl PostMessWithThread {
    fn new(text: String, thread: String) -> Self {
        Self {
            text,
            thread: Thread { name: thread },
        }
    }
}
impl Message for PostMessWithThread {}

#[allow(unused)]
trait Message {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");
    const THREAD_URL: &str = "&messageReplyOption=REPLY_MESSAGE_OR_FAIL";
    let url = format!("{}{}", url, THREAD_URL);

    let thread =
        env::var("WEBHOOK_THREAD_REPLY").expect("WEBHOOK_THREAD_REPLY must be set");

    // let mess = PostMess::new("Hello webhook from rust!".into());
    let mess =
        PostMessWithThread::new("Reply to `Hello webhook from rust!`".into(), thread);

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&mess).unwrap())
        .send()
        .await?;

    println!("res: {}", res.text().await?);
    Ok(())
}
