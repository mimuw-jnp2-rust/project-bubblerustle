use crate::{despawn_screen, AppState, Scores};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::MaterialMesh2dBundle;
use board::BoardPlugin;
use bubble::BubblePlugin;
use components::{Bubble, BubbleSize, GameScreen, Hook, Movement, Player, Reward, Wall};
use player::PlayerPlugin;
use std::collections::HashSet;

use self::components::{LivesText, RewardScore, ScoreText};

mod board;
mod bubble;
mod components;
mod player;

pub struct GamePlugin;

// ASSETS
const PLAYER_FILE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_SCALE: f32 = 0.5;

const HOOK_FILE: &str = "hook.png";
const HOOK_SIZE: (f32, f32) = (8., 199.);
const HOOK_WIDTH_SCALE: f32 = 1.1;

// COLOR
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BALL_COLOR: Color = Color::rgb(0.01, 0.9, 0.1);
const REWARD_COLOR: Color = Color::GOLD;
const SCORE_TEXT_COLOR: Color = Color::GOLD;
const LIVES_TEXT_COLOR: Color = Color::GREEN;

// GAME_CONFIGURATION
const LIVE_COUNT: usize = 3;

const TIME_STEP: f32 = 1. / 60.;
const PLAYER_SPEED: f32 = 300.;
const HOOK_SPEED: f32 = 100.;
const BALL_SPEED_X: f32 = 200.;
const BALL_SLOWDOWN: f32 = 600.;
const REWARD_SPEED: f32 = 300.;
const REWARD_MAX: usize = 1200;

const BALL_RADIUS: f32 = 10.;
const REWARD_SIZE: f32 = 15.;

const WALL_SIZE: f32 = 20.;
const LEFT: f32 = -550.;
const RIGHT: f32 = 550.;
const BOTTOM: f32 = -400.;
const TOP: f32 = 400.;

const LIVES_TEXT_DEFAULT: &str = "Lives: 3";
const LIVES_TEXT_X: f32 = -225.;
const LIVES_TEXT_Y: f32 = 450.;
const LIVES_TEXT_SIZE: f32 = 50.0;

const SCORE_TEXT_DEFAULT: &str = "Score: 0";
const SCORE_TEXT_X: f32 = 225.;
const SCORE_TEXT_Y: f32 = 450.;
const SCORE_TEXT_SIZE: f32 = 40.0;

// RESOURCES
#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    hook: Handle<Image>,
}

#[derive(Default)]
struct CollisionEvent;

#[derive(Resource, Default)]
struct Score {
    score: usize,
}

#[derive(Resource)]
struct PlayerState {
    lives: usize,
    is_alive: bool,
    hook_shoted: bool,
}

impl PlayerState {
    fn kill(&mut self) {
        self.lives -= 1;
        self.is_alive = false;
    }
    fn spawn(&mut self) {
        self.is_alive = true;
    }

    fn shoot_hook(&mut self) {
        self.hook_shoted = true;
    }

    fn unhook(&mut self) {
        self.hook_shoted = false;
    }

    fn is_completely_dead(&mut self) -> bool {
        self.lives == 0
    }

    fn restart(&mut self) {
        self.lives = LIVE_COUNT;
        self.is_alive = false;
        self.hook_shoted = false;
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            lives: LIVE_COUNT,
            is_alive: false,
            hook_shoted: false,
        }
    }
}

#[derive(Resource)]
struct BubbleState {
    count: usize,
    sizes: Vec<f32>,
    positions: Vec<(f32, f32)>,
    spawned: bool,
}

impl Default for BubbleState {
    fn default() -> Self {
        Self {
            count: 1,
            spawned: false,
            sizes: vec![4.],
            positions: vec![(0., 0.)],
        }
    }
}

impl BubbleState {
    fn spawn(&mut self) {
        self.spawned = true;
    }

    fn despawn(&mut self) {
        self.spawned = false;
    }

    fn restart(&mut self) {
        self.count = 1;
        self.spawned = false;
        self.sizes = vec![4.];
        self.positions = vec![(0., 0.)];
    }
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_FILE),
        hook: asset_server.load(HOOK_FILE),
    };

    commands.insert_resource(game_textures);
}

fn velocity_system(mut query: Query<(&mut Transform, &mut Movement), Without<Hook>>) {
    for (mut transform, mut movement) in &mut query {
        transform.translation.x += movement.v_x * TIME_STEP;
        transform.translation.y += movement.v_y * TIME_STEP;
        movement.v_y -= movement.a * TIME_STEP;
    }
}

fn rope_hook_system(mut query: Query<(&mut Transform, &Movement), With<Hook>>) {
    if let Ok((mut transform, movement)) = query.get_single_mut() {
        transform.translation.y += movement.v_y * TIME_STEP;
        let rope_height = transform.scale.y * HOOK_SIZE.1;
        transform.scale.y *= (rope_height + 2. * movement.v_y * TIME_STEP) / rope_height;
    }
}

fn hook_wall_collision_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    hook_query: Query<(Entity, &Transform), With<Hook>>,
    wall_query: Query<&Transform, With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut despawned_entities = HashSet::new();

    for (hook_entity, hook_transform) in hook_query.iter() {
        if despawned_entities.contains(&hook_entity) {
            continue;
        }
        for wall_transform in wall_query.iter() {
            if despawned_entities.contains(&hook_entity) {
                continue;
            }

            let collision = collide(
                hook_transform.translation,
                Vec2::new(
                    hook_transform.scale.x * HOOK_SIZE.0,
                    hook_transform.scale.y * HOOK_SIZE.1,
                ),
                wall_transform.translation,
                Vec2::new(wall_transform.scale.x, wall_transform.scale.y),
            );

            if collision.is_some() {
                collision_events.send_default();
                commands.entity(hook_entity).despawn();
                despawned_entities.insert(hook_entity);
                player_state.unhook();
            }
        }
    }
}

fn bubble_wall_collision_system(
    mut bubble_query: Query<(&mut Movement, &Transform, &BubbleSize), With<Bubble>>,
    wall_query: Query<&Transform, With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut bubble_movement, bubble_transform, bubble_size) in bubble_query.iter_mut() {
        for wall_transform in wall_query.iter() {
            let collision = collide(
                bubble_transform.translation,
                bubble_transform.scale.truncate(),
                wall_transform.translation,
                wall_transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                collision_events.send_default();

                match collision {
                    Collision::Left => {
                        bubble_movement.v_x = -bubble_movement.v_x;
                    }
                    Collision::Right => {
                        bubble_movement.v_x = -bubble_movement.v_x;
                    }
                    Collision::Top => {
                        bubble_movement.v_y = BALL_SPEED_X * bubble_size.size;
                    }
                    Collision::Bottom => {
                        bubble_movement.v_y = -bubble_movement.v_y;
                    }
                    Collision::Inside => {}
                }
            }
        }
    }
}

// This is intended.
#[allow(clippy::too_many_arguments)]
fn bubble_hook_collision_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    hook_query: Query<(Entity, &Transform), With<Hook>>,
    bubble_query: Query<(Entity, &Transform, &BubbleSize), With<Bubble>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bubble_state: ResMut<BubbleState>,
    mut game_state: ResMut<State<AppState>>,
    mut scores: ResMut<Scores>,
    current_score: Res<Score>,
) {
    let mut despawned_entities = HashSet::new();
    for (hook_entity, hook_transform) in hook_query.iter() {
        if despawned_entities.contains(&hook_entity) {
            continue;
        }
        for (bubble_entity, bubble_transform, bubble_size) in bubble_query.iter() {
            if despawned_entities.contains(&hook_entity)
                || despawned_entities.contains(&bubble_entity)
            {
                continue;
            }

            let collision = collide(
                bubble_transform.translation,
                bubble_transform.scale.truncate(),
                hook_transform.translation,
                Vec2::new(
                    hook_transform.scale.x * HOOK_SIZE.0,
                    hook_transform.scale.y * HOOK_SIZE.1,
                ),
            );

            if collision.is_some() {
                collision_events.send_default();
                commands.entity(hook_entity).despawn();
                despawned_entities.insert(hook_entity);
                commands.entity(bubble_entity).despawn();
                let (bubble_position_x, bubble_position_y) = (
                    bubble_transform.translation.x,
                    bubble_transform.translation.y,
                );
                bubble_state.count -= 1;
                if bubble_size.size > 2. {
                    bubble_state.count += 2;
                    let new_bubble_size = bubble_size.size - 1.;
                    for a in 0..2 {
                        let direction = if a == 0 { -1. } else { 1. };
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::default().into()).into(),
                                material: materials.add(ColorMaterial::from(BALL_COLOR)),
                                transform: Transform {
                                    translation: Vec3::new(
                                        bubble_position_x,
                                        bubble_position_y,
                                        0.,
                                    ),
                                    scale: Vec3::new(
                                        BALL_RADIUS * new_bubble_size,
                                        BALL_RADIUS * new_bubble_size,
                                        0.,
                                    ),
                                    ..default()
                                },
                                ..default()
                            },
                            Bubble,
                            Movement {
                                v_x: direction * BALL_SPEED_X,
                                v_y: BALL_SPEED_X * new_bubble_size,
                                a: BALL_SLOWDOWN,
                            },
                            BubbleSize {
                                size: new_bubble_size,
                            },
                            GameScreen,
                        ));
                    }
                }
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Quad::default().into()).into(),
                        material: materials.add(ColorMaterial::from(REWARD_COLOR)),
                        transform: Transform {
                            translation: Vec3::new(bubble_position_x, bubble_position_y, 0.),
                            scale: Vec3::new(REWARD_SIZE, REWARD_SIZE, 0.),
                            ..default()
                        },
                        ..default()
                    },
                    Movement {
                        v_x: 0.,
                        v_y: -REWARD_SPEED,
                        a: 0.,
                    },
                    Reward,
                    RewardScore {
                        score: REWARD_MAX / (bubble_size.size as usize),
                    },
                    GameScreen,
                ));
                player_state.unhook();
            }
        }
    }
    if bubble_state.count == 0 {
        game_state.set(AppState::Menu).unwrap();
        bubble_state.despawn();
        scores.score_list.push(current_score.score);
        player_state.restart();
        bubble_state.restart();
    }
}

// This is intended.
#[allow(clippy::too_many_arguments)]
fn bubble_player_collision_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    current_score: ResMut<Score>,
    bubble_query: Query<&Transform, With<Bubble>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut game_state: ResMut<State<AppState>>,
    mut bubble_state: ResMut<BubbleState>,
    mut scores: ResMut<Scores>,
    mut lives_text_query: Query<&mut Text, With<LivesText>>,
) {
    if player_state.is_alive {
        if let Ok((player_entity, player_transform)) = player_query.get_single() {
            for bubble_transform in bubble_query.iter() {
                let collision = collide(
                    player_transform.translation,
                    Vec2::new(
                        player_transform.scale.x * PLAYER_SIZE.0,
                        player_transform.scale.y * PLAYER_SIZE.1,
                    ),
                    bubble_transform.translation,
                    bubble_transform.scale.truncate(),
                );

                if collision.is_some() {
                    collision_events.send_default();
                    commands.entity(player_entity).despawn();
                    player_state.kill();

                    if let Ok(mut lives_text) = lives_text_query.get_single_mut() {
                        lives_text.sections[0].value = format!("Lives: {}", player_state.lives);
                    }

                    if player_state.is_completely_dead() {
                        game_state.set(AppState::Menu).unwrap();
                        bubble_state.despawn();
                        scores.score_list.push(current_score.score);
                        player_state.restart();
                        bubble_state.restart();
                    }
                    break;
                }
            }
        }
    }
}

fn reward_player_collision_system(
    mut commands: Commands,
    player_state: ResMut<PlayerState>,
    reward_query: Query<(Entity, &Transform, &RewardScore), With<Reward>>,
    player_query: Query<&Transform, With<Player>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut current_score: ResMut<Score>,
    mut score_text_query: Query<&mut Text, With<ScoreText>>,
) {
    if player_state.is_alive {
        if let Ok(player_transform) = player_query.get_single() {
            for (reward_entity, reward_transform, reward_score) in reward_query.iter() {
                let collision = collide(
                    player_transform.translation,
                    Vec2::new(
                        player_transform.scale.x * PLAYER_SIZE.0,
                        player_transform.scale.y * PLAYER_SIZE.1,
                    ),
                    reward_transform.translation,
                    reward_transform.scale.truncate(),
                );

                if collision.is_some() {
                    collision_events.send_default();
                    commands.entity(reward_entity).despawn();
                    current_score.score += reward_score.score;
                    if let Ok(mut score_text) = score_text_query.get_single_mut() {
                        score_text.sections[0].value = format!("Score: {}", current_score.score);
                    }
                    break;
                }
            }
        }
    }
}

fn reward_wall_collision_system(
    mut commands: Commands,
    wall_query: Query<&Transform, With<Wall>>,
    mut reward_query: Query<(Entity, &Transform), With<Reward>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (reward_entity, reward_transform) in reward_query.iter_mut() {
        for wall_transform in wall_query.iter() {
            let collision = collide(
                reward_transform.translation,
                reward_transform.scale.truncate(),
                wall_transform.translation,
                wall_transform.scale.truncate(),
            );

            if collision.is_some() {
                collision_events.send_default();
                commands.entity(reward_entity).despawn();
                break;
            }
        }
    }
}

fn score_system(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_plugin(BoardPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(BubblePlugin)
            .add_startup_system(setup_system)
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(score_system))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(rope_hook_system)
                    .with_system(hook_wall_collision_system)
                    .with_system(bubble_wall_collision_system)
                    .with_system(bubble_hook_collision_system)
                    .with_system(bubble_player_collision_system)
                    .with_system(reward_player_collision_system)
                    .with_system(reward_wall_collision_system)
                    .with_system(velocity_system),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Game).with_system(despawn_screen::<GameScreen>),
            );
    }
}
