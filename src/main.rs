use std::env;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL must be set");

    let mess = PostMess::new("Hello webhook from rust!".into());

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .body(serde_json::to_string(&mess).unwrap())
        .send()
        .await?;

    println!("res: {:?}", res);
    Ok(())
}
