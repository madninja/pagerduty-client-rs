use crate::{models::OnCall, Client, Stream};
use serde::Serialize;

/// Get all oncalls as an automatically paged Stream.
///
/// Note, do not pass `limit` or `offset` parameters since they are used
/// internally for paging.
///
/// See: [List
/// OnCalls](https://developer.pagerduty.com/api-reference/b3A6Mjc0ODE2Mw-list-all-of-the-on-calls)
pub fn all<Q>(client: &Client, page_size: usize, params: &'static Q) -> Stream<OnCall>
where
    Q: Serialize + ?Sized + std::marker::Sync,
{
    client.get_stream("oncalls", "/oncalls", page_size, params)
}

#[cfg(test)]
mod test {
    use crate::{models, oncalls, Client, StreamExt, TryStreamExt, NO_QUERY};
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::from_env("test.env").expect("client");
        let oncalls: Vec<models::OnCall> = oncalls::all(&client, 10, NO_QUERY)
            .take(10)
            .try_collect()
            .await
            .expect("oncalls");
        assert!(oncalls.len() > 0);
    }
}
