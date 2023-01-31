use crate::game::components::{Hook, Movement, Player};
use crate::game::{
    GameTextures, PlayerState, BOTTOM, HOOK_SIZE, HOOK_SPEED, HOOK_WIDTH_SCALE, LEFT, PLAYER_SCALE,
    PLAYER_SIZE, PLAYER_SPEED, RIGHT, TIME_STEP, WALL_SIZE,
};
use crate::AppState;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::on_in_stack_update(AppState::Game).with_system(spawn_player_system),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(move_player_system)
                    .with_system(shot_player_system),
            );
    }
}

fn spawn_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    game_textures: Res<GameTextures>,
) {
    if !player_state.is_alive {
        let player_y_pos = BOTTOM + WALL_SIZE / 2. + PLAYER_SIZE.1 / 2. * PLAYER_SCALE;
        commands.spawn((
            SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(0.0, player_y_pos, 0.),
                    scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                    ..default()
                },
                ..default()
            },
            Player,
        ));

        player_state.spawn();
    }
}

fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut player_transform) = query.get_single_mut() {
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let new_player_position_x =
            player_transform.translation.x + direction * PLAYER_SPEED * TIME_STEP;

        let left_bound = LEFT + PLAYER_SIZE.0 / 2. * PLAYER_SCALE + WALL_SIZE / 2.;

        let right_bound = RIGHT - PLAYER_SIZE.0 / 2. * PLAYER_SCALE - WALL_SIZE / 2.;

        player_transform.translation.x = new_player_position_x.clamp(left_bound, right_bound);
    }
}

fn shot_player_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_state: ResMut<PlayerState>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = query.get_single() {
        if keyboard_input.just_pressed(KeyCode::Space) && !player_state.hook_shoted {
            let (x_pos, y_pos) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
            commands.spawn((
                SpriteBundle {
                    texture: game_textures.hook.clone(),
                    transform: Transform {
                        translation: Vec3::new(x_pos, y_pos, 0.),
                        scale: Vec3::new(
                            HOOK_WIDTH_SCALE,
                            PLAYER_SIZE.1 * PLAYER_SCALE / 2. / HOOK_SIZE.1,
                            1.,
                        ),
                        ..default()
                    },
                    ..default()
                },
                Hook,
                Movement {
                    v_x: 0.,
                    v_y: HOOK_SPEED,
                    a: 0.,
                },
            ));
            player_state.shoot_hook();
        }
    }
}
