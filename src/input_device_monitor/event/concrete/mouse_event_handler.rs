use nannou::event::WindowEvent;
use std::sync::{Arc, Mutex};

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
            | WindowEvent::MouseExited => {
                let sender = self.sender.lock().unwrap();
                sender.send_event(event);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nannou::{
        event::{MouseButton, WindowEvent},
        glam::Vec2,
    };

    struct MockMouseEvent;

    impl IEventSender for MockMouseEvent {
        fn send_event(&self, event: &WindowEvent) {
            match event {
                WindowEvent::MouseMoved(event) => assert_eq!(*event, Vec2::new(0.0, 0.0)),
                WindowEvent::MousePressed(event) => assert_eq!(*event, MouseButton::Left),
                WindowEvent::MouseReleased(event) => assert_eq!(*event, MouseButton::Left),
                WindowEvent::MouseEntered => assert_eq!(*event, WindowEvent::MouseEntered),
                WindowEvent::MouseExited => assert_eq!(*event, WindowEvent::MouseExited),
                _ => panic!("Unexpected mouse event"),
            }
        }
    }

    #[test]
    fn test_handle_mouse_moving() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseMoved(Vec2::new(0.0, 0.0));
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_pressed() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MousePressed(MouseButton::Left);
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_released() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseReleased(MouseButton::Left);
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_entered() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseEntered;
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_exited() {
        let sender: Arc<Mutex<dyn IEventSender + Send>> = Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseExited;
        handler.handle_event(&mouse_event);
    }
}
