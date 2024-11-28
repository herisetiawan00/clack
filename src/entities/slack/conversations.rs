use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub ok: bool,
    pub channels: Vec<Channel>,
    pub response_metadata: ResponseMetadata,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Channel {
    pub id: String,
    pub name: Option<String>,
    pub is_channel: Option<bool>,
    pub is_group: Option<bool>,
    pub is_im: bool,
    pub created: i64,
    pub creator: Option<String>,
    pub is_archived: Option<bool>,
    pub is_general: Option<bool>,
    pub unlinked: Option<f32>,
    pub name_normalized: Option<String>,
    pub is_shared: Option<bool>,
    pub is_ext_shared: Option<bool>,
    pub is_org_shared: bool,
    pub pending_shared: Option<Vec<String>>,
    pub is_pending_ext_shared: Option<bool>,
    pub is_member: Option<bool>,
    pub is_private: Option<bool>,
    pub is_mpim: Option<bool>,
    pub is_open: Option<bool>,
    pub updated: Option<i64>,
    pub topic: Option<TopicPurpose>,
    pub purpose: Option<TopicPurpose>,
    pub priority: Option<f32>,
    pub user: Option<String>,
    pub is_user_deleted: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct TopicPurpose {
    pub value: String,
    pub creator: String,
    pub last_set: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}
