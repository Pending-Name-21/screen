#[derive(Debug, Clone)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub height: f64,
    pub width: f64,
}

#[derive(Debug, Clone)]
pub struct Sound {
    pub can_play: bool,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Sprite {
    pub position: Coord,
    pub size: Size,
    pub is_hidden: bool,
    pub path: String,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub sprites: Vec<Sprite>,
    pub sounds: Vec<Sound>,
}