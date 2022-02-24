use chrono::{DateTime, Utc};
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: Option<String>,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub created_at: DateTime<Utc>,
    pub status: String,
}
