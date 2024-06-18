use nannou::event::WindowEvent;
use serde_json;
use std::io::prelude::*;
use std::os::unix::net::UnixStream;

use crate::input_device_monitor::my_event::MyWindowEvent;
use crate::input_device_monitor::sender::IEventSender;

const SOCKET_SERVER_PATH: &str = "/tmp/events-socket.sock";

pub struct SocketClientSender {
    stream: UnixStream,
}

impl SocketClientSender {
    pub fn new() -> std::io::Result<Self> {
        let stream = UnixStream::connect(SOCKET_SERVER_PATH)?;
        Ok(Self { stream })
    }
}

impl IEventSender for SocketClientSender {
    fn send_event(&mut self, event: &WindowEvent) {
        let my_event: MyWindowEvent = event.clone().into();
        let event_json = serde_json::to_string(&my_event).unwrap();
        writeln!(self.stream, "{}", event_json).unwrap();
        println!("✓ Sent : {}", event_json);
    }
}
