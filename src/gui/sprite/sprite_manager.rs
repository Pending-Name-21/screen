use std::collections::HashMap;

use nannou::prelude::*;

use super::sprite::{self, Sprite};

pub struct SpriteList {
    pub sprites: HashMap<String, Sprite>,
}

impl SpriteList{
    pub fn new() -> Self {
        SpriteList {
            sprites: HashMap::new(),
        }
    }

    pub fn add_sprite(&mut self,app: &App, image_file: &str, key_name: &str, pos_x: f32, pos_y: f32 ){
        let texture = sprite::Sprite::create_texture_from_image(app,&image_file);
        let sprite = sprite::Sprite::new_sprite( texture, pos_x,pos_y);
        self.sprites.insert(key_name.to_string(), sprite);

    }

    pub fn add_sprite_dim(&mut self, app: &App, image_file: &str, key_name: &str, pos_x: f32, pos_y: f32, width: f32, height: f32){
        let texture = sprite::Sprite::create_texture_from_image(app,&image_file);
        let sprite_with_size = sprite::Sprite::new_with_size(texture, pos_x, pos_y, width, height);
        self.sprites.insert(key_name.to_string(), sprite_with_size);
    }

    pub fn update_sprite(&mut self, key: &str, pos_x: f32, pos_y: f32, width: Option<f32>, height: Option<f32>) {
        if let Some(sprite) = self.sprites.get_mut(key) {
            sprite.position = pt2(pos_x, pos_y);
            if let (Some(w), Some(h)) = (width, height) {
                sprite.dimensions = Some(vec2(w, h));
            }
        }
    }

    pub fn remove_sprite(&mut self, key: &str) {
        self.sprites.remove(key);
    }
}