use crate::listener::Listener;

pub struct OnlineListenerImpl {}

impl Listener for OnlineListenerImpl {
    fn notify(&self, _json: &str) {
        println!("in onlineSetListener notify");
    }
}

impl OnlineListenerImpl {}

pub fn new() -> OnlineListenerImpl {
    return OnlineListenerImpl {};
}
