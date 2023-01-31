use bevy::prelude::*;
use crate::{despawn_screen, AppState};

pub struct SplashPlugin;

const LOGO_FILE: &str = "logo.png";
const LOGO_SCALE: f32 = 0.5;

const SPLASH_TIME: f32 = 2.0;

#[derive(Component)]
struct SplashScreen;


#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(splash_setup))
        .add_system_set(SystemSet::on_update(AppState::Splash).with_system(countdown))
        .add_system_set(
            SystemSet::on_exit(AppState::Splash)
                .with_system(despawn_screen::<SplashScreen>),
        );
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let logo = asset_server.load(LOGO_FILE);

    commands.spawn((
        ImageBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            image: UiImage(logo),
            transform: Transform {
                scale: Vec3::new(LOGO_SCALE, LOGO_SCALE, 1.),
                ..default()
            },
            ..default()
        },
        SplashScreen,
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(SPLASH_TIME, TimerMode::Once)));

}

fn countdown(
    mut game_state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::Game).unwrap();
    }
}