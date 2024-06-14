use nannou::prelude::*;
use screen::gui::sprite::sprite_manager::SpriteList;
use screen::input_device_monitor::app::AppHandler;
use screen::socket_server::conversion::{convert_frame_to_gui_sprite, play_frame_sound};
use screen::socket_server::frame::Frame as ScreenFrame;
use screen::socket_server::server::run_server;
use screen::sound_manager::audio::Audio;
use crossbeam_channel::{unbounded, Receiver};
use rayon::ThreadPoolBuilder;
use std::sync::Arc;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    app_handler: AppHandler,
    sprite_list: SpriteList,
    receiver: Receiver<ScreenFrame>,
    audio: Arc<Audio>,
    thread_pool: rayon::ThreadPool,
}

fn model(app: &App) -> Model {
    let (tx, rx) = unbounded();

    std::thread::spawn(move || {
        run_server(tx);
    });

    let _window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();

    let mut app_handler = AppHandler::new();
    app_handler.init();

    let sprite_list = SpriteList::new();
    let audio = Arc::new(Audio::new());

    let thread_pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    Model {
        app_handler,
        sprite_list,
        receiver: rx,
        audio,
        thread_pool,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    while let Ok(frame) = model.receiver.try_recv() {
        if let Some(ref frame_sprite) = frame.sprite {
            if let Err(e) = convert_frame_to_gui_sprite(app, frame_sprite).and_then(|_gui_sprite| {
                model.sprite_list.add_sprite(
                    app,
                    &frame_sprite.file_path,
                    "received_sprite",
                    frame_sprite.position.x as f32,
                    frame_sprite.position.y as f32,
                );
                Ok(())
            }) {
                eprintln!("{}", e);
            }
        }

        if let Some(ref frame_sound) = frame.sound {
            let audio = Arc::clone(&model.audio);
            let frame_sound = frame_sound.clone();
            model.thread_pool.spawn(move || {
                if let Err(e) = play_frame_sound(&frame_sound, &audio) {
                    eprintln!("{}", e);
                }
            });
        }
    }
}
fn window_event(app: &App, model: &mut Model, event: WindowEvent) {
    let _ = app;
    model.app_handler.handle_window_event(&event);
}

fn view(app: &App, model: &Model, frame: nannou::Frame) {
    let draw = app.draw();
    frame.clear(DIMGRAY);

    for sprite in model.sprite_list.sprites.values() {
        let texture = &sprite.texture;
        let position = sprite.position;
        let image_size = texture.size();
        let (width, height) = (image_size[0] as f32, image_size[1] as f32);
        let dimensions = sprite.dimensions.unwrap_or_else(|| vec2(width, height));
        draw.texture(texture)
            .xy(position)
            .wh(dimensions);
    }

    draw.to_frame(app, &frame).unwrap();
}
