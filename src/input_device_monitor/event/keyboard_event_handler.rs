use nannou::event::WindowEvent;
use crate::input_device_monitor::sender::ISender;
use crate::input_device_monitor::event::AEventHandler;

pub struct KeyboardEventHandler {
    sender: Box<dyn ISender>,
}

impl KeyboardEventHandler {
    pub fn new(sender: Box<dyn ISender>) -> Self {
        Self { sender }
    }
}

impl AEventHandler for KeyboardEventHandler {
    fn handle_event(&self, event: &WindowEvent) {
        if let WindowEvent::KeyPressed(key) = event {
            println!("Key pressed: {:?}", key);
            self.sender.send_event(event);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nannou::event::{WindowEvent, Key};

    struct MockSender;
    impl ISender for MockSender {
        fn send_event(&self, event: &WindowEvent) {
            match event {
                WindowEvent::KeyPressed(key) => assert_eq!(*key, Key::A),
                _ => panic!("Unexpected event type"),
            }
        }
    }

    #[test]
    fn test_handle_key_pressed() {
        let sender = Box::new(MockSender);
        let handler = KeyboardEventHandler::new(sender);
        let key_event = WindowEvent::KeyPressed(Key::A);
        handler.handle_event(&key_event);
    }
}
