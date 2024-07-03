use crate::input_device_monitor::my_event::event_type::event_type::EventType;

pub trait IEventSender<T> {
    fn send_event(&mut self, event: &T, type_event: &EventType);
}
