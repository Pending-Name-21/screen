use crate::coffee_time::output::frame::frame_generated::coffee_time::output::frame::root_as_frame;

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

pub fn deserialize_frame(buffer: &[u8]) -> Result<Frame, String> {
    let flat_frame = root_as_frame(buffer)
        .map_err(|e| format!("Failed to get root as Frame: {:?}", e))?;

    let sounds = flat_frame.sounds().ok_or_else(|| "No sounds found".to_string())?;
    let sounds_vec: Vec<Sound> = sounds.iter().map(|sound| Sound {
        can_play: sound.can_play(),
        path: sound.path().unwrap_or_default().to_string(),
    }).collect();

    let sprites = flat_frame.sprites().ok_or_else(|| "No sprites found".to_string())?;
    let sprites_vec: Vec<Sprite> = sprites.iter().map(|sprite| {
        let position = sprite.position().ok_or_else(|| "Sprite position not found".to_string())?;
        let size = sprite.size_().ok_or_else(|| "Sprite size not found".to_string())?;
        Ok(Sprite {
            position: Coord {
                x: position.x(),
                y: position.y(),
            },
            size: Size {
                height: size.height(),
                width: size.width(),
            },
            is_hidden: sprite.is_hidden(),
            path: sprite.path().unwrap_or_default().to_string(),
        })
    }).collect::<Result<Vec<Sprite>, String>>()?;

    Ok(Frame {
        sounds: sounds_vec,
        sprites: sprites_vec,
    })
}
