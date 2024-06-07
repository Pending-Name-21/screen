// This main file need to be updated on UI manager US
use nannou::prelude::*;
use screen::input_device_monitor::app::AppHandler;

fn main() {
    nannou::app(model).update(update).view(view).run();
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

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(DIMGRAY);
}

struct Model {
    app_handler: AppHandler,
}
