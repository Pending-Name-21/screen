pub mod socket_client_sender;
pub mod transmitter;

pub use socket_client_sender::SocketClientSender;
pub use socket_client_sender::SOCKET_SERVER_PATH;

#[allow(dead_code, unused_imports)]
#[path = "input-events_generated.rs"]
mod event_generated;
pub use event_generated::coffee_time::input_events::{root_as_keyboard_event, KeyboardEvent, KeyboardEventArgs};
