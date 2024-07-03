use nannou::prelude::*;
use crate::socket_server::frame::Frame;
use crate::socket_server::conversion::{convert_frame_to_gui_sprite, play_frame_sound};
use crate::gui::sprite::sprite_manager::SpriteList;
use crate::sound_manager::audio::Audio;
use std::path::Path;
use std::sync::Arc;
use rayon::ThreadPool;

pub fn process_frame(app: &App, sprite_list: &mut SpriteList, audio: &Arc<Audio>, thread_pool: &Arc<ThreadPool>, frame: Frame) {
    for frame_sprite in &frame.sprites {
        let path = Path::new(&frame_sprite.path);
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("received_sprite");
        let key = file_stem.split('-').next().unwrap_or(file_stem);

        match convert_frame_to_gui_sprite(app, frame_sprite) {
            Ok(_gui_sprite) => {
                sprite_list.add_sprite(
                    app,
                    &frame_sprite.path,
                    key,
                    frame_sprite.position.x as f32,
                    frame_sprite.position.y as f32,
                );
            }
            Err(e) => {
                eprintln!("Failed to add sprite: {}", e);
            }
        }
    }

    for frame_sound in &frame.sounds {
        if frame_sound.can_play {
            let audio = Arc::clone(audio);
            let mut frame_sound = frame_sound.clone();
            frame_sound.can_play = false;

            thread_pool.spawn(move || {
                if let Err(e) = play_frame_sound(&frame_sound, &audio, 10, 1.0) {
                    eprintln!("Failed to play sound: {}", e);
                }

                frame_sound.can_play = true;
            });
        } else {
            eprintln!("Sound {} is already playing. Skipping duplicate request.", frame_sound.path);
        }
    }
}
