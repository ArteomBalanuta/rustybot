use std::fmt::{self, Debug};

use tokio::sync::mpsc;

use crate::model::HackChatCommand;

use crate::model::User;

#[derive(Debug)]
pub enum EngineCommand {
    AddActiveUser(User),
    SetOnlineUsers(Vec<User>),
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
    tx_ws: mpsc::UnboundedSender<String>,
    rx_ws: Option<mpsc::UnboundedReceiver<String>>,
}

impl Clone for EventHandler {
    fn clone(&self) -> Self {
        Self {
            // UnboundedSender is cheap to clone
            tx: self.tx.clone(),
            // Receiver is NOT cloneable, so the clone gets None
            rx: None,
            tx_ws: self.tx_ws.clone(),
            rx_ws: None,
        }
    }
}

impl EventHandler {
    pub async fn start(mut self) {
        tokio::spawn(async move {
            // our ws message receiver
            let mut rx_ws = self.rx_ws.take().expect("No WS receiver");
            let mut rx = self.rx.take().expect("No engine receiver");

            loop {
                tokio::select! {
                    // ws -> engine
                    Some(ws_msg) = rx_ws.recv() => {
                        self.dispatch(&ws_msg);
                    },

                    // engine -> ws
                    Some(rx_msg) = rx.recv() => {
                        self.send_to_ws(rx_msg);
                    }
                }
            }
        });
    }

    fn dispatch(&mut self, j: &str) {
        match serde_json::from_str::<HackChatCommand>(j) {
            Ok(msg) => match msg {
                HackChatCommand::OnlineSet(data) => {
                    self.send_to_engine(EngineCommand::SetOnlineUsers(data.users));
                }
                HackChatCommand::OnlineAdd(u) => {
                    self.send_to_engine(EngineCommand::AddActiveUser(u))
                }
                HackChatCommand::Chat { text, nick } => {
                    let f = fmt::format(format_args!("<{}>: {}", nick, text));
                    println!("{}", f);
                }
                HackChatCommand::Info { text } => {
                    let f = fmt::format(format_args!("<info>: {}", text));
                    println!("{}", f);
                }
                _ => {
                    println!("unknown cmd: {}", msg.to_string());
                }
            },
            Err(r) => {
                println!("Err: {}", r);
            }
        }
    }

    fn send_to_engine(&self, command: EngineCommand) {
        self.tx.send(command).unwrap();
    }

    fn send_to_ws(&self, json: String) {
        self.tx_ws.send(json).unwrap();
    }
}

pub fn new(
    tx: mpsc::UnboundedSender<EngineCommand>, // to engine
    rx: mpsc::UnboundedReceiver<String>,      // from engine
    tx_ws: mpsc::UnboundedSender<String>,     // to WS
    rx_ws: mpsc::UnboundedReceiver<String>,   // from WS
) -> EventHandler {
    EventHandler {
        tx: tx,
        rx: Some(rx),
        tx_ws: tx_ws,
        rx_ws: Some(rx_ws),
    }
}
