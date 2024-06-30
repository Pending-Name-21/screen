use serde::{Deserialize, Serialize};

use super::{key::MyKey, mouse_button::MyMouseButton, vec2::MyVec2};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MyWindowEvent {
    MyKeyPressed(MyKey),
    MyKeyReleased(MyKey),
    MyMouseMoved(MyVec2),
    MyMousePressed(MyMouseButton),
    MyMouseReleased(MyMouseButton),
    MyMouseEntered,
    MyMouseExited,
}