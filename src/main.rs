use std::collections::HashMap;
// use std::sync::{Arc};

use futures::SinkExt;

use tokio::sync::mpsc;

mod model;

mod core;
use core::Connection;

mod listener;

use crate::core::{EngineCommand, EngineImpl};

#[tokio::main]
async fn main() {
    let url = "wss://hack.chat/chat-ws";
    let (tx_ws, rx_ws) = Connection::connect(url).await;

    let join = r#"{"cmd": "join", "channel": "programming", "nick": "rustskymonke"}"#;
    tx_ws.send(String::from(join)).unwrap();

    let (tx_feedback, rx_feedback) = mpsc::unbounded_channel::<String>();
    let (tx_engine, rx_engine) = mpsc::unbounded_channel::<EngineCommand>();
    let engine = EngineImpl {
        name: "blah".to_string(),
        prefix: "*".to_string(),
        channel: "programming".to_string(),
        active_users: HashMap::new(),
        afk_users: HashMap::new(),
        online_listener: None,
        chat_listener: None,
        tx: tx_engine.clone(),
        rx: Some(rx_engine),

        tx_feedback: tx_feedback.clone(),
    };

    let handle = core::new(engine.get_tx(), rx_feedback, tx_ws, rx_ws);

    // background engine loop that checks for incoming EngineCommand events
    engine.start().await;

    // dispatched engine responses
    handle.start().await;

    tokio::signal::ctrl_c().await.unwrap();
}
