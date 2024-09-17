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
