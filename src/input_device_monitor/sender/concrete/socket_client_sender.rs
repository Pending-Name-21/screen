use nannou::event::WindowEvent;
use crate::input_device_monitor::sender::IEventSender;

pub struct SocketClientSender;

impl IEventSender for SocketClientSender {
    fn send_event(&self, event: &WindowEvent) {
        // Here is the implementation for SocketClientSender
        println!("Sending event: {:?}", event);
    }
}