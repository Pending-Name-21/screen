use std::sync::Arc;
use std::sync::Mutex;

use nannou::event::WindowEvent;

use crate::input_device_monitor::event_caster::concrete::flatbuffer_caster::flatbuffer_event_serializer::FlatBufferEventSerializer;
use crate::input_device_monitor::event_handler::abstractions::AEventHandler;
use crate::input_device_monitor::event_handler::abstractions::KeyboardEventHandler;
use crate::input_device_monitor::event_handler::abstractions::MouseEventHandler;
use crate::input_device_monitor::sender::abstractions::IEventSender;
use crate::input_device_monitor::sender::concrete::sockets::SocketClientSender;
use crate::input_device_monitor::sender::concrete::sockets::SOCKET_SERVER_PATH;

pub struct AppHandler<T> {
    pub event_handlers: Vec<Box<dyn AEventHandler<T>>>,
}

impl<T> AppHandler<T> {
    pub fn new(event_handlers: Vec<Box<dyn AEventHandler<T>>>) -> Self {
        Self { event_handlers }
    }

    pub fn handle_window_event(&self, event: &T) {
        for handler in &self.event_handlers {
            handler.handle_event(event);
        }
    }
}

pub fn init_event_window_app_handler() -> AppHandler<WindowEvent> {
    let serilizer = Box::new(FlatBufferEventSerializer);
    let sender: Arc<Mutex<dyn IEventSender<WindowEvent> + Send>> = Arc::new(Mutex::new(
        SocketClientSender::new(SOCKET_SERVER_PATH, serilizer).unwrap(),
    ));

    let keyboard_handler: Box<dyn AEventHandler<WindowEvent>> =
        Box::new(KeyboardEventHandler::new(sender.clone()));
    let mouse_handler: Box<dyn AEventHandler<WindowEvent>> =
        Box::new(MouseEventHandler::new(sender.clone()));

    let mut handlers = Vec::new();

    handlers.push(keyboard_handler);
    handlers.push(mouse_handler);

    let app = AppHandler::new(handlers);
    return app;
}
