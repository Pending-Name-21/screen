use nannou::event::WindowEvent;

use crate::input_device_monitor::event_caster::IEventBytesCaster;

use super::event_as_flatbuffer;

pub struct FlatBufferEventBytesCaster;

impl IEventBytesCaster for FlatBufferEventBytesCaster {
    fn cast_event(&self, event: &WindowEvent) -> Vec<u8> {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let flatbuffer_event = event_as_flatbuffer(&mut builder, event);
        builder.finish(flatbuffer_event, None);
        let buf: &[u8] = builder.finished_data();
        buf.to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::input_device_monitor::event_caster::{
        flatbuffer_caster::flatbuffer_event_bytes_caster::FlatBufferEventBytesCaster,
        IEventBytesCaster,
    };
    use nannou::event::{MouseButton, WindowEvent};

    #[test]
    fn test_event_as_bytes() {
        let initial_event = WindowEvent::MousePressed(MouseButton::Left);
        let caster = FlatBufferEventBytesCaster;
        let buf = caster.cast_event(&initial_event);
        let expected_buf = vec![
            12, 0, 0, 0, 8, 0, 10, 0, 0, 0, 4, 0, 8, 0, 0, 0, 16, 0, 0, 0, 0, 0, 10, 0, 16, 0, 4,
            0, 8, 0, 12, 0, 10, 0, 0, 0, 32, 0, 0, 0, 16, 0, 0, 0, 8, 0, 0, 0, 4, 0, 4, 0, 4, 0, 0,
            0, 4, 0, 0, 0, 76, 101, 102, 116, 0, 0, 0, 0, 12, 0, 0, 0, 77, 111, 117, 115, 101, 80,
            114, 101, 115, 115, 101, 100, 0, 0, 0, 0,
        ];
        assert_eq!(expected_buf, buf);
    }
}
