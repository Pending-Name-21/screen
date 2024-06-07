use nannou::event::WindowEvent;
use crate::event::{AEventHandler, KeyboardEventHandler};
use crate::sender::{ISender, SocketClientSender};

pub struct AppHandler {
    pub event_handlers: Vec<Box<dyn AEventHandler>>,
}

impl AppHandler {
    pub fn new() -> Self {
        Self {
            event_handlers: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        let sender: Box<dyn ISender> = Box::new(SocketClientSender);
        let keyboard_handler: Box<dyn AEventHandler> = Box::new(KeyboardEventHandler::new(sender));
        self.event_handlers.push(keyboard_handler);
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        for handler in &self.event_handlers {
            handler.handle_event(event);
        }
    }
}
