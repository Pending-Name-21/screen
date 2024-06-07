use std::sync::Arc;
use std::sync::Mutex;

use nannou::event::WindowEvent;

use crate::input_device_monitor::event::concrete::KeyboardEventHandler;
use crate::input_device_monitor::event::concrete::MouseEventHandler;
use crate::input_device_monitor::event::AEventHandler;
use crate::input_device_monitor::sender::concrete::SocketClientSender;
use crate::input_device_monitor::sender::IEventSender;

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
        let sender: Arc<Mutex<dyn IEventSender + Send>> 
            = Arc::new(Mutex::new(SocketClientSender));

        let keyboard_handler: Box<dyn AEventHandler> 
            = Box::new(KeyboardEventHandler::new(sender.clone()));
        let mouse_handler: Box<dyn AEventHandler> 
            = Box::new(MouseEventHandler::new(sender.clone()));
            
        self.event_handlers.push(keyboard_handler);
        self.event_handlers.push(mouse_handler);
    }

    pub fn handle_window_event(&self, event: &WindowEvent) {
        for handler in &self.event_handlers {
            handler.handle_event(event);
        }
    }
}
