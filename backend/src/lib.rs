mod progenitor_client;

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///Test
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "test"
    ///  ],
    ///  "properties": {
    ///    "test": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Test {
        pub test: String,
    }

    impl From<&Test> for Test {
        fn from(value: &Test) -> Self {
            value.clone()
        }
    }

    ///UpdateSessionValue
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "entries"
    ///  ],
    ///  "properties": {
    ///    "entries": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "expires": {
    ///      "type": "integer",
    ///      "format": "uint64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpdateSessionValue {
        pub entries: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expires: Option<u64>,
    }

    impl From<&UpdateSessionValue> for UpdateSessionValue {
        fn from(value: &UpdateSessionValue) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Hello World
///
///Version: 1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0"
    }
}

#[allow(clippy::all)]
impl Client {
    ///Say hello
    ///
    ///Sends a `GET` request to `/hello`
    pub async fn hello<'a>(&'a self) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/hello", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Greetings
    ///
    ///Sends a `GET` request to `/greet`
    pub async fn greet<'a>(
        &'a self,
        name: Option<&'a str>,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/greet", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/test`
    pub async fn test<'a>(
        &'a self,
        body: &'a types::Test,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!("{}/test", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Session
    ///
    ///Sends a `GET` request to `/load_session`
    pub async fn load_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
        let url = format!("{}/load_session", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("session_id", session_id.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/update_session`
    pub async fn update_session<'a>(
        &'a self,
        session_id: &'a str,
        body: &'a types::UpdateSessionValue,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/update_session", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("session_id", session_id.to_string()));
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/remove_session`
    pub async fn remove_session<'a>(
        &'a self,
        session_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/remove_session", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("session_id", session_id.to_string()));
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
