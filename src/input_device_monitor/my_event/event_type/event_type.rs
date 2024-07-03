use std::fmt;

pub enum EventType {
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventType::KeyPressed => write!(f, "{}", "Key pressed"),
            EventType::KeyReleased => write!(f, "{}", "Key released"),
            EventType::MouseButtonPressed => write!(f, "{}", "Mouse button pressed"),
            EventType::MouseButtonReleased => write!(f, "{}", "Mouse button released"),
        }
    }
}
