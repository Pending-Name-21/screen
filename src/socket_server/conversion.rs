use crate::gui::sprite::sprite::Sprite as GuiSprite;
use crate::socket_server::frame::{Sound as FrameSound, Sprite as FrameSprite};
use crate::sound_manager::audio::Audio;
use nannou::image::open;
use nannou::prelude::*;

pub fn convert_frame_to_gui_sprite(app: &App, frame_sprite: &FrameSprite) -> Result<GuiSprite, String> {
    match open(&frame_sprite.file_path) {
        Ok(image) => {
            let texture = wgpu::Texture::from_image(app, &image);
            Ok(GuiSprite {
                position: pt2(frame_sprite.position.x as f32, frame_sprite.position.y as f32),
                texture,
                dimensions: Some(vec2(frame_sprite.size.width as f32, frame_sprite.size.height as f32)),
            })
        },
        Err(_) => Err(format!("Failed to load image: {}", frame_sprite.file_path)),
    }
}

pub fn play_frame_sound(frame_sound: &FrameSound, audio: &Audio) -> Result<(), String> {
    let path = &frame_sound.file_path;
    if std::path::Path::new(path).exists() {
        audio.play(path, 10, 0.5);
        Ok(())
    } else {
        Err(format!("Sound file not found: {}", path))
    }
}