use game::{ Scene, Sprite };
use sdl2::rect::Rect;

pub struct Line((u32, u32), (u32, u32)); // (pos1, pos2)

// - outside map
// - dialog
// - cutscene
// - battle
// - battle animation
// - house
// - cave
impl Scene {}

pub enum SceneType {
    Outside {
        map: String,
        sprites: Vec<String>,
    },
}

pub enum Zone {
    Rect(Sprite),
    Line(Line),
}

impl Zone {
    /// Checks if the rect going from `start` to `end` entered the rect / crossed the line
    pub fn sprite_triggered(&self, start: Rect, end: Rect) -> bool {
        true // TODO: implement this function
    }
}

