use flatbuffers::{FlatBufferBuilder, WIPOffset};
use nannou::event::WindowEvent;

use super::{keyboard_as_flatbuffer, mouse_as_flatbuffer};
use crate::input_device_monitor::my_event::flatbuffer::{Event, EventArgs, Keyboard, Mouse};

pub fn event_as_flatbuffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    event: &WindowEvent,
) -> WIPOffset<Event<'a>> {
    match event {
        WindowEvent::KeyPressed(key) => {
            let key_event = keyboard_as_flatbuffer(builder, "KeyPressed", key);
            return keyboard_event_as_flatbuffer(builder, key_event);
        }
        WindowEvent::KeyReleased(key) => {
            let key_event = keyboard_as_flatbuffer(builder, "KeyReleased", key);
            return keyboard_event_as_flatbuffer(builder, key_event);
        }
        WindowEvent::MousePressed(button) => {
            let key_event = mouse_as_flatbuffer(builder, "MousePressed", button);
            return mouse_event_as_flatbuffer(builder, key_event);
        }
        WindowEvent::MouseReleased(button) => {
            let key_event = mouse_as_flatbuffer(builder, "MouseReleased", button);
            return mouse_event_as_flatbuffer(builder, key_event);
        }
        _ => {
            panic!("Unhandled window event: {:?}", event);
        }
    }
}

fn keyboard_event_as_flatbuffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    keyboard_event: WIPOffset<Keyboard>,
) -> WIPOffset<Event<'a>> {
    let event = Event::create(
        builder,
        &EventArgs {
            keyboard: Some(keyboard_event),
            mouse: None,
        },
    );

    return event;
}

fn mouse_event_as_flatbuffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    mouse_event: WIPOffset<Mouse>,
) -> WIPOffset<Event<'a>> {
    let event = Event::create(
        builder,
        &EventArgs {
            keyboard: None,
            mouse: Some(mouse_event),
        },
    );

    return event;
}

#[cfg(test)]
mod tests {
    use nannou::event::{Key, MouseButton};

    use crate::input_device_monitor::{
        event_caster::flatbuffer_caster::{flatbuffer_event_caster::mouse_event_as_flatbuffer, keyboard_as_flatbuffer, mouse_as_flatbuffer},
        my_event::flatbuffer::{Event, EventArgs, Keyboard, KeyboardArgs, Mouse, MouseArgs, Position, PositionArgs},
    };

    use super::keyboard_event_as_flatbuffer;

    #[test]
    fn test_keyboard_event_as_flatbuffer() {
        let mut builder_one = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let mut builder_two = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let initial_key = Key::A;

        let event_state = builder_one.create_string("KeyPressed");
        let key_name = builder_one.create_string(&format!("{:?}", initial_key));

        let keyboard_event_exp = Keyboard::create(
            &mut builder_one,
            &KeyboardArgs {
                type_: Some(event_state),
                key: Some(key_name),
            },
        );

        let expected_event = Event::create(
            &mut builder_one,
            &EventArgs {
                keyboard: Some(keyboard_event_exp),
                mouse: None,
            },
        );

        let keyboard_event = keyboard_as_flatbuffer(&mut builder_two, "KeyPressed", &initial_key);
        let result = keyboard_event_as_flatbuffer(&mut builder_two, keyboard_event);

        assert_eq!(expected_event, result);
    }

    #[test]
    fn test_mouse_event_as_flatbuffer() {
        let mut builder_one = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let mut builder_two = flatbuffers::FlatBufferBuilder::with_capacity(1024);
        let initial_mouse_button = MouseButton::Left;

        let x = 0.0;
        let y = 0.0;
        let event_state = builder_one.create_string("MousePressed");
        let button_name = builder_one.create_string(&format!("{:?}", initial_mouse_button));

        let position = Position::create(&mut builder_one, &PositionArgs { x, y });
        let expected_mouse_event = Mouse::create(
            &mut builder_one,
            &MouseArgs {
                type_: Some(event_state),
                button: Some(button_name),
                position: Some(position),
            },
        );

        let expected_event = Event::create(
            &mut builder_one,
            &EventArgs {
                keyboard: None,
                mouse: Some(expected_mouse_event),
            },
        );

        let keyboard_event = mouse_as_flatbuffer(&mut builder_two, "MousePressed", &initial_mouse_button);
        let result = mouse_event_as_flatbuffer(&mut builder_two, keyboard_event);

        assert_eq!(expected_event, result);
    }
}
