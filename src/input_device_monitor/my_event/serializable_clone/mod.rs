pub mod key;
pub mod mouse_button;
pub mod vec2;
pub mod window;

pub use key::MyKey;
pub use mouse_button::MyMouseButton;
pub use vec2::{MyVec2, update_last_mouse_point, get_last_mouse_point};
pub use window::MyWindowEvent;