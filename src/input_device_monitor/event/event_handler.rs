use std::sync::{Arc, Mutex};
use nannou::event::WindowEvent;
use crate::input_device_monitor::sender::IEventSender;

pub trait AEventHandler {
    fn handle_event(&self, event: &WindowEvent);
}

pub struct EventHandler {
    pub sender: Arc<Mutex<dyn IEventSender + Send>>,
}

impl EventHandler {
    pub fn new(sender: Arc<Mutex<dyn IEventSender + Send>>) -> Self {
        Self { sender }
    }
}
