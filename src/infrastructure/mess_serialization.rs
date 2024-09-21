use serde::{ser::SerializeStruct, Serialize};
use serde_json::json;

use crate::Message;

/// `Message` 構造体をwebApi呼ぶときのbodyに入れたい
///
/// しかし、これは低レイヤーで定義しているため
/// `MessageWrapper` (eq: `Self`) に対してbodyに埋め込める形式にシリアライズする機能を追加
#[derive(Debug, Clone)]
pub struct MessageWrapper(Message);

impl Serialize for MessageWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // thread_idがNoneのとき 2 でもいいっぽい
        let mut state = serializer.serialize_struct("MessageWrapper", 2)?;
        state.serialize_field("text", &self.0.get_text())?;

        if let Some(thread_id) = self.0.get_thread_id() {
            state.serialize_field("thread", &json!({"name": thread_id}))?;
        }

        state.end()
    }
}

impl MessageWrapper {
    /// 安全にシリアライズする
    pub fn serialize_safe(&self) -> String {
        // 失敗しないのでpanicは考慮しなくてよい
        serde_json::to_string(self)
            .expect("Failed to convert to string. This is unexpected.")
    }
}

impl From<Message> for MessageWrapper {
    fn from(value: Message) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::Reply;

    use super::*;

    #[test]
    fn test_for_message_wrapper_serialize_gives_valid_1() {
        let message = Message::new("text_foo".into(), None);
        assert_eq!(
            MessageWrapper::from(message).serialize_safe(),
            r#"{"text":"text_foo"}"#
        );
    }
    #[test]
    fn test_for_message_wrapper_serialize_gives_valid_2() {
        let message = Message::new(
            "text_foo".into(),
            Some(Reply::new("thread_bar".into(), true)),
        );
        assert_eq!(
            MessageWrapper::from(message).serialize_safe(),
            r#"{"text":"text_foo","thread":{"name":"thread_bar"}}"#,
        );
    }

    /// 特殊記号 `"` がエスケープされるかのテスト
    #[test]
    fn test_for_message_wrapper_serialize_gives_valid_3() {
        let message = Message::new(r#"text"foo"#.into(), None);
        assert_eq!(
            MessageWrapper::from(message).serialize_safe(),
            r#"{"text":"text\"foo"}"#
        );
    }
}
