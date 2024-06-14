extern crate nannou;

use nannou::image::DynamicImage;
use nannou::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Size {
    pub height: f64,
    pub width: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sound {
    pub file_path: String,
    pub can_play: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sprite {
    pub position: Coord,
    pub size: Size,
    pub is_hidden: bool,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FrameData {
    pub sprite: Option<Sprite>,
    pub sound: Option<Sound>,
}

struct Model {
    receiver: mpsc::Receiver<FrameData>,
    frame_data: Option<FrameData>,
    texture: Option<wgpu::Texture>,
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}

fn model(app: &App) -> Model {
    let (tx, rx) = mpsc::channel();

    // Hilo para el servidor
    thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let tx = tx.clone();
                    thread::spawn(move || {
                        let mut buffer = Vec::new();
                        stream.read_to_end(&mut buffer).unwrap();
                        let data = String::from_utf8(buffer).unwrap();
                        let frame_data: FrameData = serde_json::from_str(&data).unwrap();
                        tx.send(frame_data).unwrap();
                    });
                }
                Err(_) => {
                    eprintln!("Connection failed");
                }
            }
        }
    });

    app.new_window().size(800, 600).view(view).build().unwrap();

    Model { receiver: rx, frame_data: None, texture: None }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Recibir frame desde el canal
    if let Ok(frame_data) = model.receiver.try_recv() {
        if let Some(ref sprite) = frame_data.sprite {
            if let Ok(image) = nannou::image::open(&sprite.file_path) {
                model.texture = Some(wgpu::Texture::from_image(app, &image));
            }
        }
        model.frame_data = Some(frame_data);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    if let Some(ref frame_data) = model.frame_data {
        if let Some(ref sprite) = frame_data.sprite {
            if !sprite.is_hidden {
                draw.rect()
                    .x_y(sprite.position.x as f32, sprite.position.y as f32)
                    .w_h(sprite.size.width as f32, sprite.size.height as f32)
                    .color(WHITE);

                if let Some(ref texture) = model.texture {
                    draw.texture(texture)
                        .x_y(sprite.position.x as f32, sprite.position.y as f32)
                        .w_h(sprite.size.width as f32, sprite.size.height as f32);
                }
            }
        }

        if let Some(ref sound) = frame_data.sound {
            println!("Sound file path: {}", sound.file_path);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
 