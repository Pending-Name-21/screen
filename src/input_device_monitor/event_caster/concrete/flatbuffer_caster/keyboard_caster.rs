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

#[cfg(test)]
mod tests {

    use nannou::event::Key;

    use crate::input_device_monitor::{
        event_caster::concrete::flatbuffer_caster::keyboard_as_flatbuffer,
        my_event::flatbuffer::{Keyboard, KeyboardArgs},
    };

    #[test]
    fn test_keyboard_as_flatbuffer() {
        let mut builder_one = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let mut builder_two = flatbuffers::FlatBufferBuilder::with_capacity(1024);

        let initial_key = Key::A;

        let event_state = builder_one.create_string("KeyPressed");
        let key_name = builder_one.create_string(&format!("{:?}", initial_key));

        let expected_keyboard_event = Keyboard::create(
            &mut builder_one,
            &KeyboardArgs {
                type_: Some(event_state),
                key: Some(key_name),
            },
        );

        let result = keyboard_as_flatbuffer(&mut builder_two, "KeyPressed", &initial_key);

        assert_eq!(expected_keyboard_event, result);
    }
}
