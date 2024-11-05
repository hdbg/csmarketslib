use std::{borrow::Cow, num::NonZeroU32};

use eyre::ContextCompat;
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter,
};
use http::header::CONTENT_TYPE;
use reqwest::Request;
use serde::de::DeserializeOwned;

pub mod typedefs;

pub mod endpoints;

#[derive(serde::Serialize, serde::Deserialize)]
struct ApiResponse<T> {
    pub success: bool,
    pub error: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

static TM_SEC_QUOTA: u32 = 5;
static API_BASE: &str = "https://market.csgo.com/api/v2";

pub struct TMClient {
    client: reqwest::Client,
    key: String,
    ratelimiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
}

impl TMClient {
    pub fn new(key: String) -> Self {
        let ratelimiter =
            RateLimiter::direct(Quota::per_second(NonZeroU32::new(TM_SEC_QUOTA).unwrap()));
        Self {
            client: reqwest::Client::new(),
            key,
            ratelimiter,
        }
    }
    pub async fn exec<E: Endpoint>(&self, endpoint: E) -> eyre::Result<E::Data> {
        self.ratelimiter.until_ready().await;
        let url = format!("{}{}", API_BASE, endpoint.endpoint());

        let request: reqwest::RequestBuilder = self.client.request(E::METHOD.into(), url);
        let mut request = request.query(endpoint.params().as_slice());

        if let Some(Body { data, content_type }) = endpoint.body() {
            request = request.header(CONTENT_TYPE, content_type).body(data);
        }

        if E::REQUIRES_API_KEY {
            request = request.query(&[("key", self.key.as_str())]);
        }

        let request = request.build()?;

        let resp = self.client.execute(request).await?.error_for_status()?;

        let data: ApiResponse<E::Data> = resp.json().await?;

        if !data.success {
            eyre::bail!(data
                .error
                .unwrap_or("TM returned success = false, but haven't specified error".to_owned()));
        }

        data.data.context("data was empty")
    }
}
