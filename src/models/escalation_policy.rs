use crate::{
    models::{BaseModel, Schedule, Service, User},
    Dereference, Reference,
};
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, BaseModel, Dereference)]
#[dereference(client = "escalation_policies")]
#[serde(tag = "type")]
pub enum EscalationPolicy {
    #[serde(rename = "escalation_policy_reference")]
    Reference(Reference),
    #[serde(rename = "escalation_policy")]
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
    pub num_loops: u32,
    pub on_call_handoff_notifications: String,
    pub services: Vec<Service>,
    pub escalation_rules: Vec<EscalationRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EscalationRule {
    pub id: String,
    pub escalation_delay_in_minutes: u32,
    pub targets: Vec<EscalationTarget>,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum EscalationTarget {
    User(User),
    Scedule(Schedule),
}

impl Model {
    pub fn contains_service(&self, id: &str) -> bool {
        self.services.iter().any(|entry| entry.id() == id)
    }
}
