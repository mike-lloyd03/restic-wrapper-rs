use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub time: String,
    pub tree: String,
    pub paths: Vec<String>,
    pub hostname: String,
    pub username: String,
    pub excludes: Option<Vec<String>>,
    pub id: String,
    pub short_id: String,
}

impl Snapshot {
    pub fn from_string(input: String) -> Vec<Self> {
        serde_json::from_str(&input).unwrap()
    }
}
