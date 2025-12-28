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
