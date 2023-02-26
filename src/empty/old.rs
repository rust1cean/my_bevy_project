use std::time::Duration;

use bevy::prelude::*;

fn main() {
    App::new()
        // .add_plugin(Init)
        .insert_resource(GreetTimer(Timer::new(
            Duration::from_secs(2),
            TimerMode::Repeating,
        )))
        .add_startup_system(add_people)
        .add_system(greet_people)
        .add_plugins(DefaultPlugins)
        .run();
}

// pub struct Init;

// impl Plugin for Init {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.05)))
//             .add_plugins(DefaultPlugins.set(WindowPlugin {
//                 window: WindowDescriptor {
//                     title: "Snake".to_string(),
//                     width: 500.,
//                     height: 500.,
//                     ..default()
//                 },
//                 ..default()
//             }))
//             .add_startup_system(setup);
//     }
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name(String::from("John"))));
    commands.spawn((Person, Name(String::from("Jade"))));
    commands.spawn((Person, Name(String::from("Alex"))));

    println!("Hm");
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}

// fn draw_grid(mut commands: Commands) {
//     // Rectangle
//     commands.spawn(SpriteBundle {
//         sprite: Sprite {
//             // color: Color::rgb(0.25, 0.25, 0.25),
//             custom_size: Some(Vec2::new(50.0, 50.0)),
//             ..default()
//         },
//         ..default()
//     });
// }
