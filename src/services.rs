use crate::{models::ServiceModel, Client, Result, Stream, NO_QUERY};
use serde::Serialize;

/// Get a service by its id
///
/// See: [Get a
/// Service](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODE5OA-get-a-service)
pub async fn get(client: &Client, id: &str) -> Result<ServiceModel> {
    client
        .get("service", &format!("/services/{}", id), NO_QUERY)
        .await
}

/// Get a stream of all known services
///
/// See: [List
/// Services](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODE5Ng-list-services)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<ServiceModel>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("services", "/services", page_size, params)
}

#[cfg(test)]
mod test {
    use crate::{env_var, models, services, BaseModel, Client, NO_QUERY};
    use futures::TryStreamExt;
    use tokio::test;

    #[test]
    async fn get() {
        let client = Client::from_env("test.env").expect("client");
        let service_id = env_var::<String>("PAGERDUTY_TEST_SERVICE").expect("test service");
        let service = services::get(&client, &service_id).await.expect("service");

        assert_eq!(service_id, service.id());
    }

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let services: Vec<models::ServiceModel> = services::all(&client, 2, NO_QUERY)
            .try_collect()
            .await
            .expect("services");

        assert!(services.len() > 0);
    }
}
