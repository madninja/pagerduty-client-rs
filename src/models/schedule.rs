use crate::models::{Reference, User};
use chrono_tz::Tz;
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Schedule {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub name: String,
    pub description: Option<String>,
    pub time_zone: Tz,
    pub users: Vec<Reference<User>>,
}
