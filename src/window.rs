use crate::cfg::{CLEAR_COLOR, TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub struct Window;

impl Plugin for Window {
    fn build(&self, app: &mut App) {
        self.clear_bg(app);
        self.create_window(app);

        app.add_startup_system(Self::camera_setup);
    }
}

impl Window {
    pub fn clear_bg(&self, app: &mut App) {
        app.insert_resource(ClearColor(CLEAR_COLOR));
    }

    pub fn create_window(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: TITLE.to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }));
    }

    pub fn camera_setup(mut cmd: Commands) {
        cmd.spawn(Camera2dBundle::default());
    }
}
