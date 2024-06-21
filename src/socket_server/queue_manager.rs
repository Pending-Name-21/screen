use nannou::prelude::*;
use crate::socket_server::frame::Frame;
use crate::socket_server::conversion::{convert_frame_to_gui_sprite, play_frame_sound};
use crate::gui::sprite::sprite_manager::SpriteList;
use crate::sound_manager::audio::Audio;
use std::sync::Arc;
use rayon::ThreadPool;

pub fn process_frame(app: &App, sprite_list: &mut SpriteList, audio: &Arc<Audio>, thread_pool: &Arc<ThreadPool>, frame: Frame) {
    if let Some(ref frame_sprite) = frame.sprite {
        if let Err(e) = convert_frame_to_gui_sprite(app, frame_sprite).and_then(|_gui_sprite| {
            sprite_list.add_sprite(
                app,
                &frame_sprite.file_path,
                "received_sprite",
                frame_sprite.position.x as f32,
                frame_sprite.position.y as f32,
            );
            Ok(())
        }) {
            eprintln!("Failed to add sprite: {}", e);
        }
    }

    if let Some(ref frame_sound) = frame.sound {
        let audio = Arc::clone(audio);
        let frame_sound = frame_sound.clone();
        thread_pool.spawn(move || {
            if let Err(e) = play_frame_sound(&frame_sound, &audio) {
                eprintln!("Failed to play sound: {}", e);
            }
        });
    }
}
