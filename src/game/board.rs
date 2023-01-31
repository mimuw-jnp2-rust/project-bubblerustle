use bevy::prelude::*;

use crate::game::{Wall, BOTTOM, LEFT, RIGHT, TOP, WALL_COLOR, WALL_SIZE};
use crate::{AppState, Fonts};

use super::components::{GameScreen, LivesText, ScoreText};
use super::{
    LIVES_TEXT_COLOR, LIVES_TEXT_DEFAULT, LIVES_TEXT_SIZE, LIVES_TEXT_X, LIVES_TEXT_Y,
    SCORE_TEXT_COLOR, SCORE_TEXT_DEFAULT, SCORE_TEXT_SIZE, SCORE_TEXT_X, SCORE_TEXT_Y,
};
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(add_walls_system)
                .with_system(score_system)
                .with_system(lives_system),
        );
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
    commands.spawn((WallBundle::new(WallLocation::Left), GameScreen));
    commands.spawn((WallBundle::new(WallLocation::Right), GameScreen));
    commands.spawn((WallBundle::new(WallLocation::Bottom), GameScreen));
    commands.spawn((WallBundle::new(WallLocation::Top), GameScreen));
}

fn score_system(mut commands: Commands, fonts: Res<Fonts>) {
    let score_text_style = TextStyle {
        font: fonts.default.clone(),
        font_size: SCORE_TEXT_SIZE,
        color: SCORE_TEXT_COLOR,
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(SCORE_TEXT_X),
                        bottom: Val::Px(SCORE_TEXT_Y),
                        ..default()
                    },
                    margin: UiRect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GameScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(SCORE_TEXT_DEFAULT.to_string(), score_text_style),
                ScoreText,
            ));
        });
}

fn lives_system(mut commands: Commands, fonts: Res<Fonts>) {
    let score_text_style = TextStyle {
        font: fonts.default.clone(),
        font_size: LIVES_TEXT_SIZE,
        color: LIVES_TEXT_COLOR,
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px(LIVES_TEXT_X),
                        bottom: Val::Px(LIVES_TEXT_Y),
                        ..default()
                    },
                    margin: UiRect::all(Val::Auto),
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            GameScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(LIVES_TEXT_DEFAULT.to_string(), score_text_style),
                LivesText,
            ));
        });
}
