use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::rc::{Rc, Weak};

use serde_json::Value;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::core::event_handler::EngineCommand;
use crate::model::Flair;
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

    pub online_listener: Option<Box<dyn Listener>>,
    pub chat_listener: Option<Box<dyn Listener>>,

    pub tx: mpsc::UnboundedSender<EngineCommand>,
    pub rx: Option<mpsc::UnboundedReceiver<EngineCommand>>,

    pub tx_feedback: mpsc::UnboundedSender<String>,
}

impl EngineImpl {
    pub fn s(&self) -> String {
        return "dummy".to_string();
    }

    pub fn get_tx(&self) -> mpsc::UnboundedSender<EngineCommand> {
        return self.tx.clone();
    }

    pub fn set_online_listener(&mut self, l: OnlineListenerImpl) {
        self.online_listener = Some(Box::from(l));
    }

    pub fn set_chat_listener(&mut self, l: ChatListenerImpl) {
        self.chat_listener = Some(Box::from(l));
    }
}

pub fn new() -> EngineImpl {
    let (tx, rx) = mpsc::unbounded_channel::<EngineCommand>();

    let (tx_feedback, rx_feedback) = mpsc::unbounded_channel::<String>();

    return EngineImpl {
        name: "bot".to_string(),
        channel: "programing".to_string(),
        prefix: "*".to_string(),
        active_users: HashMap::new(),
        afk_users: HashMap::new(),
        online_listener: None,
        chat_listener: None,
        tx: tx,
        rx: Some(rx),

        tx_feedback: tx_feedback,
    };
}

impl Engine for EngineImpl {
    async fn start(&mut self) {
        let tx = self.tx_feedback.clone();
        let o = self.rx.take();
        match o {
            Some(mut rx) => {
                // received from handle
                tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        println!("engine received: {}", msg.to_string());

                        let response =
                            fmt::format(format_args!("acknowledged: {}", msg.to_string()));

                        // responding
                        tx.send(response);
                    }
                });
            }
            None => {}
        }
    }

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
                    println!("notifying onlineSet");
                    l.notify(msg);
                }
            }
            "onlineAdd" => {}
            "onlineRemove" => {}
            "chat" => {
                if let Some(l) = &self.chat_listener {
                    println!("notifying chat");
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

    fn AddActiveUser(&self, joined: User) {}
    fn RemoveActiveUser(&self, left: &User) {}

    fn AddAfkUser(&self, u: &User, reason: &str) {}
    fn GetAfkUsers(&self) -> HashMap<User, String> {
        return HashMap::new();
    }

    fn GetActiveUserByName(&self, name: &str) -> User {
        return User {
            name: self.s(),
            channel: self.s(),
            trip: self.s(),
            u_type: self.s(),
            hash: self.s(),
            color: self.s(),
            flair: Flair::Boolean(false),
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
