use crate::{models::Team, Client, Result, Stream, NO_QUERY};
use serde::Serialize;

/// Get all teams as an automatically paged Stream.
///
/// Note, do not pass `limit` or `offset` parameters since they are used
/// internally for paging.
///
/// See: [List
/// Teams](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIyMw-list-teams)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<Team>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("teams", "/teams", page_size, params)
}

/// Get a specific team by their id
///
/// See: [Get a
/// Team](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODIyNA-get-a-team)
pub async fn get(client: &Client, id: &str) -> Result<Team> {
    client
        .get("team", &format!("/teams/{}", id), NO_QUERY)
        .await
}

#[cfg(test)]
mod test {
    use crate::{env_var, teams, Client, IntoVec, StreamExt, NO_QUERY};
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let teams = users::all(&client, 10, NO_QUERY)
            .take(10)
            .into_vec()
            .await
            .expect("teams");
        assert!(teams.len() >= 0);
    }

    #[test]
    async fn get() {
        let client = Client::from_env("test.env").expect("client");
        let _ = teams::get(
            &client,
            &env_var::<String>("PAGERDUTY_TEST_TEAM").expect("test team id"),
        )
        .await
        .expect("team");
    }
}
