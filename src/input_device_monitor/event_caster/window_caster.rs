use crate::input_device_monitor::my_event::MyWindowEvent;
use nannou::event::WindowEvent;

impl From<WindowEvent> for MyWindowEvent {
    fn from(event: WindowEvent) -> Self {
        match event {
            WindowEvent::KeyPressed(key) => MyWindowEvent::MyKeyPressed(key.into()),
            WindowEvent::MouseMoved(vec) => MyWindowEvent::MyMouseMoved(vec.into()),
            WindowEvent::MousePressed(button) => MyWindowEvent::MyMousePressed(button.into()),
            WindowEvent::MouseReleased(button) => MyWindowEvent::MyMouseReleased(button.into()),
            WindowEvent::MouseEntered => MyWindowEvent::MyMouseEntered,
            WindowEvent::MouseExited => MyWindowEvent::MyMouseExited,
            _ => {
                panic!("Unhandled window event: {:?}", event);
            }
        }
    }
}
