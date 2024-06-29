pub mod flatbuffer_event_caster;
pub mod flatbuffer_caster;
pub mod keyboard_caster;
pub mod mouse_caster;

pub use keyboard_caster::keyboard_as_flatbuffer;
pub use mouse_caster::mouse_as_flatbuffer;
pub use flatbuffer_event_caster::event_as_flatbuffer;
