use crate::models::{Reference, Schedule, User};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OnCall {
    pub user: Reference<User>,
    pub schedule: Option<Reference<Schedule>>,
    pub escalation_level: u32,
    // pub escalation_policy: Reference<EscalationPolicy>,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
