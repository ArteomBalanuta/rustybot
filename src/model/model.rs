use std::{fmt, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
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

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.channel.hash(state);
        self.name.hash(state);
        self.trip.hash(state);
        self.u_type.hash(state);
        self.hash.hash(state);
        self.color.hash(state);
    }
}
impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for User {}

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

#[derive(Deserialize, Debug, Clone)]
pub struct OnlineSetData {
    pub users: Vec<User>, // This matches the "users" array in JSON
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd")] // This tells Serde to look at the "cmd" field
pub enum HackChatCommand {
    #[serde(rename = "onlineSet")]
    OnlineSet(OnlineSetData),
    #[serde(rename = "onlineAdd")]
    OnlineAdd(User),
    #[serde(rename = "onlineRemove")]
    OnlineRemove(User),
    #[serde(rename = "chat")]
    Chat { text: String, nick: String },
    #[serde(rename = "info")]
    Info { text: String },

    #[serde(other)] // Catch-all for commands you don't care about yet
    Unknown,
}

impl fmt::Display for HackChatCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
