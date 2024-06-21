use crate::input_device_monitor::my_event::MyMouseButton;
use nannou::event::MouseButton;

impl From<MouseButton> for MyMouseButton {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => MyMouseButton::MyLeft,
            MouseButton::Right => MyMouseButton::MyRight,
            MouseButton::Middle => MyMouseButton::MyMiddle,
            MouseButton::Other(code) => MyMouseButton::MyOther(code),
        }
    }
}
