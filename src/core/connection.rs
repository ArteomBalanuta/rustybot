use std::process::{ExitCode, exit};
use std::sync::{Arc, Mutex};
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

use crate::core::{Engine, EngineImpl};

pub struct Connection {}
impl Connection {
    //"wss://hack.chat/chat-ws"
    pub async fn connect(
        url: &str,
    ) -> (
        mpsc::UnboundedSender<String>,
        mpsc::UnboundedReceiver<String>,
    ) {
        let url_request = url.into_client_request().unwrap();
        let (ws_stream, _) = connect_async(url_request).await.unwrap();

        println!("WebSocket handshake has been successfully completed");

        let (mut write, mut read) = ws_stream.split();
        let (tx_writer, mut rx_writer) = mpsc::unbounded_channel::<String>();
        // writer
        tokio::spawn(async move {
            while let Some(msg) = rx_writer.recv().await {
                if let Err(e) = write.send(Message::Text(msg.into())).await {
                    eprintln!("Write error: {}", e);
                    break;
                }
            }
        });

        let (tx_receiver, rx_receiver) = mpsc::unbounded_channel::<String>();
        // receiver
        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        println!("Received: {}", text);
                        let _ = tx_receiver.send(text.to_string());
                    }
                    Ok(Message::Binary(bin)) => println!("Received binary: {:?}", bin),
                    Ok(Message::Close(frame)) => {
                        println!("Server sent close frame: {:?}", frame);
                    }
                    Err(e) => {
                        eprintln!("Error reading: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        return (tx_writer, rx_receiver);
    }
}
