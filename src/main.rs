// This main file need to be updated on UI manager US
use nannou::prelude::*;
use screen::gui::sprite::sprite_manager::SpriteList;
use screen::input_device_monitor::app::AppHandler;

extern crate dotenv_codegen;

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

    let mut sprite_list = SpriteList::new();
    sprite_list.add_sprite(app, "assets/images/pacman.jpeg", "pacman1", 50.0, 50.0);
    sprite_list.add_sprite_dim(
        app,
        "assets/images/pacman.jpeg",
        "pacman2",
        100.0,
        100.0,
        50.0,
        50.0,
    );

    Model {
        app_handler,
        sprite_list,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model
        .sprite_list
        .update_sprite("pacman1", 200.0, 200.0, Some(75.0), Some(75.0));
    _model
        .sprite_list
        .update_sprite("pacman1", 300.0, 300.0, None, None);

    _model.sprite_list.remove_sprite("pacman2");
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    model.app_handler.handle_window_event(&event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(DIMGRAY);

    for sprite in model.sprite_list.sprites.values() {
        let texture = &sprite.texture;
        let position = sprite.position;
        let image_size = texture.size();
        let (width, height) = (image_size[0] as f32, image_size[1] as f32);
        let dimensions = sprite.dimensions.unwrap_or_else(|| vec2(width, height));
        draw.texture(texture).xy(position).wh(dimensions);
    }

    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    app_handler: AppHandler,
    sprite_list: SpriteList,
}
