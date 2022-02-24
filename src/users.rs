use crate::{models::User, Client, Result, Stream, NO_QUERY};
use serde::Serialize;

/// Get all users as an automatically paged Stream.
///
/// Note, do not pass `limit` or `offset` parameters since they are used
/// internally for paging.
///
/// See: [List
/// Users](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIzMw-list-users)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<User>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("users", "/users", page_size, params)
}

/// Get a specific user by their id
///
/// See: [Get a
/// User](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIzNQ-get-a-user)
pub async fn get(client: &Client, id: &str) -> Result<User> {
    client.get("user", &format!("/users/{}", id), NO_QUERY).await
}

#[cfg(test)]
mod test {
    use crate::{env_var, users, Client, IntoVec, StreamExt, NO_QUERY};
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let users = users::all(&client, 10, NO_QUERY)
            .take(10)
            .into_vec()
            .await
            .expect("users");
        assert_eq!(users.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::from_env("test.env").expect("client");
        let _ = users::get(
            &client,
            &env_var::<String>("PAGERDUTY_TEST_USER").expect("test user id"),
        )
        .await
        .expect("user");
    }
}
