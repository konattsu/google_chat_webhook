/// 送信するメッセージ
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    text: String,
    reply: Option<Reply>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reply {
    thread_id: String,
    /// - `true`: `thread_id` が存在しないと失敗
    /// - `false`: `thread_id` が存在しないと、新しいスレッドを作成
    reply_only: bool,
}

impl Message {
    pub fn new(text: String, reply: Option<Reply>) -> Self {
        Self { text, reply }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn has_thread(&self) -> bool {
        self.reply.is_some()
    }

    pub fn get_thread_id(&self) -> Option<String> {
        Some(self.reply.clone()?.get_thread_id())
    }

    pub fn reply_only(&self) -> Option<bool> {
        Some(self.reply.clone()?.reply_only())
    }
}

impl Reply {
    pub fn new(thread_id: String, reply_only: bool) -> Self {
        Self {
            thread_id,
            reply_only,
        }
    }

    fn get_thread_id(&self) -> String {
        self.thread_id.clone()
    }

    fn reply_only(&self) -> bool {
        self.reply_only
    }
}
