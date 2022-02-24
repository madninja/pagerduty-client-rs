mod escalation_policy;
mod oncall;
mod reference;
mod schedule;
mod service;
#[cfg(feature = "team")]
#[cfg_attr(docsrs, doc(cfg(feature = "team")))]
mod team;
mod user;

pub use escalation_policy::{EscalationPolicy, EscalationRule};
pub use oncall::OnCall;
pub use reference::{Reference, ReferenceType};
pub use schedule::Schedule;
pub use service::Service;
#[cfg(feature = "team")]
#[cfg_attr(docsrs, doc(cfg(feature = "team")))]
pub use team::Team;
pub use user::User;
