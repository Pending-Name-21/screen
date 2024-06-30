use crate::input_device_monitor::my_event::serializable_clone::MyWindowEvent;
use nannou::event::WindowEvent;

impl From<WindowEvent> for MyWindowEvent {
    fn from(event: WindowEvent) -> Self {
        match event {
            WindowEvent::KeyPressed(key) => MyWindowEvent::MyKeyPressed(key.into()),
            WindowEvent::KeyReleased(key) => MyWindowEvent::MyKeyReleased(key.into()),
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

#[cfg(test)]
mod tests {
    use nannou::event::{Key, MouseButton, WindowEvent};
    use crate::input_device_monitor::my_event::serializable_clone::{MyVec2, MyWindowEvent};

    #[test]
    fn test_key_pressed_as_my_key_pressed() {
        let initial_key_event = WindowEvent::KeyPressed(Key::A);
        let expected_key_event = MyWindowEvent::MyKeyPressed(Key::A.into());

        let result: MyWindowEvent = initial_key_event.into();
        assert_eq!(expected_key_event, result);
    }

    #[test]
    fn test_key_released_as_my_key_released() {
        let initial_key_event = WindowEvent::KeyReleased(Key::A);
        let expected_key_event = MyWindowEvent::MyKeyReleased(Key::A.into());

        let result: MyWindowEvent = initial_key_event.into();
        assert_eq!(expected_key_event, result);
    }

    #[test]
    fn test_mouse_moved_as_my_mouse_moved() {
        let initial_mouse_event = WindowEvent::MouseMoved((100.0, 200.0).into());
        let some_position = MyVec2::new(100.0, 200.0);
        let expected_mouse_event = MyWindowEvent::MyMouseMoved(some_position);

        let result: MyWindowEvent = initial_mouse_event.into();
        assert_eq!(expected_mouse_event, result);
    }

    #[test]
    fn test_mouse_pressed_as_my_mouse_pressed() {
        let initial_mouse_event = WindowEvent::MousePressed(MouseButton::Left);
        let expected_mouse_event = MyWindowEvent::MyMousePressed(MouseButton::Left.into());

        let result: MyWindowEvent = initial_mouse_event.into();
        assert_eq!(expected_mouse_event, result);
    }

    #[test]
    fn test_mouse_released_as_my_mouse_released() {
        let initial_mouse_event = WindowEvent::MouseReleased(MouseButton::Left);
        let expected_mouse_event = MyWindowEvent::MyMouseReleased(MouseButton::Left.into());

        let result: MyWindowEvent = initial_mouse_event.into();
        assert_eq!(expected_mouse_event, result);
    }

    #[test]
    fn test_mouse_entered_as_my_mouse_entered() {
        let initial_mouse_event = WindowEvent::MouseEntered;
        let expected_mouse_event = MyWindowEvent::MyMouseEntered;

        let result: MyWindowEvent = initial_mouse_event.into();
        assert_eq!(expected_mouse_event, result);
    }

    #[test]
    fn test_mouse_exited_as_my_mouse_exited() {
        let initial_mouse_event = WindowEvent::MouseExited;
        let expected_mouse_event = MyWindowEvent::MyMouseExited;

        let result: MyWindowEvent = initial_mouse_event.into();
        assert_eq!(expected_mouse_event, result);
    }
}
