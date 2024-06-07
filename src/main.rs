use nannou::event::WindowEvent;
use nannou::prelude::*;

pub trait ISender {
    fn send_event(&self, event: &WindowEvent);
}

pub struct SocketClientSender;

impl ISender for SocketClientSender {
    fn send_event(&self, event: &WindowEvent) {
        println!("Sending event: {:?}", event);
    }
}

pub trait AEventHandler {
    fn handle_event(&self, event: &WindowEvent);
}

pub struct KeyboardEventHandler {
    sender: Box<dyn ISender>,
}

impl KeyboardEventHandler {
    pub fn new(sender: Box<dyn ISender>) -> Self {
        Self { sender }
    }
}

impl AEventHandler for KeyboardEventHandler {
    fn handle_event(&self, event: &WindowEvent) {
        if let WindowEvent::KeyPressed(key) = event {
            println!("Key pressed: {:?}", key);
            self.sender.send_event(event);
        }
    }
}

pub struct AppHandler {
    event_handlers: Vec<Box<dyn AEventHandler>>,
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

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    app_handler: AppHandler,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let mut app_handler = AppHandler::new();
    app_handler.init();

    Model { app_handler }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    model.app_handler.handle_window_event(&event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}
