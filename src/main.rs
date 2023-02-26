use bevy::prelude::*;
use map::Map;
use player::Player;
use window::Window;

mod cfg;
mod food;
mod map;
mod player;
mod window;

fn main() {
    App::new()
        .add_plugin(Window)
        .add_plugin(Player)
        .add_plugin(Map)
        // .add_plugin(Food)
        .run();
}
