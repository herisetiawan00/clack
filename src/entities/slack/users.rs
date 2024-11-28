use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ApiResponse {
    pub ok: bool,
    pub members: Vec<Member>,
    pub response_metadata: ResponseMetadata,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Member {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub deleted: bool,
    pub color: Option<String>,
    pub real_name: Option<String>,
    //pub tz: Option<String>,
    //pub tz_label: Option<String>,
    //pub tz_offset: Option<i64>,
    pub profile: MemberProfile,
    pub is_admin: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub is_bot: bool,
    pub is_app_user: bool,
    pub updated: Option<i64>,
    pub is_email_confirmed: Option<bool>,
    pub who_can_share_contact_card: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MemberProfile {
    pub title: Option<String>,
    pub phone: Option<String>,
    pub skype: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    //pub fields: Option<serde_json::Value>,
    pub status_text: Option<String>,
    //pub status_emoji: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseMetadata {
    pub next_cursor: String,
}
