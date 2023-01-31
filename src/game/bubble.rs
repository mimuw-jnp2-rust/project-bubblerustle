use crate::game::components::{Bubble, BubbleSize, Movement};
use crate::game::{BubbleState, BALL_COLOR, BALL_RADIUS, BALL_SLOWDOWN, BALL_SPEED_X, TIME_STEP};
use crate::AppState;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::time::FixedTimestep;
pub struct BubblePlugin;

impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BubbleState::default())
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(bubble_spawn_system))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
            );
    }
}

fn bubble_spawn_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bubble_state: ResMut<BubbleState>,
) {
    if !bubble_state.spawned {
        for i in 0..bubble_state.count {
            let (bubble_position_x, bubble_position_y) = *bubble_state.positions.get(i).unwrap();
            let bubble_size = *bubble_state.sizes.get(i).unwrap();
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::default().into()).into(),
                    material: materials.add(ColorMaterial::from(BALL_COLOR)),
                    transform: Transform {
                        translation: Vec3::new(bubble_position_x, bubble_position_y, 0.),
                        scale: Vec3::new(BALL_RADIUS * bubble_size, BALL_RADIUS * bubble_size, 0.),
                        ..default()
                    },
                    ..default()
                },
                Bubble,
                Movement {
                    v_x: BALL_SPEED_X,
                    v_y: BALL_SPEED_X * bubble_size,
                    a: BALL_SLOWDOWN,
                },
                BubbleSize { size: bubble_size },
            ));
        }

        bubble_state.spawn();
    }
}
