use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::MaterialMesh2dBundle;
use bubble::BubblePlugin;
use components::{Bubble, BubbleSize, Hook, Movement, Player, Wall};
use player::PlayerPlugin;
use std::collections::HashSet;
use wall::WallPlugin;

mod bubble;
mod components;
mod player;
mod wall;

// WINDOW CONFIGURATION

const GAME_NAME: &str = "Bubble Rustle!";
const WINDOW_MODE: WindowMode = WindowMode::Fullscreen;
const RESIZABLE: bool = false;

// ASSETS

const PLAYER_FILE: &str = "player.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_SCALE: f32 = 0.5;

const HOOK_FILE: &str = "hook.png";
const HOOK_SIZE: (f32, f32) = (8., 199.);
const HOOK_WIDTH_SCALE: f32 = 1.1;

// COLOR

const BACKGROUND_COLOR: Color = Color::rgb(0.5, 0.45, 0.5);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const BALL_COLOR: Color = Color::rgb(0.01, 0.9, 0.1);

// GAME_CONFIGURATION

const TIME_STEP: f32 = 1. / 60.;
const PLAYER_SPEED: f32 = 300.;
const HOOK_SPEED: f32 = 100.;
const BALL_SPEED_X: f32 = 200.;
const BALL_SLOWDOWN: f32 = 600.;

const BALL_RADIUS: f32 = 10.;

const WALL_SIZE: f32 = 20.;
const LEFT: f32 = -550.;
const RIGHT: f32 = 550.;
const BOTTOM: f32 = -400.;
const TOP: f32 = 400.;

// RESOURCES

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
    hook: Handle<Image>,
}

#[derive(Default)]
struct CollisionEvent;

#[derive(Resource, Default)]
struct PlayerState {
    is_alive: bool,
    hook_shoted: bool,
}

impl PlayerState {
    fn kill(&mut self) {
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
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_FILE),
        hook: asset_server.load(HOOK_FILE),
    };

    commands.insert_resource(game_textures);
}

fn bubble_velocity_system(mut query: Query<(&mut Transform, &mut Movement), With<Bubble>>) {
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

fn bubble_hook_collision_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    hook_query: Query<(Entity, &Transform), With<Hook>>,
    bubble_query: Query<(Entity, &Transform, &BubbleSize), With<Bubble>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                if bubble_size.size > 2. {
                    let new_bubble_size = bubble_size.size - 1.;
                    let (bubble_position_x, bubble_position_y) = (
                        bubble_transform.translation.x,
                        bubble_transform.translation.y,
                    );
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
                        ));
                    }
                }
                player_state.unhook();
            }
        }
    }
}

fn bubble_player_collision_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    bubble_query: Query<&Transform, With<Bubble>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut collision_events: EventWriter<CollisionEvent>,
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
                    break;
                }
            }
        }
    }
}

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
        .add_event::<CollisionEvent>()
        .add_plugin(WallPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BubblePlugin)
        .add_startup_system(setup_system)
        .add_system(rope_hook_system)
        .add_system(hook_wall_collision_system)
        .add_system(bubble_wall_collision_system)
        .add_system(bubble_hook_collision_system)
        .add_system(bubble_player_collision_system)
        .add_system(bubble_velocity_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}
