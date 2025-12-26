use std::sync::Mutex;
use std::sync::Weak;

use crate::core::Engine;
use crate::core::EngineImpl;
use crate::listener::Listener;

pub struct ChatListenerImpl {}

impl Listener for ChatListenerImpl {
    fn notify(&self, json: &str) {
        println!("chat: {}", json);
    }
}

pub fn new() -> ChatListenerImpl {
    ChatListenerImpl {}
}
