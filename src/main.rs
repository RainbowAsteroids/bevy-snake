mod snake;
mod types;
mod consts;
mod apple;
mod ui;

use rand::seq::SliceRandom;
use bevy::{prelude::*, window::WindowResolution, app::AppExit};

use types::*;
use snake::*;
use consts::*;
use apple::*;
use ui::*;

fn tilemap_to_global(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        (x as f32 * CELL_SIZE) - (WIDTH / 2.) + (CELL_SIZE / 2.),
        (-y as f32 * CELL_SIZE) + (HEIGHT / 2.) - (CELL_SIZE / 2.),
        0.
    )
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(0., 0., 0.1)))
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Window {
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    resizable: false,
                    title: "Snake game".to_owned(),
                    ..Default::default()
                }.into(),
                ..Default::default()
            }
        ))
        .add_state::<AppState>()
        .add_startup_system(spawn_camera)
        .init_resource::<ScoreManager>()

        .add_plugin(SnakePlugin)
        .add_plugin(ApplePlugin)
        .add_plugin(UiPlugin)

        .add_system(game_cleanup.in_schedule(OnExit(AppState::LoseScreen)))
        .add_system(game_cleanup.in_schedule(OnExit(AppState::WinScreen)))
        .add_system(game_cleanup.in_schedule(OnExit(AppState::StartScreen)))

        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn game_cleanup(
    mut commands: Commands,
    heads: Query<Entity, With<Head>>,
    apples: Query<Entity, With<Apple>>,
    tails: Query<Entity, With<Tail>>
) {
    for entity in heads.iter().chain(apples.iter()).chain(tails.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}
