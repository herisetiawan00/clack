use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub ok: bool,
    pub messages: Vec<Message>,
    //pub response_metadata: ResponseMetadata,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub user: Option<String>,
    pub bot_id: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub ts: String,
    pub client_msg_id: Option<String>,
    pub text: Option<String>,
    pub team: Option<String>,
    pub blocks: Option<Vec<Block>>,
    pub thread_ts: Option<String>,
    pub reply_count: Option<u32>,
    pub reply_users_count: Option<u32>,
    pub latest_reply: Option<String>,
    pub reply_users: Option<Vec<String>>,
    pub is_locked: Option<bool>,
    pub subscribed: Option<bool>,
    pub edited: Option<Edited>,
    pub last_read: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Block {
    #[serde(rename = "type")]
    pub block_type: String,
    pub block_id: Option<String>,
    pub elements: Option<Vec<Element>>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Element {
    #[serde(rename = "type")]
    pub element_type: String,
    pub elements: Option<Vec<Element>>, // Nested elements
    pub text: Option<String>,
    pub user_id: Option<String>,
    pub emoji: Option<String>,
    pub unicode: Option<String>,
    pub range: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Edited {
    pub user: String,
    pub ts: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Reaction {
    pub name: String,
    pub users: Vec<String>,
    pub count: u32,
}
