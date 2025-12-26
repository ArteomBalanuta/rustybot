use std::collections::HashMap;
// use std::sync::{Arc};

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{SinkExt, StreamExt, future, pin_mut};

use serde_json::Value;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{Method, Request};

use tokio::io::{AsyncWriteExt, stdout};
use tokio::time::sleep;

mod model;
use model::User;

mod core;
use core::Connection;
use core::Engine;

mod listener;
use listener::OnlineListenerImpl;

use crate::core::{EngineCommand, EngineImpl, EventHandler};
use crate::listener::ChatListenerImpl;
use crate::model::parse_user;

#[tokio::main]
async fn main() {
    let url = "wss://hack.chat/chat-ws";
    let (tx, mut rx) = Connection::connect(url).await;

    let join = r#"{"cmd": "join", "channel": "programming", "nick": "rustskymonke"}"#;
    tx.send(String::from(join)).unwrap();

    let (tx_feedback, rx_feedback) = mpsc::unbounded_channel::<String>();
    let (tx_engine, rx_engine) = mpsc::unbounded_channel::<EngineCommand>();
    let mut engine = EngineImpl {
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

    let mut handle = core::new(engine.get_tx(), rx_feedback);

    // background engine loop that checks for incoming EngineCommand events
    engine.start().await;

    // check for the response immediately
    handle.process_response().await;

    // our ws message receiver
    while let Some(v) = rx.recv().await {
        // send received WS message to the engine
        handle.to_engine(&v);
    }
}
