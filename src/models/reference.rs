use crate::models::BaseModel;
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BaseModel)]
pub struct Reference {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
}
