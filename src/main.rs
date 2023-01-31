use bevy::prelude::*;
use game::GamePlugin;
use menu::MenuPlugin;
use splash::SplashPlugin;

mod game;
mod menu;
mod splash;

// WINDOW CONFIGURATION
const GAME_NAME: &str = "Bubble Rustle!";
const WINDOW_MODE: WindowMode = WindowMode::Fullscreen;
const RESIZABLE: bool = false;

// COLOR
const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.45, 0.5);

// FONTS
const TEXT_FONT_FILE: &str = "FiraSans-Bold.ttf";

// RESOURCES
#[derive(Resource, Default)]
struct Scores {
    score_list: Vec<usize>,
}

#[derive(Resource)]
struct Fonts {
    default: Handle<Font>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Splash,
    Menu,
    Game,
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn fonts_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fonts = Fonts {
        default: asset_server.load(TEXT_FONT_FILE),
    };
    commands.insert_resource(fonts);
}

fn main() {
    App::new()
        .add_startup_system(fonts_system)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Scores::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: GAME_NAME.to_string(),
                mode: WINDOW_MODE,
                resizable: RESIZABLE,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_state(AppState::Splash)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}
