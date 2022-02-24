use crate::{
    models::{EventType, Webhook},
    Client, Result, NO_QUERY,
};
use http::Uri;
use serde::{Deserialize, Serialize};

/// Get all users as an automatically paged Stream.
///
/// Note, do not pass `limit` or `skip` parameters since they are used
/// internally for paging.
///
/// See: [List
/// Webhooks](https://bonusly.docs.apiary.io/#reference/0/webhooks)
pub async fn all(client: &Client) -> Result<Vec<Webhook>> {
    client.get("/webhooks", NO_QUERY).await
}

/// Configuration for creating or updating a webhook
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookConfig {
    /// The webhook URL to use. Note that the domain for this URL is validated
    /// by the API.
    #[serde(with = "http_serde::uri")]
    pub url: Uri,
    /// Optional event types for the webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<EventType>>,
}

#[derive(Debug, Clone, Deserialize)]
struct IdResult {
    id: String,
}

impl From<IdResult> for String {
    fn from(v: IdResult) -> Self {
        v.id
    }
}

/// Create a webhook using a given config
///
/// See: [Create a
/// Webhook](https://bonusly.docs.apiary.io/#reference/0/webhooks/create-webhook)
pub async fn create(client: &Client, config: &WebhookConfig) -> Result<String> {
    let result = client.post::<_, IdResult>("/webhooks", config).await?;
    Ok(result.into())
}

/// Update a given webhook. Returns the id of the given webhook
///
/// See: [Create a
/// Webhook](https://bonusly.docs.apiary.io/#reference/0/webhooks/update-webhook)
pub async fn update(client: &Client, id: &str, config: &WebhookConfig) -> Result<String> {
    let result = client
        .put::<_, IdResult>(&format!("/webhooks/{}", id), config)
        .await?;
    Ok(result.into())
}

/// Delete a given webhook. Returns the id of the deleted webhook
///
/// See: [Delete a
/// Webhook](https://bonusly.docs.apiary.io/#reference/0/webhooks/remove-webhook)
pub async fn delete(client: &Client, id: &str) -> Result<String> {
    let result = client
        .delete::<IdResult>(&format!("/webhooks/{}", id))
        .await?;
    Ok(result.id)
}

#[cfg(test)]
mod test {
    use crate::{
        env_var,
        webhooks::{self, EventType, WebhookConfig},
        Client,
    };
    use http::Uri;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let _ = webhooks::all(&client).await.expect("webhooks");
    }

    #[test]
    async fn crud() {
        let client = Client::from_env("test.env").expect("client");
        let url = env_var::<Uri>("BONUSLY_TEST_WEBHOOK_URL").expect("test webhook url");

        let config = WebhookConfig {
            url,
            event_types: Some(vec![EventType::BonusCreated]),
        };

        let webhook_id = webhooks::create(&client, &config)
            .await
            .expect("created webhook");

        let mut updated_config = config.clone();
        updated_config.event_types = Some(vec![EventType::BonusCreated]);
        let updated_id = webhooks::update(&client, &webhook_id, &updated_config)
            .await
            .expect("updated webhook");
        assert_eq!(updated_id, webhook_id);

        webhooks::delete(&client, &webhook_id)
            .await
            .expect("deleted webhook");
    }
}
