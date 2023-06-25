use rand::{*, seq::SliceRandom};

use bevy::{prelude::*, window::WindowResolution};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;

const CELL_SIZE: f32 = 40.;
const CELL_MARGIN: f32 = 10.;

const BOARD_WIDTH: f32 = WIDTH / CELL_SIZE;
const BOARD_HEIGHT: f32 = HEIGHT / CELL_SIZE;

const SECONDS_BETWEEN_MOVES: f32 = 1. / 12.;

const HEAD_COLOR: Color = Color::WHITE;
const TAIL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const APPLE_COLOR: Color = Color::RED;

#[derive(Component)]
struct Head {
    body: Vec<Entity>
}

#[derive(Component)]
struct Velocity { 
    vector: Vec2 
}

#[derive(Component)]
struct MoveTimer {
    timer: Timer
}

#[derive(Component)]
struct Tail;

#[derive(Component)]
struct Apple;

#[derive(Component)]
struct Cell {
    position: Vec2,
}

pub struct AppleEaten;
pub struct SpawnTail(Entity);

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

        .add_startup_system(spawn_camera)

        .add_system(fix_position)

        .add_event::<SpawnTail>()
        .add_startup_system(spawn_snake)
        .add_system(move_snake)
        .add_system(snake_input.before(move_snake))
        .add_system(handle_spawn_tail)
        .add_system(snake_collision_check)
        
        .add_event::<AppleEaten>()
        .add_startup_system(spawn_first_apple)
        .add_system(apple_collision)
        .add_system(handle_apple_eaten)

        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: HEAD_COLOR,
            custom_size: Vec2::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN).into(),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    })
    .insert(Head { body: Vec::new() })
    .insert(Velocity { vector: Vec2::new(1., 0.) })
    .insert(MoveTimer { timer: Timer::from_seconds(SECONDS_BETWEEN_MOVES, TimerMode::Repeating) })
    .insert(Cell { position: Vec2::new(5., 5.) });
}

fn fix_position(mut cells: Query<(&Cell, &mut Transform)>) {
    for (cell, mut transform) in &mut cells {
        transform.translation.x = (cell.position.x * CELL_SIZE) - (WIDTH / 2.) + (CELL_SIZE / 2.);
        transform.translation.y = (-cell.position.y * CELL_SIZE) + (HEIGHT / 2.) - (CELL_SIZE / 2.);
    }
}

fn move_snake(
    time: Res<Time>,
    mut heads: Query<(&Head, &Velocity, &mut MoveTimer, &mut Cell)>, 
    mut tail_query: Query<&mut Cell, (With<Tail>, Without<Head>)>
) {
    for (head, velocity, mut move_timer, mut head_cell) in &mut heads {
        move_timer.timer.tick(time.delta());

        if move_timer.timer.finished() {
            // form an iterator that both yields the element and the element preceeding it
            let iter = head.body.iter().cloned().rev();
            let next = iter.clone().skip(1).map(|e| Some(e)).chain(std::iter::once(None));

            for (entity, next_entity) in iter.zip(next) {
                let position = next_entity.map(|e| tail_query.get(e).unwrap()).unwrap_or(&head_cell).position;
                let mut tail = tail_query.get_mut(entity).unwrap();

                if tail.position != position {
                    tail.position = position;
                }
            }

            head_cell.position += velocity.vector;
        }
    }
}

fn handle_spawn_tail(
    mut commands: Commands, 
    mut heads: Query<(&mut Head, &Cell)>,
    tails: Query<&Cell, With<Tail>>,
    mut tail_event: EventReader<SpawnTail>
) {
    for event in tail_event.iter() {
        let (mut head, head_cell) = heads.get_mut(event.0).unwrap();

        let position = head.body.last()
            .map(|e| tails.get(e.clone()).unwrap())
            .unwrap_or(head_cell).position;

        let entity = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: TAIL_COLOR, 
                custom_size: Vec2::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN).into(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(Tail)
        .insert(Cell { position })
        .id();

        head.body.push(entity);
    }
}

fn snake_input(
    mut heads: Query<(&mut Velocity, &Head, &Cell)>,
    cells: Query<&Cell, With<Tail>>,
    keys: Res<Input<KeyCode>>
) {
    let new_velocity = {
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            Vec2::new(-1., 0.).into()
        } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            Vec2::new(1., 0.).into()
        } else if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            Vec2::new(0., -1.).into()
        } else if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            Vec2::new(0., 1.).into()
        } else { None }
    };

    if let Some(v) = new_velocity {
        for (mut velocity, head, head_cell) in &mut heads {
            let prev_cell = head.body.first().map(|e| cells.get(e.clone()).unwrap());
            if prev_cell.map_or(true, |cell| cell.position - v != head_cell.position) {
                velocity.vector = v;
            }
        }
    }
}

fn snake_collision_check(
    heads: Query<(&Head, &Cell)>,
    tails: Query<&Cell, With<Tail>>,
) {
    fn die() { 
        println!("dead");
        loop { } 
    }

    for (head, head_cell) in &heads {
        let position = head_cell.position;

        if position.x < 0. || position.x >= BOARD_WIDTH || position.y < 0. || position.y >= BOARD_HEIGHT {
            die();
        }

        if head.body.len() >= 3 {
            for tail_cell in head.body.iter().cloned().map(|e| tails.get(e).unwrap()) {
                if tail_cell.position == position && head.body.len() != 2 {
                    die();
                }
            }
        }
    }
}


fn spawn_apple(commands: &mut Commands, position: Vec2) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: APPLE_COLOR,
            custom_size: Vec2::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN).into(),
            ..default()
        },
        ..default()
    })
    .insert(Apple)
    .insert(Cell { position });
}

fn spawn_first_apple(mut commands: Commands) {
    spawn_apple(&mut commands, Vec2::new(10., 3.))
}

fn apple_collision(
    mut commands: Commands,
    apples: Query<(Entity, &Cell), With<Apple>>,
    heads: Query<(Entity, &Cell), With<Head>>,
    mut apple_events: EventWriter<AppleEaten>,
    mut tail_events: EventWriter<SpawnTail>
) {
    for (apple_entity, apple_cell) in &apples {
        for (head_entity, head_cell) in &heads {
            if head_cell.position == apple_cell.position {
                commands.entity(apple_entity).despawn_recursive();
                apple_events.send(AppleEaten);
                tail_events.send(SpawnTail(head_entity));
            }
        }
    }
}

fn handle_apple_eaten(
    mut commands: Commands,
    cells: Query<&Cell>,
    mut apple_events: EventReader<AppleEaten>,
) {
    if !apple_events.is_empty() {
        println!("Apple eaten!");
    
        let positions = (0..BOARD_WIDTH as i32)
            .map(|x| (0..BOARD_HEIGHT as i32).map(move |y| Vec2::new(x as f32, y as f32)))
            .flatten()
            .filter(|v| !cells.iter().any(|cell| cell.position == *v))
            .collect::<Vec<Vec2>>();

        let mut rng = rand::thread_rng();

        for _ in apple_events.iter() {
            if let Some(apple_position) = positions.choose(&mut rng) {
                spawn_apple(&mut commands, *apple_position);
            } else {
                // TODO: win game
            }
        }
    }
}
