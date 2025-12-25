use std::sync::Mutex;
use std::sync::Weak;

use crate::core::Engine;
use crate::core::EngineImpl;
use crate::listener::Listener;

pub struct ChatListenerImpl {
    engine: Weak<Mutex<EngineImpl>>,
}

impl Listener for ChatListenerImpl {
    fn notify(&self, json: &str) {
        println!("chat: {}", json);
        if let Some(engine) = self.engine.upgrade() {
            println!("engine name: {}", engine.lock().unwrap().name);
        } else {
            panic!()
        }
    }
}

impl ChatListenerImpl {
    pub fn new(engine: Weak<Mutex<EngineImpl>>) -> ChatListenerImpl {
        ChatListenerImpl { engine }
    }
}
