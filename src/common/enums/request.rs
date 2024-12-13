pub enum Request {
    Authorization(String),
    GetConversationHistory(String),
    GetConversationReplies,
}
