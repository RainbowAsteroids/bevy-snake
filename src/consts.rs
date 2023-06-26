use bevy::prelude::*;

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;

pub const CELL_SIZE: f32 = 40.;
pub const CELL_MARGIN: f32 = 10.;

pub const BOARD_WIDTH: f32 = WIDTH / CELL_SIZE;
pub const BOARD_HEIGHT: f32 = HEIGHT / CELL_SIZE;

pub const SECONDS_BETWEEN_MOVES: f32 = 1. / 10.;

pub const HEAD_COLOR: Color = Color::WHITE;
pub const TAIL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const DEAD_SNAKE_COLOR: Color = Color::rgb(0.3, 0.05, 0.05);

pub const APPLE_COLOR: Color = Color::RED;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const UI_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const BUTTON_HOVER_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
