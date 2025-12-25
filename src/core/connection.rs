use std::time::Duration;

use futures::stream::{Any, SplitSink};
use futures::{SinkExt, StreamExt, future, pin_mut};

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tungstenite::Message;
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{Method, Request};

use tokio::io::{AsyncWriteExt, stdout};
use tokio::time::sleep;

pub struct Connection {
    url: String,
    writer: mpsc::UnboundedSender<Message>,
}

impl Connection {
    //"wss://hack.chat/chat-ws"
    pub async fn new(url: &str) -> Self {
        let url_request = url.into_client_request().unwrap();
        let (ws_stream, err) = connect_async(url_request).await.unwrap();

        println!("WebSocket handshake has been successfully completed");

        let (mut write, mut read) = ws_stream.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        // writer
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    eprintln!("Write error: {}", e);
                    break;
                }
            }
        });

        // reader
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => println!("Received: {}", text),
                    Ok(Message::Binary(bin)) => println!("Received binary: {:?}", bin),
                    Err(e) => {
                        eprintln!("Error reading: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Connection {
            url: url.to_string(),
            writer: tx,
        }
    }

    //   let payload = r#"{"cmd": "join", "channel": "programming", "nick": "rustskymonke"}"#;
    pub async fn write(&mut self, msg: &str) {
        self.writer.send(Message::Text(msg.into())).unwrap();
    }
}
