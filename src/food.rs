use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{
    cfg::{
        CELL_SIZE, FOOD_COLOR, HALF_CELL_SIZE, HALF_WINDOW_HEIGHT, HALF_WINDOW_WIDTH, MAX_FOOD,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
    game::{Cell, SpawnEvent},
    player::{is_collision, Player},
};

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::spawn);
    }
}

impl FoodPlugin {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut onspawn_writer: EventWriter<SpawnEvent>,
        food_query: Query<&Food>,
        cell_query: Query<&Transform, With<Cell>>,
    ) {
        if food_query.iter().len() < MAX_FOOD {
            let food_translation = get_random_position();

            // Check if cell and food have the same coordinates
            for Transform { translation, .. } in cell_query.iter() {
                if is_collision(translation, &food_translation) {
                    return;
                }
            }

            let scale = Vec3::new(CELL_SIZE, CELL_SIZE, 1.);
            let food = MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                transform: Transform {
                    translation: food_translation,
                    scale,
                    ..default()
                },
                material: materials.add(ColorMaterial::from(FOOD_COLOR)),
                ..default()
            };

            cmd.spawn(food).insert(Cell).insert(Food);
            onspawn_writer.send(SpawnEvent);
        }
    }
}

pub fn get_random_position() -> Vec3 {
    let mut rng = rand::thread_rng();

    let max_horizontal_count: i32 = HALF_WINDOW_WIDTH as i32 / CELL_SIZE as i32;
    let max_vertical_count: i32 = HALF_WINDOW_HEIGHT as i32 / CELL_SIZE as i32;

    let horizontal_count: i32 = rng.gen_range(-max_horizontal_count..max_horizontal_count);
    let vertical_count: i32 = rng.gen_range(-max_vertical_count..max_vertical_count);

    let x = (CELL_SIZE) * horizontal_count as f32 + HALF_CELL_SIZE;
    let y = (CELL_SIZE) * vertical_count as f32 + HALF_CELL_SIZE;

    // let max_horizontal_count: i32 = WINDOW_WIDTH as i32 / CELL_SIZE as i32;
    // let max_vertical_count: i32 = WINDOW_HEIGHT as i32 / CELL_SIZE as i32;

    // let horizontal_count: i32 = rng.gen_range(0..max_horizontal_count);
    // let vertical_count: i32 = rng.gen_range(0..max_vertical_count);

    // let x = (CELL_SIZE) * horizontal_count as f32;
    // let y = (CELL_SIZE) * vertical_count as f32;

    Vec3::new(x, y, 1.)
}

#[derive(Component)]
pub struct Food;
