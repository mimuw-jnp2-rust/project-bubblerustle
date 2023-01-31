use bevy::prelude::*;
use game::GamePlugin;

mod game;

// WINDOW CONFIGURATION
const GAME_NAME: &str = "Bubble Rustle!";
const WINDOW_MODE: WindowMode = WindowMode::Fullscreen;
const RESIZABLE: bool = false;

// COLOR
const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.45, 0.5);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: GAME_NAME.to_string(),
                mode: WINDOW_MODE,
                resizable: RESIZABLE,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(GamePlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}
