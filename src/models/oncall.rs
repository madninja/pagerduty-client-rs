use crate::models::{EscalationPolicy, Schedule, User};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct OnCall {
    pub user: User,
    pub schedule: Option<Schedule>,
    pub escalation_level: u32,
    pub escalation_policy: EscalationPolicy,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}
