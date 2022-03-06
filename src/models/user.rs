use crate::{BaseModel, Dereference, Reference};
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BaseModel, Dereference)]
#[dereference(client = "users")]
#[serde(tag = "type")]
pub enum User {
    #[serde(rename = "user_reference")]
    Reference(Reference),
    #[serde(rename = "user")]
    Model(Model),
}

#[derive(Debug, Clone, Deserialize, BaseModel)]
pub struct Model {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub name: String,
    pub email: String,
    pub time_zone: String,
}
