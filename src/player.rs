use crate::cfg::{
    BORDER_BOT, BORDER_LEFT, BORDER_RIGHT, BORDER_TOP, CELL_STEP, PLAYER_COLOR, PLAYER_SIZE,
};
use crate::map::Cell;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimePerStep(Timer::from_seconds(0.2, TimerMode::Repeating)));
        app.add_startup_system(Self::spawn);
        app.add_system(Self::moving);
        app.add_system(Self::move_controls);
        app.add_system(Self::borders);
    }
}

impl Player {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let scale: Vec3 = Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 1.);
        let player: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform { scale, ..default() },
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            ..default()
        };

        cmd.spawn(player)
            .insert(Cell)
            .insert(Head)
            .insert(Direction::None);
    }

    pub fn moving(
        time: Res<Time>,
        mut timer: ResMut<TimePerStep>,
        mut query: Query<(&mut Transform, &Direction), With<Head>>,
    ) {
        if let Ok((mut tf, dir)) = query.get_single_mut() {
            if timer.0.tick(time.delta()).just_finished() {
                match dir {
                    Direction::Left => tf.translation.x -= CELL_STEP,
                    Direction::Right => tf.translation.x += CELL_STEP,
                    Direction::Up => tf.translation.y += CELL_STEP,
                    Direction::Down => tf.translation.y -= CELL_STEP,
                    _ => (),
                }
            }
        }
    }

    pub fn move_controls(
        key_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Direction, With<Head>>,
    ) {
        if let Ok(mut dir) = query.get_single_mut() {
            if key_input.pressed(KeyCode::Left) {
                *dir = Direction::Left;
            } else if key_input.pressed(KeyCode::Up) {
                *dir = Direction::Up;
            } else if key_input.pressed(KeyCode::Right) {
                *dir = Direction::Right;
            } else if key_input.pressed(KeyCode::Down) {
                *dir = Direction::Down;
            }
        }
    }

    fn borders(mut cmd: Commands, query: Query<(Entity, &Transform), With<Head>>) {
        if let Ok((entity, tf)) = query.get_single() {
            let Transform { translation, .. } = &tf;

            if translation.x > BORDER_RIGHT
                || translation.x < BORDER_LEFT
                || translation.y > BORDER_TOP
                || translation.y < BORDER_BOT
            {
                cmd.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct Head;

#[derive(Component)]
pub struct Tail(Vec<Entity>);

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

#[derive(Resource)]
pub struct TimePerStep(Timer);
