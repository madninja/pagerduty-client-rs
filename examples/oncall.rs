use futures::{TryFutureExt, TryStreamExt};
use pagerduty_client::{
    models::{OnCall, ServiceModel, User},
    oncalls, services, BaseModel, Client, Dereference, Result, NO_QUERY,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result {
    let client = Client::from_dotenv()?;
    let mut services: HashMap<ServiceModel, Vec<User>> = services::all(&client, 10, NO_QUERY)
        .map_ok(|service| (service, vec![]))
        .try_collect::<Vec<(_, _)>>()
        .map_ok(HashMap::from_iter)
        .await?;
    let mut oncalls: Vec<OnCall> =
        oncalls::all(&client, 10, &[("include[]", "escalation_policies")])
            .try_collect::<Vec<_>>()
            .await?;
    oncalls.sort_unstable_by_key(|oncall| oncall.escalation_level);

    for (service, users) in services.iter_mut() {
        *users = find_oncall(&client, service, &oncalls).await?;
    }

    for (service, users) in services {
        let names: Vec<&str> = users.iter().map(|user| user.summary()).collect();
        println!("{:.<40}{}", service.summary(), names.join(", "));
    }

    Ok(())
}

async fn find_oncall(
    client: &Client,
    service: &ServiceModel,
    oncalls: &[OnCall],
) -> Result<Vec<User>> {
    let mut result = vec![];
    for oncall in oncalls {
        let policy = oncall.escalation_policy.dereference(client).await?;
        if policy.contains_service(&service.id) {
            result.push(oncall.user.clone())
        }
    }
    Ok(result)
}
