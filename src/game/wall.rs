use bevy::prelude::*;

use crate::game::{Wall, BOTTOM, LEFT, RIGHT, TOP, WALL_COLOR, WALL_SIZE};
use crate::AppState;
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(add_walls_system));
    }
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT, 0.),
            WallLocation::Right => Vec2::new(RIGHT, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM),
            WallLocation::Top => Vec2::new(0., TOP),
        }
    }

    fn size(&self) -> Vec2 {
        let height = TOP - BOTTOM;
        let width = RIGHT - LEFT;
        assert!(height > 0.0);
        assert!(width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => Vec2::new(WALL_SIZE, height + WALL_SIZE),
            WallLocation::Bottom | WallLocation::Top => Vec2::new(width + WALL_SIZE, WALL_SIZE),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    wall: Wall,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            wall: Wall,
        }
    }
}

fn add_walls_system(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
