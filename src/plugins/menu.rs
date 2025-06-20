use crate::systems::menu::{cleanup_main_menu, handle_play_button, handle_quit_button, handle_settings_button, setup_main_menu, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu)
            .add_systems(
                Update,
                handle_play_button,
            )
            .add_systems(
                Update,
                handle_settings_button,
            )
            .add_systems(
                Update,
                handle_quit_button,
            )
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}
