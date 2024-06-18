use nannou::event::WindowEvent;

pub trait IEventSender {
    fn send_event(&mut self, event: &WindowEvent);
}
