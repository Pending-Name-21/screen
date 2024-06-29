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
