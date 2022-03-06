use crate::{models::EscalationPolicyModel, Client, Result, Stream, NO_QUERY};
use serde::Serialize;

/// Get a service by its id
///
/// See: [Get an
/// Escalation Policy](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODEyNg-get-an-escalation-policy)
pub async fn get(client: &Client, id: &str) -> Result<EscalationPolicyModel> {
    client
        .get(
            "escalation_policy",
            &format!("/escalation_policies/{}", id),
            NO_QUERY,
        )
        .await
}

/// Get a stream of all known escalation policoes
///
/// See: [List
/// Escalation Policies](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODEyNA-list-escalation-policies)
pub fn all<Q>(
    client: &Client,
    page_size: usize,
    params: &'static Q,
) -> Stream<EscalationPolicyModel>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream(
        "escalation_policies",
        "/escalation_policies",
        page_size,
        params,
    )
}

#[cfg(test)]
mod test {
    use crate::{
        env_var, escalation_policies, models::EscalationPolicy, Client, TryStreamExt, NO_QUERY,
    };
    use tokio::test;

    #[test]
    async fn get() {
        let client = Client::from_env("test.env").expect("client");
        let policy_id =
            env_var::<String>("PAGERDUTY_TEST_ESCALATION_POLICY").expect("test service");
        let policy = escalation_policies::get(&client, &policy_id)
            .await
            .expect("escalation_policy");

        let id = match policy {
            EscalationPolicy::Model(m) => m.id,
            EscalationPolicy::Reference(r) => r.id,
        };
        assert_eq!(policy_id, id);
    }

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let policies: Vec<EscalationPolicy> = escalation_policies::all(&client, 2, NO_QUERY)
            .try_collect()
            .await
            .expect("escalation policies");

        assert!(policies.len() > 0);
    }
}
