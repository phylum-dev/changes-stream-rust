#[derive(Debug)]
pub enum Error {
    InvalidUrl(url::ParseError),
    RequestFailed(reqwest::Error),
    InvalidStatus(reqwest::StatusCode),
    ParsingFailed(serde_json::Error, String),
}
