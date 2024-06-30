use flatbuffers::{FlatBufferBuilder, WIPOffset};
use nannou::event::MouseButton;

use crate::input_device_monitor::{
    event_handler::mouse_moved::get_last_mouse_point,
    my_event::flatbuffer::{Mouse, MouseArgs, Position, PositionArgs},
};

pub fn mouse_as_flatbuffer<'a>(
    builder: &mut FlatBufferBuilder<'a>,
    state: &str,
    button: &MouseButton,
) -> WIPOffset<Mouse<'a>> {
    let point = get_last_mouse_point();
    let x = point.x;
    let y = point.y;
    let event_state = builder.create_string(state);
    let button_name = builder.create_string(&format!("{:?}", button));

    let position = Position::create(builder, &PositionArgs { x, y });
    let mouse_event = Mouse::create(
        builder,
        &MouseArgs {
            type_: Some(event_state),
            button: Some(button_name),
            position: Some(position),
        },
    );

    mouse_event
}

#[cfg(test)]
mod tests {
    use crate::input_device_monitor::my_event::flatbuffer::{
        Mouse, MouseArgs, Position, PositionArgs,
    };

    use super::mouse_as_flatbuffer;
    use nannou::event::MouseButton;

    #[test]
    fn test_mouse_button_as_flatbuffer() {
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

        let result = mouse_as_flatbuffer(&mut builder_two, "MousePressed", &initial_mouse_button);

        assert_eq!(expected_mouse_event, result);
    }
}
