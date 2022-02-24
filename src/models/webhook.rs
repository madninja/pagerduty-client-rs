use http::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EventType {
    #[serde(rename = "bonus.created")]
    BonusCreated,
    #[serde(rename = "achievement_event.created")]
    AchievementCreated,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    pub id: String,
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
    pub event_types: Vec<EventType>,
}
