pub mod board;
pub mod interactions;
pub mod pieces;

use bevy::prelude::*;

use crate::game::{board::MapPlugin, interactions::InteractionsPlugin, pieces::PiecesPlugin};

#[derive(States, PartialEq, Eq, Default, Hash, Debug, Clone)]
pub enum GameState {
    #[default]
    EvadingEnemies,
    SelectingUpgrade,
    BossBattle,
}

pub struct GamePlayPlugin;

impl Plugin for GamePlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((MapPlugin, PiecesPlugin, InteractionsPlugin));
    }
}
