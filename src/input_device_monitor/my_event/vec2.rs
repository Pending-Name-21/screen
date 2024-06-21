use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MyVec2 {
    pub x: f32,
    pub y: f32,
}

impl MyVec2 {
    pub fn new(x: f32, y: f32) -> MyVec2 {
        MyVec2 { x, y }
    }
}
