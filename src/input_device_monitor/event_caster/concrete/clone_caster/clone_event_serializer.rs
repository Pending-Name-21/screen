use nannou::event::WindowEvent;

use crate::input_device_monitor::{
    event_caster::abstractions::IEventSerializer, my_event::serializable_clone::MyWindowEvent,
};

pub struct CloneEventSerializer;

impl IEventSerializer<WindowEvent> for CloneEventSerializer {
    fn serialize_event(&self, event: &nannou::prelude::WindowEvent) -> Vec<u8> {
        let my_event: MyWindowEvent = event.clone().into();
        let event_json = serde_json::to_string(&my_event).unwrap();
        let buf = event_json.into_bytes();
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::CloneEventSerializer;
    use crate::input_device_monitor::event_caster::abstractions::IEventSerializer;
    use nannou::event::{MouseButton, WindowEvent};

    #[test]
    fn test_event_as_bytes() {
        let initial_event = WindowEvent::MousePressed(MouseButton::Left);
        let serializer = CloneEventSerializer;
        let buf = serializer.serialize_event(&initial_event);
        let expected_buf = vec![
            123, 34, 77, 121, 77, 111, 117, 115, 101, 80, 114, 101, 115, 115, 101, 100, 34, 58, 34,
            77, 121, 76, 101, 102, 116, 34, 125,
        ];
        assert_eq!(expected_buf, buf);
    }
}
