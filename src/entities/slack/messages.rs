use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub ok: bool,
    pub messages: Vec<Message>,
    //pub response_metadata: ResponseMetadata,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub user: Option<String>,
    pub bot_id: Option<String>,
    pub ts: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}

pub struct Block {}

pub struct Element {}

pub struct ElementItem {}
