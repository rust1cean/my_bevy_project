use crate::cfg::{
    BORDER_BOT, BORDER_LEFT, BORDER_RIGHT, BORDER_TOP, CELL_STEP, PLAYER_COLOR, PLAYER_POSITION,
    PLAYER_SIZE, TAIL_SIZE,
};
use crate::food::Food;
use crate::game::{Cell, SpawnEvent};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct GrowthEvent;
pub struct GameOverEvent;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Head;

#[derive(Component, Debug)]
pub struct Tail;

#[derive(Component)]
pub struct Segment;

#[derive(Component, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

impl Direction {
    pub fn invert(&self) -> Self {
        match self {
            &Self::Left => Self::Right,
            &Self::Up => Self::Down,
            &Self::Right => Self::Left,
            &Self::Down => Self::Up,
            &Self::None => Self::None,
        }
    }
}

#[derive(Resource)]
pub struct TimePerStep(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimePerStep(Timer::from_seconds(0.1, TimerMode::Repeating)));
        app.add_startup_system(Self::spawn);
        app.add_system(Self::moving);
        app.add_system(Self::move_controls);
        app.add_system(Self::collision);
        app.add_system(Self::did_player_eat);
        app.add_system(Self::snake_growth);

        // Events
        app.add_event::<GrowthEvent>();
        app.add_event::<GameOverEvent>();
    }
}

impl PlayerPlugin {
    pub fn spawn(
        mut cmd: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut onspawn_writer: EventWriter<SpawnEvent>,
    ) {
        let translation: Vec3 = Vec3::new(PLAYER_POSITION.0, PLAYER_POSITION.1, 1.);
        let scale: Vec3 = Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 1.);
        let player: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation,
                scale,
                ..default()
            },
            material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
            ..default()
        };

        // Spawn snake
        cmd.spawn(player)
            .insert(Cell)
            .insert(Player)
            .insert(Direction::None);

        onspawn_writer.send(SpawnEvent);
    }

    pub fn moving(
        time: Res<Time>,
        mut timer: ResMut<TimePerStep>,
        mut player_query: Query<(&mut Transform, &Direction), With<Player>>,
        mut tail_query: Query<&mut Transform, (With<Tail>, Without<Player>)>,
    ) {
        if let Ok((mut player_transform, direction)) = player_query.get_single_mut() {
            if timer.0.tick(time.delta()).just_finished() {
                // Move tail
                let mut last_position = player_transform.translation.clone();
                tail_query.iter_mut().for_each(|mut segment_transform| {
                    let tmp = segment_transform.translation;
                    segment_transform.translation = last_position;
                    last_position = tmp;
                });

                // Add a horizontal step if this is it
                if let Some(step) = next_horizontal_step(direction) {
                    player_transform.translation.x += step;
                }
                // Add a vertical step if this is it
                if let Some(step) = next_vertical_step(direction) {
                    player_transform.translation.y += step;
                }
            }
        }
    }

    pub fn move_controls(
        key_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Direction, With<Player>>,
    ) {
        if let Ok(mut dir) = query.get_single_mut() {
            let new_dir: Direction;

            if key_input.pressed(KeyCode::Left) || key_input.pressed(KeyCode::A) {
                new_dir = Direction::Left;
            } else if key_input.pressed(KeyCode::Up) || key_input.pressed(KeyCode::W) {
                new_dir = Direction::Up;
            } else if key_input.pressed(KeyCode::Right) || key_input.pressed(KeyCode::D) {
                new_dir = Direction::Right;
            } else if key_input.pressed(KeyCode::Down) || key_input.pressed(KeyCode::S) {
                new_dir = Direction::Down;
            } else {
                new_dir = Direction::None;
            }

            if new_dir != dir.invert() && new_dir != Direction::None {
                *dir = new_dir;
            }
        }
    }

    fn collision(
        mut cmd: Commands,
        mut game_over_writer: EventWriter<GameOverEvent>,
        player: Query<(Entity, &Transform), With<Player>>,
        tail: Query<&Transform, (With<Tail>, Without<Player>)>,
    ) {
        let ate_his_tail =
            |player: &Transform, tail: &Query<&Transform, (With<Tail>, Without<Player>)>| match tail
                .iter()
                .find(|transform| transform.translation == player.translation)
            {
                Some(_) => true,
                None => false,
            };

        if let Ok((player, tf)) = player.get_single() {
            let Transform { translation, .. } = &tf;
            let mut player_collided: bool = false;

            if went_over_borders(&translation) || ate_his_tail(&tf, &tail) {
                player_collided = true;
            }

            if player_collided {
                cmd.entity(player).despawn();
                game_over_writer.send(GameOverEvent);
            }
        }
    }

    pub fn did_player_eat(
        mut cmd: Commands,
        mut growth_writer: EventWriter<GrowthEvent>,
        player_q: Query<&Transform, With<Player>>,
        food_q: Query<(Entity, &Transform), With<Food>>,
    ) {
        if let Ok(player_tf) = player_q.get_single() {
            food_q.iter().for_each(|(food, food_tf)| {
                let player_t = player_tf.translation;
                let food_t = food_tf.translation;

                if is_collision(&player_t, &food_t) {
                    cmd.entity(food).despawn();
                    growth_writer.send(GrowthEvent);
                }
            });
        }
    }

    pub fn snake_growth(
        mut growth_reader: EventReader<GrowthEvent>,
        mut query: Query<(&Transform, &Direction), With<Player>>,
        cmd: Commands,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let player_ate: bool = growth_reader.iter().next().is_some();
        if player_ate {
            if let Ok((transform, direction)) = query.get_single_mut() {
                let inverted = direction.invert();

                let horizontal_step = match next_horizontal_step(&inverted) {
                    Some(step) => step,
                    None => 0.,
                };

                let vertical_step = match next_vertical_step(&inverted) {
                    Some(step) => step,
                    None => 0.,
                };

                let translation: Vec3 = Vec3::new(
                    transform.translation.x + horizontal_step,
                    transform.translation.y + vertical_step,
                    1.,
                );

                create_tail_segment(translation, cmd, meshes, materials);
            }
        }
    }
}

pub fn next_horizontal_step(direction: &Direction) -> Option<f32> {
    match direction {
        Direction::Left => Some(-CELL_STEP),
        Direction::Right => Some(CELL_STEP),
        _ => None,
    }
}

pub fn next_vertical_step(direction: &Direction) -> Option<f32> {
    match direction {
        Direction::Up => Some(CELL_STEP),
        Direction::Down => Some(-CELL_STEP),
        _ => None,
    }
}

pub fn create_tail_segment(
    translation: Vec3,
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let scale: Vec3 = Vec3::new(TAIL_SIZE, TAIL_SIZE, 1.);
    let segment: MaterialMesh2dBundle<ColorMaterial> = MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        material: materials.add(ColorMaterial::from(PLAYER_COLOR)),
        ..default()
    };

    cmd.spawn(segment).insert(Cell).insert(Tail);
}

pub fn went_over_borders(translation: &Vec3) -> bool {
    translation.x > BORDER_RIGHT
        || translation.x < BORDER_LEFT
        || translation.y > BORDER_TOP
        || translation.y < BORDER_BOT
}

pub fn is_collision(first: &Vec3, second: &Vec3) -> bool {
    let fx = first.x.round();
    let fy = first.y.round();
    ((fx - 1.)..=(fx + 1.)).contains(&second.x) && ((fy - 1.)..=(fy + 1.)).contains(&second.y)
}
