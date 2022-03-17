pub mod escalation_policy;
pub mod oncall;
pub mod reference;
pub mod schedule;
pub mod service;
pub mod tag;
pub mod user;
pub mod webhook;

pub use escalation_policy::{EscalationPolicy, EscalationRule, Model as EscalationPolicyModel};
pub use oncall::OnCall;
pub use pagerduty_macros::BaseModel;
pub use reference::Reference;
pub use schedule::{Model as ScheduleModel, Schedule};
pub use service::{Model as ServiceModel, Service};
pub use tag::Tag;
pub use user::{Model as UserModel, User};
pub use webhook::Webhook;

use http::Uri;

pub trait BaseModel {
    fn id(&self) -> &str;
    fn summary(&self) -> &str;
    fn html_url(&self) -> &Uri;
}

impl PartialEq for dyn BaseModel {
    fn eq(&self, other: &(dyn BaseModel)) -> bool {
        self.id() == other.id()
    }
}
