use crate::prelude::*;
pub use common_core::prelude::http_client::*;

pub use reqwest::{
    // ClientBuilder,
    Proxy,
};

use std::time::Duration;



#[derive(Builder)]
#[builder(setter(into))]
// #[builder(setter(into, optional), build_fn(into))]
pub struct OnionClient {
    #[builder(default = "true")]
    zstd: bool,
    #[builder(default = "Some(\"socks5h://127.0.0.1:9050\".to_string())")]
    proxy: Option<String>,
    #[builder(default = "3")]
    retry: u32,
    #[builder(default = "60_000")]
    timeout: u64,
    #[builder(default = "600_000")]
    max_retry_interval: u64,
    #[builder(default = "None")]
    api_key: Option<String>,
}


impl OnionClient {
    pub fn try_new() -> AResult<ClientWithMiddleware> {
        let api_key = "eUBC6atrAXG9hXFvSvJCAhBJG3PuhpynKPvFpXQxSS54H2LuJvawmW8LvsngzeRcDQ6sqYK4dS9KFNFK732CMexgLRDAY5A72rAmHhqem5RkDcJh4jW6YT2e8ZmqHN4K".to_string();
        OnionClientBuilder::default()
            .proxy(None)
            .retry(0u32)
            .api_key(Some(api_key))
            // .proxy(Proxy::all("socks5://127.0.0.1:9050")?)
            .build()?
            .into()
    }
}


impl From<OnionClient> for AResult<ClientWithMiddleware> {
    fn from(config: OnionClient) -> Self {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                Duration::from_millis(config.timeout),
                Duration::from_millis(config.max_retry_interval)
            )
            .build_with_max_retries(config.retry);

        let mut builder = Client::builder()
            .zstd(config.zstd)
            .timeout(retry_policy.min_retry_interval);


        if let Some(proxy) = config.proxy {
            builder = builder.proxy(Proxy::all(proxy)?);
        }

        if let Some(api_key) = config.api_key {
            let mut headers = HeaderMap::new();
            let value = format!("Bearer {api_key}")
                .parse()?;
            headers.insert("Authorization", value);
            builder = builder.default_headers(headers);
        }

        let client = builder.build()
            .map_err(msg)?;
        let client_with_middleware = ClientBuilder::new(client)
            // Trace HTTP requests. See the tracing crate to make use of these traces.
            .with(TracingMiddleware::default())
            // Retry failed requests.
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Ok(client_with_middleware)
    }
}




// pub fn new_onion_client() -> Result<Client, reqwest::Error> {
//     reqwest::Client::builder()
//         .zstd(true)
//         .proxy(Proxy::all("socks5://127.0.0.1:9050")?)
//         .timeout(std::time::Duration::from_secs(60))
//         .build()
// }


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use tokio;

    const TEST_URL: &str = "http://z3fqaubcsexrbbd3esgy5k3xpglirueckbwb6ahe6qxifhpi3eckl5ad.onion/id_ed25519_sk.pub";

    #[tokio::test]
    async fn test_onion_client_default() {
        let client: AResult<ClientWithMiddleware> = OnionClientBuilder::default()
            // .proxy(None::<String>)
            .build().unwrap().into();
        // let client: Client = OnionClientBuilder::default().build().unwrap();
        // assert!(client.is_ok(), "Failed to build client: {:?}", client.err());

        let client = client.unwrap();
        let response = client.get(TEST_URL).send().await;

        match response {
            Ok(resp) => {
                assert_eq!(
                    resp.status(),
                    StatusCode::OK,
                    "Expected OK status, got {}",
                    resp.status()
                );
                let body = resp.text().await.unwrap_or_default();
                println!("{body}", );
                assert!(!body.is_empty(), "Response body is empty");
            }
            Err(e) => panic!("Request failed: {}", e),
        }
    }

    #[tokio::test]
    async fn test_onion_client_no_proxy() {
        // let client: Client = OnionClientBuilder::default()
        let client: AResult<ClientWithMiddleware> = OnionClientBuilder::default()
            .proxy(None)
            .build()
            .unwrap()
            .into();
        // .build().unwrap();
        assert!(client.is_ok(), "Failed to build client: {:?}", client.err());

        let client = client.unwrap();
        let response = client.get(TEST_URL).send().await;
        println!("{client:?}", );
        println!("{response:?}", );

        assert!(
            response.is_err(),
            "Expected request to fail without proxy, but it succeeded"
        );
    }

    #[tokio::test]
    async fn test_onion_client_custom_timeout() {
        let client: AResult<ClientWithMiddleware> = OnionClientBuilder::default()
            // let client: Client = OnionClientBuilder::default()
            .timeout(50_u64)
            // .timeout(50000_u64)
            // .build();
            .build()
            // .unwrap();
            .unwrap()
            .into();
        assert!(client.is_ok(), "Failed to build client: {:?}", client.err());

        let client = client.unwrap();
        let response = client.get(TEST_URL).send().await;
        println!("{client:?}", );
        println!("{response:?}", );
        match response {
            Ok(resp) => {
                assert_eq!(
                    resp.status(),
                    StatusCode::OK,
                    "Expected OK status, got {}",
                    resp.status()
                );
            }
            Err(e) => panic!("Request failed with short timeout: {}", e),
        }
    }
}
