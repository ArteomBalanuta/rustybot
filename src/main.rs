use std::collections::HashMap;
// use std::sync::{Arc};

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{SinkExt, StreamExt, future, pin_mut};

use serde_json::Value;
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

    let mut engine = EngineImpl {
        name: "blah".to_string(),
        prefix: "*".to_string(),
        channel: "programming".to_string(),
        active_users: HashMap::new(),
        afk_users: HashMap::new(),
        online_listener: None,
        chat_listener: None,
    };

    let producer = engine.Start().await;
    let handler = core::new(producer);

    while let Some(v) = rx.recv().await {
        handler.handle(&v);
    }

    // Keep the program running so the background tasks don't die
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
