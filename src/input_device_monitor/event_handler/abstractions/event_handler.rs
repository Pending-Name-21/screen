use std::sync::{Arc, Mutex};

use crate::input_device_monitor::sender::abstractions::IEventSender;

pub trait AEventHandler<T> {
    fn handle_event(&self, event: &T);
}

pub struct EventHandler<T> {
    pub sender: Arc<Mutex<dyn IEventSender<T> + Send>>,
}

impl<T> EventHandler<T> {
    pub fn new(sender: Arc<Mutex<dyn IEventSender<T> + Send>>) -> Self {
        Self { sender }
    }
}
