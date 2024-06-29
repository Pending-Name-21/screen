use nannou::event::WindowEvent;

use crate::input_device_monitor::event_caster::IEventCaster;

use super::event_as_flatbuffer;

pub struct FlatBufferCaster;

impl IEventCaster for FlatBufferCaster {
    fn cast_event(&self, event: &WindowEvent) -> Vec<u8> {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let flatbuffer_event = event_as_flatbuffer(&mut builder, event);
        builder.finish(flatbuffer_event, None);
        let buf: &[u8] = builder.finished_data();
        buf.to_vec()
    }
}
