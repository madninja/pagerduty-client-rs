use crate::{BaseModel, Dereference, Reference};
use chrono::{DateTime, Utc};
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BaseModel, Dereference)]
#[dereference(client = "services")]
#[serde(tag = "type")]
pub enum Service {
    #[serde(rename = "service_reference")]
    Reference(Reference),
    #[serde(rename = "service")]
    Model(Model),
}

#[derive(Debug, Clone, Deserialize, BaseModel)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: Option<String>,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub created_at: DateTime<Utc>,
    pub status: String,
}
