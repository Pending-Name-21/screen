use crate::input_device_monitor::my_event::serializable_clone::MyVec2;
use nannou::glam::Vec2;

impl From<Vec2> for MyVec2 {
    fn from(vec: Vec2) -> Self {
        MyVec2 { x: vec.x, y: vec.y }
    }
}

#[cfg(test)]
mod tests {
    use crate::input_device_monitor::my_event::serializable_clone::MyVec2;
    use nannou::glam::Vec2;

    #[test]
    fn test_vec_as_my_vec() {
        let initial_vec = Vec2::new(10.0, 10.0);
        let expected_my_vec = MyVec2::new(10.0, 10.0);

        let result: MyVec2 = initial_vec.into();
        assert_eq!(expected_my_vec, result);
    }
}