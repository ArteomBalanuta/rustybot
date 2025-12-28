pub trait Listener {
    fn notify(&self, json: &str);
}
