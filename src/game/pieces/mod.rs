use bevy::app::Plugin;

use crate::game::pieces::{enemies::EnemiesPlugin, items::ItemsPlugin, player::PlayerPlugin};

pub mod enemies;
pub mod items;
pub mod player;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerPlugin, ItemsPlugin, EnemiesPlugin));
    }
}
