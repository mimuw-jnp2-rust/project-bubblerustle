use bevy::prelude::Component;

#[derive(Component)]
pub struct GameScreen;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Hook;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Bubble;

#[derive(Component)]
pub struct Reward;

#[derive(Component)]
pub struct RewardScore {
    pub score: usize,
}

#[derive(Component)]
pub struct BubbleSize {
    pub size: f32,
}

#[derive(Component)]
pub struct Movement {
    pub v_x: f32,
    pub v_y: f32,
    pub a: f32,
}
