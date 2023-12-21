use bevy::prelude::*;

use self::score::{add_score_ui, update_score_text};

pub mod score;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_score_ui)
            .add_systems(FixedUpdate, update_score_text);
    }
}
