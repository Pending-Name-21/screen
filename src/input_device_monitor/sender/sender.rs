use nannou::event::WindowEvent;

pub trait ISender {
    fn send_event(&self, event: &WindowEvent);
}

pub struct SocketClientSender;

impl ISender for SocketClientSender {
    fn send_event(&self, event: &WindowEvent) {
        // Here is the implementation for SocketClientSender
        println!("Sending event: {:?}", event);
    }
}
