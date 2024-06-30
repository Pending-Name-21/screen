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

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_update_last_mouse_point_in_another_scope() {
        let initial_point = get_last_mouse_point();
        assert_eq!(initial_point.x, 0.0);
        assert_eq!(initial_point.y, 0.0);

        {
            update_last_mouse_point(5.0, 10.0);
        }

        let updated_point = get_last_mouse_point();
        assert_eq!(updated_point.x, 5.0);
        assert_eq!(updated_point.y, 10.0);
    }

    #[test]
    fn test_update_last_mouse_point_concurrently() {
        let initial_point = get_last_mouse_point();
        assert_eq!(initial_point.x, 0.0);
        assert_eq!(initial_point.y, 0.0);

        let update_thread = thread::spawn(|| {
            update_last_mouse_point(5.0, 10.0);
        });

        let updated_point = get_last_mouse_point();
        assert_eq!(updated_point.x, 5.0);
        assert_eq!(updated_point.y, 10.0);

        update_thread.join().unwrap();
    }
}
