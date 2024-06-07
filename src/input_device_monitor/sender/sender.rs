use nannou::event::WindowEvent;

pub trait IEventSender {
    fn send_event(&self, event: &WindowEvent);
}
