use bevy::prelude::*;
use food::FoodPlugin;
use game::GamePlugin;
use general::GeneralPlugin;
use player::PlayerPlugin;

mod cfg;
mod food;
mod game;
mod general;
mod player;

fn main() {
    App::new()
        .add_plugin(GeneralPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(GamePlugin)
        .run();
}
