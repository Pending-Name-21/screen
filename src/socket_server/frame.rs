use serde::{Deserialize, Serialize};

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
pub struct Frame {
    pub sprite: Option<Sprite>,
    pub sound: Option<Sound>,
}
