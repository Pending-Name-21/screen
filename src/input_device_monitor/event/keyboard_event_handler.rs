use nannou::event::WindowEvent;
use crate::sender::ISender;
use crate::event::AEventHandler;

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
