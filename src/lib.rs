pub use error::*;
pub use files::*;
pub use types::*;

use tracing::{debug, error, trace};

mod error;
mod files;
mod types;

/// All of the requests to Telegram.
pub mod requests;

#[cfg(test)]
mod test;

static API_ENDPOINT: &str = "https://api.telegram.org/";

/// Type used for files in [TelegramRequest].
type RequestFiles = Option<Vec<(String, reqwest::multipart::Part)>>;

/// A trait for all Telegram requests.
///
/// It has as many default methods as possible but still requires some additions.
pub trait TelegramRequest: serde::Serialize + std::fmt::Debug {
    /// Response is the type used when Deserializing Telegram's result field.
    ///
    /// For convenience of debugging, it must implement [Debug](std::fmt::Debug).
    type Response: serde::de::DeserializeOwned + std::fmt::Debug;

    /// Endpoint to use for the request.
    fn endpoint(&self) -> &str;

    /// A JSON-compatible serialization of the data to send with the request.
    /// The default works for most methods.
    fn values(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    /// Files that are sent with the request.
    fn files(&self) -> RequestFiles {
        None
    }
}

/// A Telegram Bot API client.
///
/// Contains your bot token, the API endpoint, and a HTTP client.
pub struct Telegram {
    api_key: String,
    client: reqwest::Client,

    api_endpoint: String,
}

impl Telegram {
    /// Create a new Telegram instance with a specified API key.
    pub fn new(api_key: String) -> Self {
        Self::new_with_endpoint(api_key, API_ENDPOINT.into())
    }

    /// Create a new Telegram instance with a specified API key and API endpoint.
    ///
    /// The API endpoint should include the scheme, host, and a trailing slash.
    /// An example (and the default) is `https://api.telegram.org/`.
    pub fn new_with_endpoint(api_key: String, api_endpoint: String) -> Self {
        let client = reqwest::Client::builder().build().unwrap();

        Self {
            api_key,
            client,
            api_endpoint,
        }
    }

    /// Make a request for a [TelegramRequest] item and parse the response
    /// into the requested output type if the request succeeded.
    #[tracing::instrument(skip(self, request), fields(method = request.endpoint()))]
    pub async fn make_request<T>(&self, request: &T) -> Result<T::Response, Error>
    where
        T: TelegramRequest,
    {
        let endpoint = request.endpoint();

        let url = format!("{}bot{}/{}", self.api_endpoint, self.api_key, endpoint);
        let values = request.values()?;

        debug!("Making request with values: {:?}", values);

        let resp: types::Response<T::Response> = if let Some(files) = request.files() {
            // If our request has a file that needs to be uploaded, use
            // a multipart upload. Works by converting each JSON value into
            // a string and putting it into a field with the same name as the
            // original object.

            trace!("Request has files: {:?}", files);

            let mut form_values = serde_json::Map::new();
            form_values = values.as_object().unwrap_or(&form_values).clone();

            let form = form_values.iter().fold(
                reqwest::multipart::Form::new(),
                |form, (name, value)| {
                    if let Some(s) = value.as_str() {
                        form.text(name.to_owned(), s.to_string())
                    } else if let Ok(value) = serde_json::to_string(value) {
                        form.text(name.to_owned(), value)
                    } else {
                        error!(field = %name, "Skipping field due to invalid value: {:?}", value);
                        form
                    }
                },
            );

            let form = files
                .into_iter()
                .fold(form, |form, (name, part)| form.part(name, part));

            trace!("Built request form: {:?}", form);

            self.client
                .post(&url)
                .multipart(form)
                .send()
                .await?
                .json()
                .await?
        } else {
            // No files to upload, use a JSON body in a POST request to the
            // requested endpoint.

            trace!("Request has no files");

            self.client
                .post(&url)
                .json(&values)
                .send()
                .await?
                .json()
                .await?
        };

        debug!("Got response: {:?}", resp);

        resp.into()
    }

    /// Download a file from Telegram's servers.
    ///
    /// It requires a file path which can be obtained with [requests::GetFile].
    #[tracing::instrument(skip(self))]
    pub async fn download_file(&self, file_path: &str) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}file/bot{}/{}",
            self.api_endpoint, self.api_key, file_path
        );

        Ok(self.client.get(&url).send().await?.bytes().await?.to_vec())
    }
}
