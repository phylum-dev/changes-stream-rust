use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
    Change(ChangeEvent),
    Finished(FinishedEvent),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangeEvent {
    pub seq: Value,
    pub id: String,
    pub changes: Vec<Change>,

    #[serde(default)]
    pub deleted: bool,

    #[serde(default)]
    #[cfg(feature = "raw_value_doc")]
    pub doc: Option<Box<serde_json::value::RawValue>>,
    #[cfg(not(feature = "raw_value_doc"))]
    pub doc: Option<serde_json::Map<String, Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishedEvent {
    pub last_seq: Value,
    pub pending: Option<u64>, // not available on CouchDB 1.0
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Change {
    pub rev: String,
}
