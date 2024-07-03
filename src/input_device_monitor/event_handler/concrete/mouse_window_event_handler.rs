use nannou::event::WindowEvent;

use crate::input_device_monitor::{
    event_handler::{
        abstractions::{AEventHandler, MouseEventHandler},
        mouse_moved::update_last_mouse_point,
    },
    my_event::event_type::event_type::EventType,
};

impl AEventHandler<WindowEvent> for MouseEventHandler<WindowEvent> {
    fn handle_event(&self, event: &WindowEvent) {
        match event {
            WindowEvent::MouseMoved(vec) => {
                update_last_mouse_point(vec.x, vec.y);
            }
            WindowEvent::MousePressed(_) | WindowEvent::MouseReleased(_) => {
                let mut sender = self.sender.lock().unwrap();
                let type_event: EventType = event.clone().into();
                sender.send_event(event, &type_event);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::input_device_monitor::sender::abstractions::IEventSender;

    use super::*;
    use nannou::{
        event::{MouseButton, WindowEvent},
        glam::Vec2,
    };

    struct MockMouseEvent;

    impl IEventSender<WindowEvent> for MockMouseEvent {
        fn send_event(&mut self, event: &WindowEvent, _: &EventType) {
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
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseMoved(Vec2::new(0.0, 0.0));
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_pressed() {
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MousePressed(MouseButton::Left);
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_released() {
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseReleased(MouseButton::Left);
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_entered() {
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseEntered;
        handler.handle_event(&mouse_event);
    }

    #[test]
    fn test_handle_mouse_exited() {
        let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> =
            Arc::new(Mutex::new(MockMouseEvent));
        let handler = MouseEventHandler::new(sender);
        let mouse_event = WindowEvent::MouseExited;
        handler.handle_event(&mouse_event);
    }
}
