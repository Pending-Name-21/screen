use std::sync::{Arc, Mutex};
use nannou::event::WindowEvent;

use crate::input_device_monitor::{event::AEventHandler, sender::IEventSender};

pub struct MouseEventHandler {
    pub sender: Arc<Mutex<dyn IEventSender + Send>>,
}

impl MouseEventHandler {
    pub fn new(sender: Arc<Mutex<dyn IEventSender + Send>>) -> Self {
        Self { sender }
    }
}

impl AEventHandler for MouseEventHandler {
    fn handle_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseMoved(_)
            | WindowEvent::MousePressed(_)
            | WindowEvent::MouseReleased(_)
            | WindowEvent::MouseEntered
            | WindowEvent::MouseExited
            | WindowEvent::MouseWheel(_, _) => {
                let sender = self.sender.lock().unwrap();
                sender.send_event(event);
            }
            _ => {}
        }
    }
}
