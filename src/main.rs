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
use listener::OnlineListener;

#[tokio::main]
async fn main() {
    println!("main started");

    let url = "wss://hack.chat/chat-ws";
    let mut conn = Connection::new(url).await;

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
