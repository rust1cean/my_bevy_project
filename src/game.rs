use crate::{
    cfg::{HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH},
    player::GameOverEvent,
};
use bevy::prelude::*;

pub struct SpawnEvent;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::is_game_over);
        // app.add_system(Self::position_translation);

        app.add_event::<SpawnEvent>();
    }
}

impl GamePlugin {
    pub fn is_game_over(mut game_over_reader: EventReader<GameOverEvent>) {
        if game_over_reader.iter().next().is_some() {
            println!("Game over");
        }
    }
}

impl GamePlugin {
    pub fn position_translation(
        mut query: Query<&mut Transform, With<Cell>>,
        mut spawn_reader: EventReader<SpawnEvent>,
    ) {
        if spawn_reader.iter().next().is_some() {
            query.iter_mut().for_each(|mut transform| {
                let scale = transform.scale.clone();
                transform.translation.x -= HALF_WINDOW_WIDTH - (scale.x / 2.);
                transform.translation.y -= HALF_WINDOW_HEIGHT - (scale.y / 2.);
            });
        }
    }
}

#[derive(Component)]
pub struct Cell;
