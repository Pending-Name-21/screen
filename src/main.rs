use crossbeam_channel::unbounded;
use nannou::prelude::*;
use rayon::ThreadPoolBuilder;
use screen::gui::sprite::sprite_manager::SpriteList;
use screen::input_device_monitor::app::{init_event_window_app_handler, AppHandler};
use screen::socket_server::queue_manager::process_frame;
use screen::socket_server::receiver::FrameReceiver;
use screen::socket_server::server::run_server;
use screen::sound_manager::audio::Audio;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

extern crate dotenv_codegen;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    app_handler: AppHandler<WindowEvent>,
    sprite_list: SpriteList,
    frame_receiver: FrameReceiver,
    audio: Arc<Audio>,
    thread_pool: Arc<rayon::ThreadPool>,
}

fn model(app: &App) -> Model {
    let (tx, rx) = unbounded();
    let thread_pool: Arc<rayon::ThreadPool> =
        Arc::new(ThreadPoolBuilder::new().num_threads(4).build().unwrap());

    let server_thread_pool = Arc::clone(&thread_pool);
    std::thread::spawn(move || {
        run_server(tx, server_thread_pool);
    });

    let _window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let app_handler = init_event_window_app_handler();

    let sprite_list = SpriteList::new();
    let audio = Arc::new(Audio::new());
    let frame_receiver = FrameReceiver::new(rx);

    Model {
        app_handler,
        sprite_list,
        frame_receiver,
        audio,
        thread_pool,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.frame_receiver.receive_frames();

    if let Some(frame) = model.frame_receiver.get_next_frame() {
        process_frame(
            app,
            &mut model.sprite_list,
            &model.audio,
            &model.thread_pool,
            frame,
        );
    }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    let _ = app;
    model.app_handler.handle_window_event(&event);
}

fn view(app: &App, model: &Model, frame: nannou::Frame) {
    dotenv().ok();
    let selected_cells_path: String = env::var("SELECTED_CELLS_PATH").expect("ADDRESS must be set");
    let draw = app.draw();
    frame.clear(DIMGRAY);

    let mut sprites_specific_path = Vec::new();
    let mut other_sprites = Vec::new();

    for sprite in model.sprite_list.sprites.values() {
        if sprite.path.starts_with(&selected_cells_path) {
            sprites_specific_path.push(sprite);
        } else {
            other_sprites.push(sprite);
        }
    }

    for sprite in sprites_specific_path {
        let texture = &sprite.texture;
        let position = sprite.position;
        let image_size = texture.size();
        let (width, height) = (image_size[0] as f32, image_size[1] as f32);
        let dimensions = sprite.dimensions.unwrap_or_else(|| vec2(width, height));
        draw.texture(texture).xy(position).wh(dimensions);
    }

    for sprite in other_sprites {
        let texture = &sprite.texture;
        let position = sprite.position;
        let image_size = texture.size();
        let (width, height) = (image_size[0] as f32, image_size[1] as f32);
        let dimensions = sprite.dimensions.unwrap_or_else(|| vec2(width, height));
        draw.texture(texture).xy(position).wh(dimensions);
    }

    draw.to_frame(app, &frame).unwrap();
}
