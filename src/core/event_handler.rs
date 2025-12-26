use core::fmt;
use std::fmt::write;

use serde_json::Value;
use tokio::sync::mpsc;

use crate::{
    core::Engine,
    model::{User, parse_user},
};

#[derive(Debug)]
pub enum EngineCommand {
    AddActiveUser(User),
}

impl fmt::Display for EngineCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)] // Crucial: Each observer gets its own clone of the handle
pub struct EventHandler {
    tx: mpsc::UnboundedSender<EngineCommand>,
}

impl EventHandler {
    pub fn handle(&self, j: &str) {
        let v: Value = serde_json::from_str(&j).unwrap();
        if v["cmd"].is_null() {
            println!("missing cmd, payload: {}", v);
            return;
        }
        let cmd = v["cmd"].as_str().unwrap();
        match cmd {
            "join" => {}
            "onlineSet" => {
                println!("onlineSet event");
            }
            "onlineAdd" => {
                let u = parse_user(j);
                self.tx.send(EngineCommand::AddActiveUser(u)).unwrap();
            }
            "onlineRemove" => {}
            "chat" => {
                println!("chat: {}", v["text"]);
            }
            "info" => println!("info: {}", v["text"]),
            _ => {
                println!("unknown cmd: {}", v)
            }
        }
    }

    pub fn send(&self, command: EngineCommand) {
        self.tx.send(command).unwrap();
    }
}

pub fn new(tx: mpsc::UnboundedSender<EngineCommand>) -> EventHandler {
    EventHandler { tx: tx }
}
