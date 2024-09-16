#[derive(Debug, Clone)]
pub struct Message {
    text: String,
    thread_id: Option<String>,
}

impl Message {
    pub fn new(text: String, thread: Option<String>) -> Self {
        Self {
            text,
            thread_id: thread,
        }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn get_thread(&self) -> Option<String> {
        self.thread_id.clone()
    }

    pub fn has_thread(&self) -> bool {
        self.thread_id.is_some()
    }
}

// #[derive(Deserialize, Serialize, Debug)]
// pub struct PostMess {
//     text: String,
// }
// impl PostMess {
//     pub fn new(text: String) -> Self {
//         Self { text }
//     }
// }

// #[derive(Deserialize, Serialize, Debug)]
// pub struct PostMessWithThread {
//     text: String,
//     thread: Thread,
// }
// #[derive(Deserialize, Serialize, Debug)]
// struct Thread {
//     name: String,
// }
// impl PostMessWithThread {
//     pub fn new(text: String, thread: String) -> Self {
//         Self {
//             text,
//             thread: Thread { name: thread },
//         }
//     }
// }

// これは変換後の形式(変換はpost時にintoとかで)
// 内部で保持するときはstruct(threadはops)とかでいいかも
