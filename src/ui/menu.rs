use bevy::{app::AppExit, prelude::*};

use crate::core::AppState;

// Tag component used to tag entities added on a screen
#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Default)]
pub enum MenuState {
    #[default]
    Main,
    Settings,
    Disabled,
}

// https://bevyengine.org/examples/UI%20(User%20Interface)/button/
pub fn setup_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: Color::WHITE,
        ..default()
    };
    let button_bundle_style = ButtonBundle {
        style: button_style.clone(),
        background_color: Color::NAVY.into(),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                // Menu container.
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // TODO: Use scaled asteroid sprite as bg?

                    // Title
                    parent.spawn(
                        TextBundle::from_section(
                            "Asteroids",
                            TextStyle {
                                font_size: 80.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    // Menu buttons.
                    parent
                        .spawn((button_bundle_style.clone(), MenuButtonAction::Play))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Play", button_text_style.clone()));
                        });
                    parent
                        .spawn((button_bundle_style.clone(), MenuButtonAction::Settings))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((button_bundle_style.clone(), MenuButtonAction::Quit))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                        });
                });
        });
}

pub fn update_menu_game_state(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => {
                    game_state.set(AppState::InGame);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => {
                    todo!()
                }
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
