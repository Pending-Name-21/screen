use nannou::event::WindowEvent;

pub trait IEventBytesCaster:Send {
    fn cast_event(&self, event: &WindowEvent) -> Vec<u8>;
}
