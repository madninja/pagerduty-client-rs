use crate::{models::User, BaseModel, Dereference, Reference};
use chrono_tz::Tz;
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BaseModel, Dereference)]
#[dereference(client = "schedules")]
#[serde(tag = "type")]
pub enum Schedule {
    #[serde(rename = "schedule_reference")]
    Reference(Reference),
    #[serde(rename = "schedule")]
    Model(Model),
}

#[derive(Debug, Clone, Deserialize, BaseModel)]
pub struct Model {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub name: String,
    pub description: Option<String>,
    pub time_zone: Tz,
    pub users: Vec<User>,
}
