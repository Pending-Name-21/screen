use flatbuffers::{FlatBufferBuilder, WIPOffset};
use nannou::event::Key;

use crate::input_device_monitor::my_event::flatbuffer::{Keyboard, KeyboardArgs};

pub fn keyboard_as_flatbuffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    state: &str,
    key: &Key,
) -> WIPOffset<Keyboard<'a>> {
    let event_state = builder.create_string(state);
    let key_name = builder.create_string(&format!("{:?}", key));

    let keyboard_event = Keyboard::create(
        builder,
        &KeyboardArgs {
            type_: Some(event_state),
            key: Some(key_name),
        },
    );

    keyboard_event
}
