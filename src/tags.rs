use crate::{models::Tag, Client, Result, Stream, NO_QUERY};
use serde::Serialize;
use serde_json::json;

/// Get all tags as an automatically paged Stream.
///
/// Note, do not pass `limit` or `offset` parameters since they are used
/// internally for paging.
///
/// See: [List
/// Tags](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIxNw-list-tags)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<Tag>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("tags", "/tags", page_size, params)
}

/// Get a specific tag by id
///
/// See: [Get a
/// Tag](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIxOQ-get-a-tag)
pub async fn get(client: &Client, id: &str) -> Result<Tag> {
    client.get("tag", &format!("/tags/{}", id), NO_QUERY).await
}

/// Create a tag with a given label
///
/// See: [Create a
/// Tag](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIxOA-create-a-tag)
pub async fn create(client: &Client, label: &str) -> Result<Tag> {
    client
        .post(
            "tag",
            "/tags",
            &json!({
                "tag": {
                    "type":  "tag",
                    "label": label,
                }
            }),
        )
        .await
}

/// Delete a tag given its id
///
/// See: [Delete a
/// Tag](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIyMA-delete-a-tag)
pub async fn delete(client: &Client, id: &str) -> Result {
    client.delete(&format!("/tags/{}", id)).await
}

#[cfg(test)]
mod test {
    use crate::{models, tags, Client, NO_QUERY};
    use futures::TryStreamExt;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let _tags: Vec<models::Tag> = tags::all(&client, 10, NO_QUERY)
            .try_collect()
            .await
            .expect("tags");
    }

    #[test]
    async fn create_delete() {
        let client = Client::from_env("test.env").expect("client");
        let tag = tags::create(&client, &"test_tag").await.expect("a new tag");
        tags::delete(&client, &tag.id).await.expect("ok delete")
    }
}
