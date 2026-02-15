use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, reqwest};
use url::Url;

pub struct ApiClient {
    client: ClientWithMiddleware,
    base_url: Url,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = reqwest::Client::new();
    let client = ClientBuilder::new(client).build();
    let client = ApiClient::new(client, "http://httpbin.org".try_into().unwrap());
    let response: serde_json::Value =
        client.send_request(reqwest::Method::GET, "get", &None::<i32>).await.unwrap();
    println!("{response}");
}

impl ApiClient {
    pub fn new(client: ClientWithMiddleware, base_url: Url) -> Self {
        Self { client, base_url }
    }

    pub async fn send_request<Request, Response>(
        &self,
        method: reqwest::Method,
        url: &str,
        request: &Request,
    ) -> Result<Response, ApiClientError>
    where
        Request: serde::Serialize + ?Sized,
        Response: serde::de::DeserializeOwned,
    {
        let mut url = self.base_url.join(url)?;

        // Conditionally attach `request` as query params if GET.
        if method == reqwest::Method::GET {
            // Use `serde_html_form` crate instead of `serde_urlencoded` (baked in `reqwest`),
            // as it doesn't handle vec (`id=1&id=2`) and gives `unsupported value` error.
            let query_string = serde_html_form::to_string(request)?;
            if !query_string.is_empty() {
                url.set_query(Some(&query_string));
            }
        }

        let request_builder = self.client.request(method.clone(), url.clone());

        // For POST/PUT/etc, serialize `request` into the JSON body.
        let request_builder = if method != reqwest::Method::GET {
            request_builder.json(request)
        } else {
            request_builder
        };

        // Send the request.
        let response = request_builder.send().await?;

        // Handle any response error before deserialisation.
        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let err = ApiClientError::Api {
                code: status.to_string(),
                method: method.to_string(),
                url: url.to_string(),
                body: response.text().await.unwrap_or("no response body".into()),
            };
            return Err(err);
        }

        // Get the raw bytes first.
        // This is very cheap and rarely fails (unless the connection drops).
        let body_bytes =
            response.bytes().await.map_err(|err| reqwest_middleware::Error::Reqwest(err))?;

        // Create a standard JSON deserializer from the bytes.
        let mut deserializer = serde_json::Deserializer::from_slice(&body_bytes);

        // Wrap it with path tracking and deserialize.
        let response = serde_path_to_error::deserialize(&mut deserializer).map_err(|cause| {
            // We only convert bytes -> String IF there is an error.
            // Use 'from_utf8_lossy' to safely handle the body even if it contains invalid characters or binary data.
            let body_text = String::from_utf8_lossy(&body_bytes).to_string();

            // Log error with actual response body text, so we can debug it later.
            tracing::error!(%method, %url, %body_text, %cause, "Failed to parse response body");

            let err = ApiClientError::Deserialization {
                method: method.to_string(),
                url: url.to_string(),
                err: cause,
            };

            err
        })?;

        Ok(response)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Network error: {0}")]
    Network(#[from] reqwest_middleware::Error),

    #[error("Failed to serialize query: {0}")]
    QuerySerialization(#[from] serde_html_form::ser::Error),

    #[error("API error ({code}) at {method} {url}: {body}")]
    Api { code: String, method: String, url: String, body: String },

    #[error("Failed to deserialize response at {method} {url}: {err}")]
    Deserialization {
        method: String,
        url: String,
        err: serde_path_to_error::Error<serde_json::Error>,
    },
}
