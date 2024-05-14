use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse the url")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Request failed")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Server answered with non-ok status: {0}")]
    InvalidStatus(reqwest::StatusCode),
    #[error("Could not parse server response: {json}")]
    ParsingFailed {
        #[source]
        error: serde_json::Error,
        json: String,
    },
}
