use crate::core::Engine;
use crate::core::EngineImpl;

pub trait Listener {
    fn notify(&self, json: &str);
}
