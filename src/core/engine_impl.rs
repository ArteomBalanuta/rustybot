use std::collections::HashMap;

use serde_json::Value;

use crate::{core::Engine, model::User};

pub struct EngineImpl {
    pub name: String,
    pub channel: String,
    pub prefix: String,

    pub active_users: HashMap<User, String>,
    pub afk_users: HashMap<User, String>,
}
impl EngineImpl {
    pub fn s(&self) -> String {
        return "dummy".to_string();
    }
}
impl Engine for EngineImpl {
    fn Start(&self) {}
    fn Stop(&self) {}

    fn DispatchMessage(&self, msg: &str) {
        let v: Value = serde_json::from_str(msg).unwrap();
        if v["cmd"].is_null() {
            println!("unknown cmd, payload: {}", v);
            return;
        }
        let cmd = v["cmd"].as_str().unwrap();
        match cmd {
            "join" => println!("{}", msg),
            "chat" => println!("chat: {}", v["text"]),
            _ => {
                println!("unknown cmd: {}", msg)
            }
        }
    }
    fn SendRawMessage(&self, message: &str) {}
    fn SendChatMessage(&self, author: &str, message: &str, isWhisper: bool) -> String {
        return "blah".to_string();
    }

    fn AddActiveUser(&self, joined: &User) {}
    fn RemoveActiveUser(&self, left: &User) {}

    fn AddAfkUser(&self, u: &User, reason: &str) {}
    fn GetAfkUsers(&self) -> HashMap<User, String> {
        return HashMap::new();
    }

    fn GetActiveUserByName(&self, name: &str) -> User {
        return User {
            name: self.s(),
            channel: self.s(),
            is_me: false,
            is_bot: false,
            trip: self.s(),
            u_type: self.s(),
            hash: self.s(),
            level: 1111,
            color: self.s(),
            flair: 111,
            user_id: 111,
        };
    }
    fn GetActiveUsers(&self) -> HashMap<User, String> {
        return HashMap::new();
    }

    fn Kick(&self, name: &str, channel: &str) {}

    fn GetPrefix(&self) -> String {
        return "a".to_string();
    }
    fn GetName(&self) -> String {
        return "a".to_string();
    }
    fn GetChannel(&self) -> String {
        return "b".to_string();
    }

    fn SetName(&self, name: &str) {}
}
