use bevy::prelude::*;

use crate::cfg::{CELL_SIZE, CELL_STEP, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH};

pub struct Map;

impl Plugin for Map {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, Self::position_translation);
    }
}

impl Map {
    fn position_translation(mut query: Query<&mut Transform, With<Cell>>) {
        query.iter_mut().for_each(|mut transform| {
            let scale = transform.scale.clone();
            transform.translation.x -= HALF_WINDOW_WIDTH - (scale.x / 2.);
            transform.translation.y -= HALF_WINDOW_HEIGHT - (scale.y / 2.);
        });
    }
}

#[derive(Component)]
pub(crate) struct Cell;
