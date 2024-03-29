use crate::{despawn_screen, AppState, Fonts, Scores, GAME_NAME};
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MenuPlugin;

const MENU_MARGIN_PX: f32 = 50.0;

const BUTTON_SIZE_PX: (f32, f32) = (250.0, 65.0);
const BUTTON_MARGIN_PX: f32 = 20.0;

const TEXT_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

const TEXT_PLAY_BUTTON: &str = "Let's Play!";
const TEXT_QUIT_BUTTON: &str = "Quit!";
const TEXT_SCORES_BUTTON: &str = "Scores";
const TEXT_SCORES: &str = "Top scores of all time:";
const TEXT_NO_SCORES: &str = "There are no scores yet!";
const TEXT_BACK_MENU: &str = "Back to menu!";

const TEXT_TITLE_SIZE: f32 = 80.0;
const TEXT_BUTTON_SIZE: f32 = 45.0;
const TEXT_SCORE_SIZE: f32 = 60.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct MainScreen;

#[derive(Component)]
struct ScoresScreen;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
    Disabled,
    Scores,
    Main,
}

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Scores,
    BackToMain,
    Quit,
}

#[derive(Component)]
struct SelectedOption;

fn setup_system(mut menu_state: ResMut<State<MenuState>>) {
    menu_state.set(MenuState::Main).unwrap();
}

// This is intended.
#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn main_system(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.default.clone();
    let button_style = Style {
        size: Size::new(Val::Px(BUTTON_SIZE_PX.0), Val::Px(BUTTON_SIZE_PX.1)),
        margin: UiRect::all(Val::Px(BUTTON_MARGIN_PX)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let title_text_style = TextStyle {
        font: font.clone(),
        font_size: TEXT_TITLE_SIZE,
        color: TEXT_COLOR,
    };

    let button_text_style = TextStyle {
        font,
        font_size: TEXT_BUTTON_SIZE,
        color: TEXT_COLOR,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MainScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(GAME_NAME, title_text_style).with_style(Style {
                    margin: UiRect::all(Val::Px(MENU_MARGIN_PX)),
                    ..default()
                }),
            );
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        TEXT_PLAY_BUTTON.to_string(),
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Scores,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        TEXT_SCORES_BUTTON.to_string(),
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        TEXT_QUIT_BUTTON.to_string(),
                        button_text_style,
                    ));
                });
        });
}

// This is intended.
#[allow(clippy::type_complexity)]
fn action_system(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
    mut game_state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(AppState::Game).unwrap();
                    menu_state.set(MenuState::Disabled).unwrap();
                }
                MenuButtonAction::Scores => menu_state.set(MenuState::Scores).unwrap(),
                MenuButtonAction::BackToMain => menu_state.set(MenuState::Main).unwrap(),
            }
        }
    }
}

fn score_system(mut commands: Commands, fonts: Res<Fonts>, mut scores: ResMut<Scores>) {
    let font = fonts.default.clone();
    let score_text_style = TextStyle {
        font: fonts.default.clone(),
        font_size: TEXT_SCORE_SIZE,
        color: TEXT_COLOR,
    };
    let button_style = Style {
        size: Size::new(Val::Px(BUTTON_SIZE_PX.0), Val::Px(BUTTON_SIZE_PX.1)),
        margin: UiRect::all(Val::Px(BUTTON_MARGIN_PX)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font,
        font_size: TEXT_BUTTON_SIZE,
        color: TEXT_COLOR,
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ScoresScreen,
        ))
        .with_children(|parent| {
            if !scores.score_list.is_empty() {
                parent.spawn(TextBundle::from_section(
                    TEXT_SCORES.to_string(),
                    score_text_style.clone(),
                ));
            } else {
                parent.spawn(TextBundle::from_section(
                    TEXT_NO_SCORES.to_string(),
                    score_text_style.clone(),
                ));
            }
            scores.score_list.sort();
            for position in 0..5 {
                if scores.score_list.len() > position {
                    if let Some(score) = scores
                        .score_list
                        .get(scores.score_list.len() - position - 1)
                    {
                        parent.spawn(TextBundle::from_section(
                            format!("{}.    {}", position + 1, score),
                            score_text_style.clone(),
                        ));
                    }
                }
            }
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::BackToMain,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        TEXT_BACK_MENU.to_string(),
                        button_text_style,
                    ));
                });
        });
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Disabled)
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_system))
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_system))
            .add_system_set(
                SystemSet::on_exit(MenuState::Main).with_system(despawn_screen::<MainScreen>),
            )
            .add_system_set(
                SystemSet::on_exit(MenuState::Scores).with_system(despawn_screen::<ScoresScreen>),
            )
            .add_system_set(SystemSet::on_enter(MenuState::Scores).with_system(score_system))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(action_system)
                    .with_system(button_system),
            );
    }
}
