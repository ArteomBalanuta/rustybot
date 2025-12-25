use std::sync::Mutex;
use std::sync::Weak;

use crate::core::Engine;
use crate::core::EngineImpl;
use crate::listener::Listener;

pub struct OnlineListenerImpl {
    engine: Weak<Mutex<EngineImpl>>,
}

impl Listener for OnlineListenerImpl {
    fn notify(&self, json: &str) {
        if let Some(e) = self.engine.upgrade() {
            // 2. Lock the standard mutex (blocks the thread briefly, no await)
            println!("before lock in notify");
            println!("onlineset engine name: {}", e.lock().unwrap().name);
            println!("after lock in notify");
        } else {
            panic!()
        }
    }
}

impl OnlineListenerImpl {
    pub fn new(engine: Weak<Mutex<EngineImpl>>) -> OnlineListenerImpl {
        // if let Some(e) = engine.upgrade() {
        //     // 2. Lock the standard mutex (blocks the thread briefly, no await)
        //     let free_engine = e.lock().expect("Mutex was poisoned");

        //     println!("onlineset engine name: {}", free_engine.name);

        // } else {
        //     panic!()
        // }
        return OnlineListenerImpl { engine };
    }
}
