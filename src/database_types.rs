use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    pub workspace_id: Uuid,
    pub workspace_name: String,
}

impl Display for Workspace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.workspace_name)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    pub channel_id: Uuid,
    pub channel_name: String,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.channel_name)
    }
}

#[derive(Debug, Clone)]
pub struct UserMessage {
    pub user_id: Uuid,
    pub user_name: String,
    pub content: String,
}
