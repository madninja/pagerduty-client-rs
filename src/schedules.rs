use crate::{models::Schedule, Client, Result, Stream, NO_QUERY};
use serde::Serialize;

/// Get all schedules as an automatically paged Stream.
///
/// Note, do not pass `limit` or `offset` parameters since they are used
/// internally for paging.
///
/// See: [List
/// Shedules](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODE4MQ-list-schedules)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<Schedule>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("schedules", "/schedules", page_size, params)
}

/// Get a specific team by their id
///
/// See: [Get a
/// Schedule](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODE4Mw-get-a-schedule)
pub async fn get(client: &Client, id: &str) -> Result<Schedule> {
    client
        .get("schedule", &format!("/schedules/{}", id), NO_QUERY)
        .await
}

#[cfg(test)]
mod test {
    use crate::{env_var, schedules, Client, Dereference, IntoVec, StreamExt, NO_QUERY};
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let schedules = schedules::all(&client, 10, NO_QUERY)
            .take(10)
            .into_vec()
            .await
            .expect("schedules");
        assert!(schedules.len() > 0);
    }

    #[test]
    async fn get() {
        let client = Client::from_env("test.env").expect("client");
        let schedule = schedules::get(
            &client,
            &env_var::<String>("PAGERDUTY_TEST_SCHEDULE").expect("test schedule id"),
        )
        .await
        .expect("schedule");

        assert!(schedule.users.len() > 0);

        let users = schedule
            .users
            .dereference(&client)
            .await
            .expect("schedule users");
        assert_eq!(users.len(), schedule.users.len());
    }
}
