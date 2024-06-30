pub trait IEventSerializer<T>: Send {
    fn serialize_event(&self, event: &T) -> Vec<u8>;
}
