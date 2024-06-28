use crate::input_device_monitor::{
    event_caster::IEventCaster, my_event::serializable_clone::MyWindowEvent,
};

pub struct CloneCaster;

impl IEventCaster for CloneCaster {
    fn cast_event(&self, event: &nannou::prelude::WindowEvent) -> Vec<u8> {
        let my_event: MyWindowEvent = event.clone().into();
        let event_json = serde_json::to_string(&my_event).unwrap();
        let buf = event_json.into_bytes();
        buf
    }
}
