use lazy_static::lazy_static;
use std::{fmt, sync::MutexGuard};
use std::sync::Mutex;

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

impl fmt::Display for MyVec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

lazy_static! {
    pub static ref LAST_POINT: Mutex<MyVec2> = Mutex::new(MyVec2::new(0.0, 0.0));
}

pub fn update_last_mouse_point(x: f32, y:f32) {
    let mut global_vec: MutexGuard<MyVec2> = LAST_POINT.lock().unwrap();
    global_vec.x = x;
    global_vec.y = y;
}

pub fn get_last_mouse_point() -> MyVec2 {
    let global_vec = LAST_POINT.lock().unwrap();
    MyVec2 {
        x: global_vec.x,
        y: global_vec.y,
    }
}