use crate::input_device_monitor::my_event::MyVec2;
use nannou::glam::Vec2;

impl From<Vec2> for MyVec2 {
    fn from(vec: Vec2) -> Self {
        MyVec2 { x: vec.x, y: vec.y }
    }
}
