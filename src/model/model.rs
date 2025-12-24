use serde::Deserialize;
use serde_json::Result;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")] // Maps "isMe" in JSON to "is_me" in Rust
pub struct User {
    pub channel: String,
    pub is_me: bool,
    pub name: String,
    pub trip: String,
    pub u_type: String,
    pub hash: String,
    pub level: u64,
    pub color: String,
    pub flair: u64,
    pub user_id: u64,
    pub is_bot: bool,
}


fn parse_user(json: &str) -> User {
	let u: User = serde_json::from_str(json).unwrap();
	return u;
}

