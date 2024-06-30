use crate::input_device_monitor::my_event::serializable_clone::MyMouseButton;
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

#[cfg(test)]
mod tests {
    use crate::input_device_monitor::my_event::serializable_clone::MyMouseButton;
    use nannou::event::MouseButton;

    #[test]
    fn test_mouse_button_as_my_mouse_button() {
        let initial_mouse_button = MouseButton::Left;
        let expected_my_mouse_button = MyMouseButton::MyLeft;

        let result: MyMouseButton = initial_mouse_button.into();
        assert_eq!(expected_my_mouse_button, result);
    }
}
