#[cfg(feature = "team")]
#[cfg_attr(docsrs, doc(cfg(feature = "team")))]
use crate::models::Team;
use crate::models::{Reference, Service, User};
use http::Uri;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct EscalationPolicy {
    pub id: String,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,
    pub name: String,
    pub description: Option<String>,
    pub num_loops: u32,
    pub on_call_handoff_notifications: String,
    pub services: Vec<Reference<Service>>,
    pub escalation_rules: Vec<EscalationRule>,
    #[cfg(feature = "team")]
    #[cfg_attr(docsrs, doc(cfg(feature = "team")))]
    pub teams: Vec<Reference<Team>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EscalationRule {
    pub id: String,
    pub escalation_delay_in_minutes: u32,
    pub targets: Vec<Reference<User>>,
}
