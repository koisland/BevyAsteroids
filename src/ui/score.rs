use bevy::prelude::*;

use crate::player::Player;

// Move to src/ui?
#[derive(Component)]
pub struct ScoreText;

pub fn add_score_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                color: Color::GOLD,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            justify_self: JustifySelf::Center,
            ..default()
        }),
        ScoreText,
    ));
}

pub fn update_score_text(
    player_query: Query<&mut Player>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    let mut score_text = text_query.single_mut();
    score_text.sections[1].value = format!("{:.2}", player.score)
}
