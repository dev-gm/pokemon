use std::collections::HashMap;
use std::time::Duration;
use sdl2::{
    Sdl,
    VideoSubsystem,
};
use sdl2::event::{
    Event,
    EventType,
};
use sdl2::render::{
    WindowCanvas,
    TextureCreator,
    Texture,
};
use sdl2::video::{
    WindowContext,
    Window,
};
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::image::LoadTexture;
use crate::stack::Stack;
use crate::dict::*;

/// Holds basic info for a `Engine`, such as title, time between frames, size, scale, etc
pub struct EngineInfo<'a> {
    title: &'a str,
    delay: u32,
    size: (u32, u32),
    scale: (f32, f32),
}

impl<'a> EngineInfo<'a> {
    pub fn new(title: &'a str, delay: u32, size: (u32, u32), scale: (f32, f32)) -> Self {
        Self { title, delay, size, scale }
    }
}

/// Is responsible for rendering the game, holding sprites (with a spritesheet), holding/managing
/// game state, and managing various `Scene`s (held together with a stack)
pub struct Engine<'a> {
    info: EngineInfo<'a>,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    canvas: WindowCanvas,
    texture_creator: TextureCreator<WindowContext>,
    handle_quit: HandleQuitFn, // when lone scene on stack quits, this fn is called
    globals: Dict,
    spritesheet: SpriteSheet<'a>,
    stack: Stack<Scene>,
}

impl<'a> Engine<'a> {
    /// Sets up SDL2 context and returns a new `Engine` from args
    pub fn new(
        info: EngineInfo<'a>,
        handle_quit: HandleQuitFn,
        globals: Dict,
        spritesheet: &str,
        index: HashMap<String, Rect>
    ) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(info.title, info.size.0, info.size.1)
            .position_centered()
            .build()
            .or_else(|err| Err(format!("{}", err)))?;
        let mut canvas = window
            .into_canvas()
            .build()
            .or_else(|err| Err(format!("{}", err)))?;
        canvas.set_scale(info.scale.0, info.scale.1);
        let texture_creator = canvas.texture_creator();
        Ok(Self {
            info,
            sdl_context,
            video_subsystem,
            window,
            texture_creator,
            canvas,
            handle_quit,
            globals,
            spritesheet: SpriteSheet::new(
                texture_creator.load_texture(spritesheet)?,
                index,
            ),
            stack: Stack::new(),
        })
    }

    /// Runs the engine and then consumes itself, returning a game-specified `Dict` or an error
    pub fn run(mut self) -> Dict {
        let event_pump = self.sdl_context.event_pump().unwrap(); // THIS IS NOT SAFE
        'running: loop {
            if let Some(scene) = self.stack.peek() {
                scene.render(&mut self.canvas, &self.spritesheet);
            }
            if let Some(scene) = self.stack.peek_mut() {
                for event in event_pump.poll_iter() {
                    if let Some(callback) = scene.event_callbacks.get(&EventType::from(event.to_ll().unwrap().r#type)) {
                        if let Some(exit_props) = self.handle_scene_fn_outcome(callback(scene, &event)) {
                            break 'running exit_props;
                        }
                    }
                }
            } else {
                break 'running HashMap::new(); // TODO: MAKE ACTUAL ERROR MSG
            }
            ::std::thread::sleep(Duration::new(0, self.info.delay * 1E6 as u32)); // does this work?
            if let Some(scene) = self.stack.peek_mut() {
                (scene.on_tick)(scene, self.info.delay);
            } else {
                break 'running HashMap::new(); // TODO: MAKE ACTUAL ERROR MSG
            }
        }
    }

    /// Handles the outcome of a scene callback function (e.g. event callbacks, etc). Is needed
    /// because these functions have the ability to create a child scene, replace itself on the
    /// stack, and delete itself on the stack, which all require `Engine` level privileges.
    fn handle_scene_fn_outcome(&mut self, outcome: SceneFnOutcome) -> Option<Dict> { // None=continue, Some(props)=exit with props
        match outcome {
            SceneFnOutcome::CreateChild { create_scene, props } => {
                self.handle_props(props);
                self.stack.push(create_scene(props));
            },
            SceneFnOutcome::Replace {create_scene, props } => {
                self.handle_props(props);
                self.stack.replace(create_scene(props));
            },
            SceneFnOutcome::Quit(props) => {
                self.stack.pop();
                self.handle_props(props);
                if let Some(parent) = self.stack.peek_mut() {
                    return self.handle_scene_fn_outcome((parent.on_child_quit)(parent, props));
                } else {
                    return Some((self.handle_quit)(self, props));
                }
            },
            _ => {},
        }
        None
    }

    /// Processes props passed from a scene to another scene via a scene callback function. For
    /// global objects (stored in `engine.globals` and are useful for storing things such as a
    /// player object), the caller scene can request, via the props["_REQUESTS"] array, for some
    /// global to be passed to the callee. The globals retrieved will be stored in props["globals"].
    fn handle_props(&self, props: &mut Dict) {
        let globals = HashMap::new();
        if let Some(DictValue::Array(requests)) = props.get("_REQUESTS") {
            for raw_request in requests {
                if let DictValue::String(request) = raw_request {
                    if let Some(object) = self.globals.get(request) {
                        globals.insert(request, object);
                    }
                }
            }
            props.delete("_REQUESTS");
        }
        props.insert("globals", globals);
    }
}

/// Handles what happens if the game quits (the only item on the scene stack quits). Takes in props
/// passed by the last item on the stack quitting, and returns the engine's return value.
pub type HandleQuitFn = fn(engine: &mut Engine, props: Dict) -> Dict;

/// A texture with all the sprites in the game. Has an index that holds the src_rects for all
/// sprites.
pub struct SpriteSheet<'a> {
    pub texture: Texture<'a>,
    index: HashMap<String, Rect>,
}

impl<'a> SpriteSheet<'a> {
    pub fn new(texture: Texture<'a>, index: HashMap<String, Rect>) -> Self {
        Self { texture, index }
    }

    /// Retrieves src_rect for given sprite name
    pub fn get(&self, key: String) -> Option<&Rect> {
        self.index.get(&key)
    }
}

/// In different parts of a game, there will be different sprites, backgrounds, and ways the game
/// responds to events happening and time passing. For example, in a pokemon game, a user be on a
/// map and then enter a battle. These two parts of the game respond completely differently to
/// input events and time passing, and have completely different backgrounds and sprites, and so
/// therefore need different scenes.
pub struct Scene {
    background: String,
    state: Dict,
    sprites: Vec<Sprite>,
    event_callbacks: HashMap<EventType, EventCallbackFn>,
    on_tick: SceneOnTickFn,
    on_child_quit: SceneOnChildQuitFn,
}

impl Scene {
    fn render(&self, canvas: &mut WindowCanvas, spritesheet: &SpriteSheet) -> Result<(), String> {
        for sprite in self.sprites {
            match sprite {
                Sprite::Texture { rect: dst_rect, sprite: sprite_name } => {
                    let src_rect = *spritesheet.get(sprite_name)
                        .ok_or(format!("Sprite {} doesn't exist on spritesheet", sprite_name))?;
                    canvas.copy(&spritesheet.texture, src_rect, dst_rect)?;
                },
                Sprite::Rect { rect, color } => {
                    canvas.set_draw_color(color);
                    canvas.draw_rect(rect);
                    canvas.fill_rect(rect);
                },
            }
        }
        Ok(())
    }
}

/// Is called when a specified event type occurs
pub type EventCallbackFn = fn(scene: &mut Scene, event: &Event) -> SceneFnOutcome;
/// Is called between every frame. `interval` is the time that has passed since function was last
/// called. Useful for cutscenes or other scenes based on time passing instead of events.
pub type SceneOnTickFn = fn(scene: &mut Scene, interval: u32) -> SceneFnOutcome;
/// Is called when a child scene quits, and the responsibility for managing the game returns to
/// this scene.
pub type SceneOnChildQuitFn = fn(scene: &mut Scene, props: Dict) -> SceneFnOutcome;

/// Scenes are stored on a stack. Whenever a game switches from one scene to another, but requires
/// the player to go back to the old scene eventually, and so doesn't want to discard all of the
/// data, it can push the scene to the stack. In a pokemon game, an example of this is entering a
/// battle. If the player will not go back to the old scene eventually (e.g. leaving a route in a
/// pokemon game), then the scene gets replaced on the stack. These actions can be decided upon
/// when an event happens or when time passes, so event callbacks and the on_tick function can
/// return an enum that does these actions.
pub enum SceneFnOutcome {
    Continue,
    CreateChild {
        create_scene: fn(props: Dict) -> Scene,
        props: Dict,
    },
    Replace {
        create_scene: fn(props: Dict) -> Scene,
        props: Dict,
    },
    Quit(Dict), // quit scene, send Dict props to scene above
}

/// A sprite can either be rendered using a `Texture` or a `Color`. Both variants contain the rect
/// value, or where on the screen will the sprite be rendered. The Texture variant includes a name
/// of a sprite on the spritesheet, and the Rect variant includes a color that will be rendered on
/// to the rect.
pub enum Sprite {
    Texture {
        rect: Rect,
        sprite: String, // location on spritesheet
    },
    Rect {
        rect: Rect,
        color: Color,
    },
}


