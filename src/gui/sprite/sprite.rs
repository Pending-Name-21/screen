use nannou::prelude::*;

use crate::gui::loader::load_image;

pub struct Sprite {
    pub position: Point2,
    pub texture: wgpu::Texture,
    pub dimensions: Option<Vec2>,
}

pub fn create_texture_from_image(app: &App, image_file: &str) -> wgpu::Texture {
    let image = load_image::load_image(&image_file);
    wgpu::Texture::from_image(app, &image)
}

pub fn new_sprite(texture: wgpu::Texture, pos_x: f32, pos_y: f32, ) -> Sprite {
    Sprite {
        position: pt2(pos_x, pos_y),
        texture,
        dimensions: None, 
    }
}

pub fn new_with_size(texture: wgpu::Texture, pos_x: f32, pos_y: f32, width: f32, height: f32) -> Sprite {
    Sprite {
        position: pt2(pos_x, pos_y),
        texture,
        dimensions: Some(vec2(width, height)),
    }
}
