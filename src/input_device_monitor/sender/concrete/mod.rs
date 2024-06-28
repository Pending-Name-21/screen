pub mod flat_buffers_events;
pub mod socket_client_sender;

pub use socket_client_sender::SocketClientSender;
pub use socket_client_sender::SOCKET_SERVER_PATH;

pub use flat_buffers_events::coffee_time::input_events::{
    Event, EventArgs, Keyboard, KeyboardArgs, Mouse, MouseArgs, Position, PositionArgs,
};
