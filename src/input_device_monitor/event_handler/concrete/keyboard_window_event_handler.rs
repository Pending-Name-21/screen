use nannou::event::WindowEvent;

use crate::input_device_monitor::{
    event_handler::abstractions::{AEventHandler, KeyboardEventHandler},
    my_event::event_type::event_type::EventType,
};

impl AEventHandler<WindowEvent> for KeyboardEventHandler<WindowEvent> {
    fn handle_event(&self, event: &WindowEvent) {
        if let WindowEvent::KeyPressed(_) | WindowEvent::KeyReleased(_) = event {
            let mut sender = self.sender.lock().unwrap();
            let type_event: EventType = event.clone().into();
            sender.send_event(event, &type_event);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::input_device_monitor::sender::abstractions::IEventSender;

    use super::*;
    use nannou::event::{Key, WindowEvent};

    struct MockSender;
    impl IEventSender<WindowEvent> for MockSender {
        fn send_event(&mut self, event: &WindowEvent, _: &EventType) {
            match event {
                WindowEvent::KeyPressed(key) => assert_eq!(*key, Key::A),
                _ => panic!("Unexpected event type"),
            }
        }
    }

    #[test]
    fn test_handle_key_pressed() {
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockSender));
        let handler = KeyboardEventHandler::new(sender);
        let key_event = WindowEvent::KeyPressed(Key::A);
        handler.handle_event(&key_event);
    }
}
