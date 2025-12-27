use std::fmt::{self, write};

use serde_json::Value;
use tokio::sync::mpsc;

use crate::model::HackChatCommand;

use crate::{
    core::Engine,
    model::{User, parse_user},
};

#[derive(Debug)]
pub enum EngineCommand {
    AddActiveUser(User),
}

// to_string()
impl fmt::Display for EngineCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct EventHandler {
    // Crucial: Each observer gets its own clone of the handle
    tx: mpsc::UnboundedSender<EngineCommand>,
    rx: Option<mpsc::UnboundedReceiver<String>>,
}

impl Clone for EventHandler {
    fn clone(&self) -> Self {
        Self {
            // UnboundedSender is cheap to clone
            tx: self.tx.clone(),
            // Receiver is NOT cloneable, so the clone gets None
            rx: None,
        }
    }
}

impl EventHandler {
    pub fn to_engine(&self, j: &str) {
        if let Ok(msg) = serde_json::from_str::<HackChatCommand>(j) {
            match msg {
                HackChatCommand::OnlineSet(data) => {
                    data.users
                        .iter()
                        .map(|u| &u.name)
                        .for_each(|name| println!("User online: {}", name));
                }
                HackChatCommand::OnlineAdd(u) => self.send(EngineCommand::AddActiveUser(u)),
                HackChatCommand::Chat {
                    text: text,
                    nick: nick,
                } => {
                    let f = fmt::format(format_args!("<{}>: {}", nick, text));
                    println!("{}", f);
                }
                HackChatCommand::Info { text: text } => {
                    let f = fmt::format(format_args!("<info>: {}", text));
                    println!("{}", f);
                }
                _ => {
                    println!("unknown cmd: {}", msg.to_string());
                }
            }
        }
        // let v: Value = serde_json::from_str(&j).unwrap();
        // if v["cmd"].is_null() {
        //     println!("missing cmd, payload: {}", v);
        //     return;
        // }
        // let cmd = v["cmd"].as_str().unwrap();
        // match cmd {
        //     "join" => {}
        //     "onlineSet" => {
        //         println!("onlineSet event");
        //     }
        //     "onlineAdd" => {
        //         let u = parse_user(j);
        //         self.send(EngineCommand::AddActiveUser(u));
        //     }
        //     "onlineRemove" => {}
        //     "chat" => {
        //         println!("chat: {}", v["text"]);
        //     }
        //     "info" => println!("info: {}", v["text"]),
        //     _ => {

        //     }
        // }
    }

    fn send(&self, command: EngineCommand) {
        self.tx.send(command).unwrap();
    }

    // loop to process engine responses
    pub async fn process_response(&mut self) {
        let o = self.rx.take();
        tokio::spawn(async move {
            match o {
                Some(mut rx) => {
                    while let Some(v) = rx.recv().await {
                        println!("received from engine: {}", v);
                    }
                }
                None => {}
            }
        });
    }
}

pub fn new(
    tx: mpsc::UnboundedSender<EngineCommand>,
    rx: mpsc::UnboundedReceiver<String>,
) -> EventHandler {
    EventHandler {
        tx: tx,
        rx: Some(rx),
    }
}
