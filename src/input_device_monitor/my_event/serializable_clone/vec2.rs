use serde::{Deserialize, Serialize};
use std::fmt;

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

impl fmt::Display for MyVec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
