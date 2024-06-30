use crate::input_device_monitor::my_event::event_type::event_type::EventType;
use nannou::event::WindowEvent;

impl From<WindowEvent> for EventType {
    fn from(event: WindowEvent) -> Self {
        match event {
            WindowEvent::KeyPressed(_) => EventType::KeyPressed,
            WindowEvent::KeyReleased(_) => EventType::KeyReleased,
            WindowEvent::MousePressed(_) => EventType::MouseButtonPressed,
            WindowEvent::MouseReleased(_) => EventType::MouseButtonReleased,
            _ => {
                panic!("Unhandled window event: {:?}", event);
            }
        }
    }
}
