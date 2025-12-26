use std::clone;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Maps "isMe" in JSON to "is_me" in Rust
pub struct User {
    pub channel: String,
    #[serde(rename = "nick")]
    pub name: String,
    pub trip: String,
    pub u_type: String,
    pub hash: String,
    pub color: String,
    pub flair: Flair,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)] // This is the secret sauce
pub enum Flair {
    Boolean(bool),
    Code(String), // Use String to represent the "2 byte char" or code
}

pub fn parse_user(json: &str) -> User {
    // println!("parsing user: {}", json);
    let u: User = serde_json::from_str(json).unwrap();
    return u;
}
