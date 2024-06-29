use nannou::event::WindowEvent;

pub trait IEventCaster:Send {
    fn cast_event(&self, event: &WindowEvent) -> Vec<u8>;
}
