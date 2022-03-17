use async_trait::async_trait;
use futures::{
    future, stream, Future as StdFuture, FutureExt, Stream as StdStream, StreamExt, TryFutureExt,
};
use reqwest::{self, header, Method};
use serde::{de::DeserializeOwned, ser::Serialize};
use std::{collections::HashMap, env, pin::Pin, time::Duration};

mod result;
pub use result::{Error, ResponseError, Result};

pub mod escalation_policies;
pub mod models;
pub mod oncalls;
pub mod schedules;
pub mod services;
pub mod tags;
pub mod users;

/// A type alias for `Future` that may return `crate::error::Error`
pub type Future<T> = Pin<Box<dyn StdFuture<Output = Result<T>> + Send>>;

/// A type alias for `Stream` that may result in `crate::error::Error`
pub type Stream<T> = Pin<Box<dyn StdStream<Item = Result<T>> + Send>>;
pub use models::{BaseModel, Reference};

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
pub const BASE_URL: &str = "https://api.pagerduty.com";
pub const PAGE_SIZE: usize = 20;

/// A utility constant to pass an empty query slice to the various client fetch
/// functions
pub const NO_QUERY: &[&str; 0] = &[""; 0];

#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::from_dotenv().expect("client")
    }
}

impl Client {
    pub fn from_dotenv() -> Result<Self> {
        dotenv::dotenv().ok();
        Ok(Self::new(&token_from_env()?))
    }

    pub fn from_env(filename: &str) -> Result<Self> {
        dotenv::from_filename(filename).ok();
        Ok(Self::new(&token_from_env()?))
    }

    /// Create a new bonus.ly client using a given access token
    pub fn new(token: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        let mut token_value = header::HeaderValue::from_str(&format!("Token token={}", token))
            .expect("valid bearer token");
        token_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, token_value);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .gzip(true)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("reqwest client");
        Self {
            base_url: BASE_URL.to_owned(),
            client,
        }
    }

    fn _get<T, Q, V>(&self, key: &'static str, path: &str, query: &Q, add_query: &V) -> Future<T>
    where
        T: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,
        Q: Serialize + ?Sized,
        V: Serialize + ?Sized,
    {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .get(&request_url)
            .query(query)
            .query(add_query)
            .send()
            .map_err(Error::from)
            .and_then(move |result| extract_result(key, result))
            .boxed()
    }

    pub(crate) fn get<T, Q>(&self, key: &'static str, path: &str, query: &Q) -> Future<T>
    where
        T: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,
        Q: Serialize + ?Sized + std::marker::Sync,
    {
        self._get(key, path, query, NO_QUERY)
    }

    pub(crate) fn get_stream<E, Q>(
        &self,
        key: &'static str,
        path: &str,
        limit: usize,
        query: &'static Q,
    ) -> Stream<E>
    where
        E: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,

        Q: Serialize + ?Sized + std::marker::Sync,
    {
        let path = path.to_string();
        let client = self.clone();
        client
            ._get::<Vec<E>, _, _>(key, &path.clone(), query, &[("limit", limit)])
            .map_ok(move |mut data| {
                data.reverse();
                let data_len = data.len();
                let path = path.to_string();
                stream::try_unfold(
                    (data, key, path, client, data_len),
                    move |(mut data, key, path, client, offset)| async move {
                        match data.pop() {
                            Some(entry) => Ok(Some((entry, (data, key, path, client, offset)))),
                            None => {
                                //loop until we find next bit of data or run
                                // out of cursors
                                let mut data: Vec<E> = client
                                    ._get::<Vec<E>, _, _>(
                                        key,
                                        &path,
                                        query,
                                        &[("offset", offset), ("limit", limit)],
                                    )
                                    .await?;
                                if !data.is_empty() {
                                    data.reverse();
                                    let data_len = data.len();
                                    let entry = data.pop().unwrap();
                                    Ok(Some((entry, (data, key, path, client, offset + data_len))))
                                } else {
                                    Ok(None)
                                }
                            }
                        }
                    },
                )
            })
            .try_flatten_stream()
            .boxed()
    }

    // pub(crate) fn put<T, R>(&self, path: &str, json: &T) -> Future<R>
    // where
    //     T: Serialize + ?Sized,
    //     R: 'static + DeserializeOwned + std::marker::Send,
    // {
    //     self.submit(Method::PUT, path, json)
    // }

    pub(crate) fn post<T, R>(&self, key: &'static str, path: &str, json: &T) -> Future<R>
    where
        T: Serialize + ?Sized,
        R: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,
    {
        self.submit(Method::POST, key, path, json)
    }

    fn submit<T, R>(&self, method: Method, key: &'static str, path: &str, json: &T) -> Future<R>
    where
        T: Serialize + ?Sized,
        R: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,
    {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .request(method, &request_url)
            .json(json)
            .send()
            .map_err(Error::from)
            .and_then(move |response| extract_result(key, response))
            .boxed()

        //     match response.error_for_status() {
        //     Ok(result) => result
        //         .json::<HashMap<String, serde_json::Value>>()
        //         .map_err(Error::from)
        //         .and_then(move |mut json| async move {
        //             json.remove(key)
        //                 .ok_or_else(|| {
        //                     Error::api_error(format!("key \"{}\" not found in result", key))
        //                 })
        //                 .and_then(|value| {
        //                     serde_json::from_value::<R>(value).map_err(Error::from)
        //                 })
        //         })
        //         .boxed(),
        //     Err(e) => future::err(Error::response(e, response)).boxed(),
        // })
        // .boxed()
    }

    pub(crate) fn delete(&self, path: &str) -> Future<()> {
        let request_url = format!("{}{}", self.base_url, path);
        self.client
            .delete(&request_url)
            .send()
            .map_err(Error::from)
            .and_then(|response| match response.error_for_status() {
                Ok(_result) => future::ok(()),
                Err(e) => future::err(Error::from(e)),
            })
            .boxed()
    }
}

fn token_from_env() -> Result<String> {
    env_var("PAGERDUTY_TOKEN")
}

pub(crate) fn env_var<T: std::str::FromStr>(name: &str) -> Result<T> {
    env::var(name)
        .map_err(|_| Error::custom(format!("Missing env var: {}", name)))
        .and_then(|var| {
            var.parse::<T>()
                .map_err(|_| Error::custom(format!("Error parsing {}", name)))
        })
}

pub use pagerduty_macros::Dereference;

#[async_trait]
pub trait Dereference {
    type Output;
    async fn dereference(&self, client: &Client) -> Result<Self::Output>;
}

fn extract_result<T>(key: &'static str, result: reqwest::Response) -> Future<T>
where
    T: 'static + DeserializeOwned + std::marker::Send + std::fmt::Debug,
{
    let error_status = result.error_for_status_ref().is_ok();
    result
        .json::<HashMap<String, serde_json::Value>>()
        .map_err(Error::from)
        .and_then(move |mut json| async move {
            match error_status {
                true => json
                    .remove(key)
                    .ok_or_else(|| Error::api_error(format!("key \"{key}\" not found in result")))
                    .and_then(|value| serde_json::from_value::<T>(value).map_err(Error::from)),
                false => json
                    .remove("error")
                    .ok_or_else(|| {
                        Error::api_error("key \"error\" not found in result".to_string())
                    })
                    .and_then(|value| Err(Error::from_json_error(value))),
            }
        })
        .boxed()
}
