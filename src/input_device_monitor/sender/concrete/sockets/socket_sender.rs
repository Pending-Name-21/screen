use nannou::event::WindowEvent;

pub trait ISocketSender {
    fn send(&mut self, event: &WindowEvent);
}
