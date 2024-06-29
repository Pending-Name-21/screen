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
