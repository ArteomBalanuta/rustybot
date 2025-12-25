use std::collections::HashMap;
use std::rc::{Rc, Weak};

use serde_json::Value;

use crate::{
    core::Engine,
    listener::{ChatListenerImpl, Listener, OnlineListenerImpl},
    model::User,
};

pub struct EngineImpl {
    pub name: String,
    pub channel: String,
    pub prefix: String,

    pub active_users: HashMap<User, String>,
    pub afk_users: HashMap<User, String>,

    pub online_listener: Option<OnlineListenerImpl>,
    pub chat_listener: Option<ChatListenerImpl>,
}

impl EngineImpl {
    pub fn s(&self) -> String {
        return "dummy".to_string();
    }

    pub fn SetOnlineListener(&mut self, l: OnlineListenerImpl) {
        self.online_listener = Some(l);
    }

    pub fn SetChatListenerImpl(&mut self, l: ChatListenerImpl) {
        self.chat_listener = Some(l);
    }
}
impl Engine for EngineImpl {
    fn Start(&self) {}
    fn Stop(&self) {}

    fn DispatchMessage(&self, msg: &str) {
        let v: Value = serde_json::from_str(msg).unwrap();
        if v["cmd"].is_null() {
            println!("missing cmd, payload: {}", v);
            return;
        }
        let cmd = v["cmd"].as_str().unwrap();
        match cmd {
            "join" => println!("{}", msg),
            "onlineSet" => {
                if let Some(l) = &self.online_listener {
                    l.notify(msg);
                }
            }
            "onlineAdd" => {}
            "onlineRemove" => {}
            "chat" => {
                if let Some(l) = &self.chat_listener {
                    l.notify(msg);
                }
            }
            "info" => println!("chat: {}", v["text"]),
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
