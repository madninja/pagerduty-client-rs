use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub name: String,
    pub description: Option<String>,
}
