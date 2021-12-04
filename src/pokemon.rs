use engine::{ Scene, Sprite, SceneFnOutcome };
use sdl2::rect::Rect;
use std::collections::HashMap;

pub struct Line((u32, u32), (u32, u32)); // (pos1, pos2)

// - outside map
// - dialog
// - cutscene
// - battle
// - battle animation
// - house
// - cave

struct Pokemon;

pub struct PokemonTrainer {
    name: String,
    game_sprite: Sprite,
    battle_sprite: Sprite,
    team: [Pokemon; 9],
    computer: [Vec<Pokemon>; 9],
    pokedex: Vec<u16>, // pokemon id
}

pub enum SceneType<'a> {
    Outside {
        background: String,
        sprites: Vec<Sprite>,
        zones: Vec<Zone>,
        clickables: Vec<Clickable>, // clickable positions
        pos: (u32, u32), // start pos of player
    },
    SelectMenu {
        prev: &'a mut Scene,
        options: Vec<(String, MenuOptionCallbackFn)>,
    },
    TeamEditor,
    ComputerEditor,
    Dialog {
        prev: &'a mut Scene,
        dialog: Vec<String>,
        pos: (u32, u32), // pos of the player
    },
    Cutscene {
        prev: &'a mut Scene,
        timeline: Vec<Animation<'a>>,
    },
    Battle {
        background: String,
        opponent: u32, // id of opponent, stored in globals
    },
    BattleMove {
        prev: &'a mut Scene,
        pokemon_move: String,
    },
    Building {
        background: String,
        rect: Rect, // dst_rect
        sprites: Vec<Sprite>,
        zones: Vec<Zone>,
        clickables: Vec<Clickable>,
        pos: (u32, u32),
    },
}

impl SceneType<'_> {
    pub fn to_scene(&self) -> Scene {
        match self {
            Self::Outside { background, sprites, zones, clickables, pos } => {},
            Self::SelectMenu { prev, options } => {},
            Self::TeamEditor => {},
            Self::ComputerEditor => {},
            Self::Dialog { prev, dialog, pos } => {},
            Self::Cutscene { prev, timeline } => {},
            Self::Battle { background, opponent } => {},
            Self::BattleMove { prev, pokemon_move } => {},
            Self::Building { background, rect, sprites, zones, clickables, pos } => {},
        }
    }
}

pub struct SelectMenuOption {
    text: (Option<String>, Option<String>, Option<String>), // (left, mid, right)
    callback: MenuOptionCallbackFn,
}

pub type MenuOptionCallbackFn = fn(scene: &mut Scene) -> SceneFnOutcome;

pub enum Animation<'a> { // measured in milliseconds
    Keyframe {
        time: (u32, u32),
        sprite: &'a mut Sprite,
        pos: (u32, u32), // ending pos
    },
    SpriteChange {
        time: u32,
        sprite: &'a mut Sprite,
        new_texture: String,
    },
}

pub struct Clickable {
    name: String,
    pos: (u32, u32),
    callback: ClickableCallbackFn,
}

pub type ClickableCallbackFn = fn(name: &str, scene: &mut Scene) -> SceneFnOutcome;

/// Rect or line that, if triggered (for a rect by crossing its sides and for a line by crossing
/// it), calls the callback function. Useful for boxes where the player cannot go, such as
/// buildings, and wild areas.
pub enum Zone {
    Rect(Rect, ZoneCallbackFn),
    Line(Line, ZoneCallbackFn),
}

pub type ZoneCallbackFn = fn(zone: &Zone, scene: &mut Scene) -> SceneFnOutcome;

impl Zone {
    /// Checks if the rect going from `start` to `end` entered the rect / crossed the line
    pub fn sprite_triggered(&self, start: Rect, end: Rect) -> bool {
        true // TODO: implement this function
    }
}

