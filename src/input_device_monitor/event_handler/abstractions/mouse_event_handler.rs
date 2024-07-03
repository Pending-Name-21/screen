use crate::input_device_monitor::sender::abstractions::IEventSender;

use std::sync::{Arc, Mutex};

pub struct MouseEventHandler<T> {
    pub sender: Arc<Mutex<dyn IEventSender<T> + Send>>,
}

impl<T> MouseEventHandler<T> {
    pub fn new(sender: Arc<Mutex<dyn IEventSender<T> + Send>>) -> Self {
        Self { sender }
    }
}
