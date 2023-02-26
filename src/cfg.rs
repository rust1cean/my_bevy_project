use bevy::prelude::Color;

// Window
pub(crate) const TITLE: &str = "Snake!";
pub(crate) const WINDOW_WIDTH: f32 = 500.;
pub(crate) const WINDOW_HEIGHT: f32 = WINDOW_WIDTH;
pub(crate) const HALF_WINDOW_WIDTH: f32 = WINDOW_WIDTH / 2.;
pub(crate) const HALF_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT / 2.;
pub(crate) const CLEAR_COLOR: Color = Color::rgb(0., 0., 0.);

// Map
pub(crate) const CELL_SIZE: f32 = 15.;
pub(crate) const CELL_STEP: f32 = CELL_SIZE;
pub(crate) const BORDER_LEFT: f32 = -HALF_WINDOW_WIDTH;
pub(crate) const BORDER_RIGHT: f32 = HALF_WINDOW_WIDTH;
pub(crate) const BORDER_TOP: f32 = HALF_WINDOW_HEIGHT;
pub(crate) const BORDER_BOT: f32 = -HALF_WINDOW_HEIGHT;

// Player
pub(crate) const PLAYER_COLOR: Color = Color::rgb(0.2, 0.7, 0.);
pub(crate) const PLAYER_SIZE: f32 = CELL_SIZE;

// Food
pub(crate) const FOOD_COLOR: Color = Color::rgb(0.7, 0., 0.2);
