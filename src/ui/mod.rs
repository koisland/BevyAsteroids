use bevy::prelude::*;

use crate::core::AppState;

use self::{
    menu::{setup_menu, update_menu_game_state, MenuState, OnMainMenuScreen},
    pause::{setup_pause_message, OnPauseScreen},
    score::{add_score_ui, update_score_text, ScoreText},
};

pub mod menu;
pub mod pause;
pub mod score;

pub struct UIPlugin;

// https://bevyengine.org/examples/Games/game-menu/
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(
                OnTransition {
                    from: AppState::Menu,
                    to: AppState::InGame,
                },
                add_score_ui,
            )
            .add_systems(
                FixedUpdate,
                update_score_text.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                FixedUpdate,
                (update_menu_game_state).run_if(in_state(AppState::Menu)),
            )
            // Setup and delete pause message.
            .add_systems(OnEnter(AppState::Paused), setup_pause_message)
            .add_systems(OnExit(AppState::Paused), despawn_screen::<OnPauseScreen>)
            // Delete menu nodes on exiting menu.
            .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMainMenuScreen>)
            // Delete score text on exiting game.
            .add_systems(
                OnTransition {
                    from: AppState::InGame,
                    to: AppState::Menu,
                },
                despawn_screen::<ScoreText>,
            );
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
