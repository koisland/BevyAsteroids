use bevy::prelude::*;

#[derive(Component)]
pub struct OnPauseScreen;

pub fn setup_pause_message(mut commands: Commands) {
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
            OnPauseScreen,
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
                            "- Paused -",
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
                });
        });
}
