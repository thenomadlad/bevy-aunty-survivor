pub mod game;
pub mod menu;

use bevy::prelude::*;

use crate::{game::GamePlayPlugin, menu::MenuPlugin};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum ApplicationState {
    #[default]
    MainMenu,
    GamePlaying,
    GamePaused,
    GameOver,
}

pub struct AuntySurvivorApplicationPlugin;

impl Plugin for AuntySurvivorApplicationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ApplicationState>()
            .add_plugins((MenuPlugin, GamePlayPlugin));
    }
}
