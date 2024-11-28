use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authorization {
    pub ok: bool,
    pub app_id: String,
    pub authed_user: AuthedUser,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthedUser {
    pub id: String,
    pub scope: String,
    pub access_token: String,
    pub token_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
}
