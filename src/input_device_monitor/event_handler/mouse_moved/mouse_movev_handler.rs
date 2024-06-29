use crate::input_device_monitor::my_event::serializable_clone::MyVec2;
use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    pub static ref LAST_POINT: Mutex<MyVec2> = Mutex::new(MyVec2::new(0.0, 0.0));
}

pub fn update_last_mouse_point(x: f32, y: f32) {
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
