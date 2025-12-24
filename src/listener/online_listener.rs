
use crate::core::Engine;
use crate::core::EngineImpl;

pub trait OnlineListener {
    fn notify(&self, json: &str);
}

pub struct OnlineListenerImpl {
    engine: EngineImpl
}

impl OnlineListener for OnlineListenerImpl {
    fn notify(&self, json: &str) {
        println!("{}", self.engine.name);
    }
} 