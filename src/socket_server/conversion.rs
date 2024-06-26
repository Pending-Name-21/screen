use crate::gui::sprite::sprite::Sprite as GuiSprite;
use crate::socket_server::frame::{Sound as FrameSound, Sprite as FrameSprite};
use crate::sound_manager::audio::Audio;
use nannou::image::open;
use nannou::prelude::*;

pub fn convert_frame_to_gui_sprite(app: &App, frame_sprite: &FrameSprite) -> Result<GuiSprite, String> {
    match open(&frame_sprite.path) {
        Ok(image) => {
            let texture = wgpu::Texture::from_image(app, &image);
            Ok(GuiSprite {
                position: pt2(frame_sprite.position.x as f32, frame_sprite.position.y as f32),
                texture,
                dimensions: Some(vec2(frame_sprite.size.width as f32, frame_sprite.size.height as f32)),
            })
        },
        Err(_) => Err(format!("Failed to load image: {}", frame_sprite.path)),
    }
}

pub fn play_frame_sound(frame_sound: &FrameSound, audio: &Audio, duration: u64, volume: f32) -> Result<(), String> {
    let path = &frame_sound.path;
    if std::path::Path::new(path).exists() {
        audio.play(path, duration, volume);
        Ok(())
    } else {
        Err(format!("Sound file not found: {}", path))
    }
}

pub fn play_frame_playlist(frame_sounds: Vec<FrameSound>, audio: &Audio, loop_playlist: bool, random: bool, volume: f32) -> Result<(), String> {
    let paths: Vec<String> = frame_sounds.iter().map(|sound| sound.path.clone()).collect();
    audio.play_playlist(paths, loop_playlist, random, volume);
    Ok(())
}

pub fn play_frame_simple(frame_sound: &FrameSound) -> Result<(), String> {
    let path = &frame_sound.path;
    if std::path::Path::new(path).exists() {
        Audio::play_simple(path);
        Ok(())
    } else {
        Err(format!("Sound file not found: {}", path))
    }
}