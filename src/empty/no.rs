use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new().add_plugin(Window).add_plugin(Player).run();
}

pub struct Window;

impl Plugin for Window {
    fn build(&self, app: &mut App) {
        self.clear_color(app);
        self.create_window(app);

        app.add_startup_system(Self::camera_setup);
    }
}

impl Window {
    const CLEAR_COLOR: Color = Color::rgb(0.03, 0.03, 0.03);
    const TITLE: String = "snake".to_string();
    const WIDTH: f32 = 500.;
    const HEIGHT: f32 = 500.;

    fn clear_color(&self, app: &mut App) {
        app.insert_resource(ClearColor(Self::CLEAR_COLOR));
    }

    fn create_window(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: Self::TITLE,
                width: Self::WIDTH,
                height: Self::HEIGHT,
                ..default()
            },
            ..default()
        }));
    }

    fn camera_setup(mut cmd: Commands) {
        cmd.spawn(Camera2dBundle::default());
    }
}

#[derive(Component)]
pub struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
pub struct SnakeTail(Vec<Entity>);

#[derive(Component)]
pub struct Segment;

#[derive(Component)]
pub struct Position(f32, f32);

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Player;

impl Plugin for Player {
    fn build(&self, app: &mut App) {}
}

impl Player {
    // const POSITION
}

pub struct Grid;

impl Plugin for Grid {
    fn build(&self, app: &mut App) {}
}

impl Grid {
    const SIZE: f32 = 16.;
    const STEP: Position = Position(Self::SIZE / 2.0, Self::SIZE / 2.0);
}

// #[derive(Component)]
// pub struct Cell;

// pub struct Grid;

// impl Plugin for Grid {
//     fn build(&self, app: &mut App) {
//         app.add_startup_system(Self::init_grid);
//         app.add_startup_system_to_stage(StartupStage::PostStartup, Self::test);
//     }
// }

// impl Grid {
//     const CELL_SIZE: f32 = 100.;
//     const CELL_COLOR: Color = Color::rgba(0.2, 0.2, 0.2, 1.);

//     pub fn init_grid(
//         mut cmd: Commands,
//         mut meshes: ResMut<Assets<Mesh>>,
//         mut materials: ResMut<Assets<ColorMaterial>>,
//     ) {
//         let count_horizontal: f32 = Self::count_horizontal();
//         let count_vertical: f32 = Self::count_vertical();
//         let amount: f32 = (count_horizontal * count_vertical).ceil();

//         let mut x: f32 = 0.;
//         let mut y: f32 = 0.;

//         (0..amount as usize).into_iter().for_each(|i| {
//             x = (i % count_horizontal as usize) as f32 * Self::CELL_SIZE;
//             y += if i > 0 && x == 0. {
//                 Self::CELL_SIZE
//             } else {
//                 0.
//             };

//             cmd.spawn((
//                 MaterialMesh2dBundle {
//                     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
//                     transform: Transform {
//                         scale: Vec3::new(Self::CELL_SIZE, Self::CELL_SIZE, 1.),
//                         translation: Vec3::from_array(Self::position_translation(x, y)),
//                         ..default()
//                     },
//                     material: materials.add(ColorMaterial::from(Self::CELL_COLOR)),
//                     ..default()
//                 },
//                 Cell,
//             ));
//         });
//     }

//     fn count_horizontal() -> f32 {
//         (Window::WIDTH / Self::CELL_SIZE).ceil()
//     }

//     fn count_vertical() -> f32 {
//         (Window::HEIGHT / Self::CELL_SIZE).ceil()
//     }

//     fn position_translation(x: f32, y: f32) -> [f32; 3] {
//         let x: f32 = x - (Window::WIDTH / 2.) + (Self::CELL_SIZE / 2.);
//         let y: f32 = (Window::HEIGHT / 2.) - y - (Self::CELL_SIZE / 2.);
//         let z: f32 = 1.;
//         [x, y, z]
//     }
// }
