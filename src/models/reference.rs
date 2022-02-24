use http::Uri;
use serde::Deserialize;
use std::marker::PhantomData;

#[derive(Debug, Clone, Deserialize)]
pub struct Reference<T> {
    pub id: String,
    pub r#type: ReferenceType,
    pub summary: String,
    #[serde(with = "http_serde::uri")]
    pub html_url: Uri,

    #[serde(skip)]
    phantom: PhantomData<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ReferenceType {
    #[serde(rename = "user_reference")]
    User,
    #[serde(rename = "schedule_reference")]
    Schedule,
    #[serde(rename = "service_reference")]
    Service,
    #[serde(rename = "escalation_policy_reference")]
    EscalationPolicy,
}
