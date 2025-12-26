use std::sync::Mutex;
use std::sync::Weak;

use crate::core::Engine;
use crate::core::EngineImpl;
use crate::listener::Listener;

pub struct OnlineListenerImpl {}

impl Listener for OnlineListenerImpl {
    fn notify(&self, json: &str) {
        println!("in onlineSetListener notify");
    }
}

impl OnlineListenerImpl {}

pub fn new() -> OnlineListenerImpl {
    return OnlineListenerImpl {};
}
