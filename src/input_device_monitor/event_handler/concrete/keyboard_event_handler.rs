use crate::input_device_monitor::event_handler::AEventHandler;
use crate::input_device_monitor::sender::IEventSender;
use nannou::event::WindowEvent;
use std::sync::{Arc, Mutex};

pub struct KeyboardEventHandler {
    pub sender: Arc<Mutex<dyn IEventSender + Send>>,
}

impl KeyboardEventHandler {
    pub fn new(sender: Arc<Mutex<dyn IEventSender + Send>>) -> Self {
        Self { sender }
    }
}

impl AEventHandler for KeyboardEventHandler {
    fn handle_event(&self, event: &WindowEvent) {
        if let WindowEvent::KeyPressed(_) | WindowEvent::KeyReleased(_) = event {
            let mut sender = self.sender.lock().unwrap();
            sender.send_event(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nannou::event::{Key, WindowEvent};

    struct MockSender;
    impl IEventSender for MockSender {
        fn send_event(&mut self, event: &WindowEvent) {
            match event {
                WindowEvent::KeyPressed(key) => assert_eq!(*key, Key::A),
                _ => panic!("Unexpected event type"),
            }
        }
    }

    #[test]
    fn test_handle_key_pressed() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockSender));
        let handler = KeyboardEventHandler::new(sender);
        let key_event = WindowEvent::KeyPressed(Key::A);
        handler.handle_event(&key_event);
    }
}
