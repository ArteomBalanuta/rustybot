use std::collections::HashMap;
// use std::sync::{Arc};

use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::{SinkExt, StreamExt, future, pin_mut};

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

use crate::core::EngineImpl;
use crate::listener::ChatListenerImpl;

#[tokio::main]
async fn main() {
    println!("main started");

    let url = "wss://hack.chat/chat-ws";
    let mut engine = Arc::new(Mutex::new(EngineImpl {
        name: "blah".to_string(),
        prefix: "*".to_string(),
        channel: "programming".to_string(),
        active_users: HashMap::new(),
        afk_users: HashMap::new(),
        online_listener: None,
        chat_listener: None,
    }));

    {
        let weak_engine = Arc::downgrade(&engine);
        let mut unlocked_engine = engine.lock().unwrap();

        unlocked_engine.SetOnlineListener(OnlineListenerImpl::new(weak_engine.clone()));
        unlocked_engine.SetChatListenerImpl(ChatListenerImpl::new(weak_engine));
    }

    // let engine = Arc::new(e);

    let mut conn = Connection::connect(url, engine).await;

    let join = r#"{"cmd": "join", "channel": "programming", "nick": "rustskymonke"}"#;
    conn.write(join).await;

    // Keep the program running so the background tasks don't die
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}

// async fn wsConnect() {
//     let url_request = "wss://hack.chat/chat-ws".into_client_request().unwrap();
//     let (ws_stream, _) = connect_async(url_request).await.unwrap();

//     println!("WebSocket handshake has been successfully completed");

//     let (mut write, mut read) = ws_stream.split();

//     let payload = r#"{"cmd": "join", "channel": "programming", "nick": "rustskymonke"}"#;
//     let msg = Message::Text(payload.into());

//     write.send(msg).await.unwrap();

//     // spawning reader threat.
//     let reading_task = tokio::spawn(async move {
//         loop {
//             while let Some(msg) = read.next().await {
//                 match msg {
//                     Ok(Message::Text(text)) => println!("Received: {}", text),
//                     Ok(Message::Binary(bin)) => println!("Received binary: {:?}", bin),
//                     Err(e) => {
//                         eprintln!("Error reading: {}", e);
//                         break;
//                     }
//                     _ => {}
//                 }
//             }
//         }
//     });

//     let _ = reading_task.await;
// }

// async fn fun() {
//     println!("fun started");
//     sleep(Duration::from_millis(10000)).await;
//     println!("fun ended");
// }

// async fn fun2() {
//     println!("fun2 started");
//     sleep(Duration::from_millis(300)).await;
//     println!("fun2 ended");
// }
