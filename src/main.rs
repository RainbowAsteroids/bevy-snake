use rand::seq::SliceRandom;
use bevy::{prelude::*, window::WindowResolution, app::AppExit};

const WIDTH: f32 = 800.;
const HEIGHT: f32 = 600.;

const CELL_SIZE: f32 = 40.;
const CELL_MARGIN: f32 = 10.;

const BOARD_WIDTH: f32 = WIDTH / CELL_SIZE;
const BOARD_HEIGHT: f32 = HEIGHT / CELL_SIZE;

const SECONDS_BETWEEN_MOVES: f32 = 1. / 12.;

const HEAD_COLOR: Color = Color::WHITE;
const TAIL_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const DEAD_SNAKE_COLOR: Color = Color::rgb(0.3, 0.05, 0.05);

const APPLE_COLOR: Color = Color::RED;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const UI_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const BUTTON_HOVER_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

#[derive(Component)]
struct Head {
    body: Vec<Entity>
}

#[derive(Component)]
struct Velocity { 
    vector: Vec3
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
struct Menu;
#[derive(Component)]
struct PlayButton;
#[derive(Component)]
struct QuitButton;

pub struct AppleEaten;
pub struct SpawnTail(Entity);


#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    StartScreen,
    LoseScreen,
    WinScreen,
    Playing,
}

impl AppState {
    fn title(&self) -> &str {
        match self {
            AppState::StartScreen => "Snake",
            AppState::LoseScreen => "Game Over",
            AppState::WinScreen => "You Win",
            AppState::Playing => panic!("AppState::Playing.title() is undefined")
        }
    }

    fn play_button_title(&self) -> &str {
        if let AppState::Playing = self {
            panic!("AppState::Playing.play_button_title() is undefined");
        } else if let AppState::StartScreen = self {
            "Play"
        } else {
            "Play again"
        }
    }
}

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

        .add_startup_system(spawn_camera)

        .add_state::<AppState>()

        .add_event::<SpawnTail>()
        .add_system(spawn_snake.in_schedule(OnEnter(AppState::Playing)))
        .add_system(move_snake.in_set(OnUpdate(AppState::Playing)))
        .add_system(snake_input.before(move_snake).in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_spawn_tail.in_set(OnUpdate(AppState::Playing)))
        .add_system(snake_collision_check.in_set(OnUpdate(AppState::Playing)))
        .add_system(red_snake.in_schedule(OnEnter(AppState::LoseScreen)))
        
        .add_event::<AppleEaten>()
        .add_system(spawn_first_apple.in_schedule(OnEnter(AppState::Playing)))
        .add_system(apple_collision.in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_apple_eaten.in_set(OnUpdate(AppState::Playing)))

        .add_system(spawn_screen.in_schedule(OnEnter(AppState::LoseScreen)))
        .add_system(spawn_screen.in_schedule(OnEnter(AppState::WinScreen)))
        .add_system(spawn_screen.in_schedule(OnEnter(AppState::StartScreen)))

        .add_system(button_hover.in_set(OnUpdate(AppState::LoseScreen)))
        .add_system(button_hover.in_set(OnUpdate(AppState::WinScreen)))
        .add_system(button_hover.in_set(OnUpdate(AppState::StartScreen)))

        .add_system(button_click.in_set(OnUpdate(AppState::LoseScreen)))
        .add_system(button_click.in_set(OnUpdate(AppState::WinScreen)))
        .add_system(button_click.in_set(OnUpdate(AppState::StartScreen)))

        .add_system(despawn_screen.in_schedule(OnExit(AppState::LoseScreen)))
        .add_system(despawn_screen.in_schedule(OnExit(AppState::WinScreen)))
        .add_system(despawn_screen.in_schedule(OnExit(AppState::StartScreen)))

        .add_system(game_cleanup.in_schedule(OnExit(AppState::LoseScreen)))
        .add_system(game_cleanup.in_schedule(OnExit(AppState::WinScreen)))
        .add_system(game_cleanup.in_schedule(OnExit(AppState::StartScreen)))

        .run();
}

fn make_button(
    commands: &mut ChildBuilder,
    text: &str,
    asset_server: &Res<AssetServer>,
    component: impl Bundle,
) {
    commands.spawn(ButtonBundle {
        style: Style { 
            min_size: Size::new(Val::Px(0.), Val::Px(40.)),
            padding: UiRect::new(Val::Px(5.), Val::Px(5.), Val::Px(5.), Val::Px(5.)),
            margin: UiRect::new(Val::Px(0.), Val::Px(5.), Val::Px(0.), Val::Px(5.)),
            ..default()
        },
        background_color: BUTTON_COLOR.into(),
        ..default()
    }).with_children(|commands| {
        commands.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 30.,
                color: TEXT_COLOR.into(),
                font: asset_server.load("source-code-pro.ttf")
            }
        ));
    }).insert(component);
}

fn spawn_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    app_state: Res<State<AppState>>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }).insert(Menu)
    .with_children(|commands| {
        commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(40.), Val::Percent(25.)),
                padding: UiRect::new(Val::Percent(5.), Val::Percent(5.), Val::Percent(5.), Val::Percent(5.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                //min_size: Size::new(Val::Percent(0.), Val::Percent(0.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: UI_COLOR.into(),
            ..default()
        }).with_children(|commands| {
            commands.spawn(TextBundle::from_section(
                app_state.0.title(),
                TextStyle {
                    font_size: 50.,
                    color: TEXT_COLOR.into(),
                    font: asset_server.load("source-code-pro.ttf")
                }
            ).with_style(Style {
                margin: UiRect::new(Val::Px(0.), Val::Px(10.), Val::Px(0.), Val::Px(10.)),
                ..default()
            }));
            make_button(commands, app_state.0.play_button_title(), &asset_server, PlayButton);
            make_button(commands, "Quit", &asset_server, QuitButton);
        });
    });
}

fn despawn_screen(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    commands.entity(menu.single()).despawn_recursive();
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

fn button_hover(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked | Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_COLOR.into()
        }
    }
}

fn button_click(
    play_button_interaction: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    quit_button_interaction: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app_exit: EventWriter<AppExit>
) {
    if let Ok(Interaction::Clicked) = play_button_interaction.get_single() {
        app_state.set(AppState::Playing);
    } else if let Ok(Interaction::Clicked) = quit_button_interaction.get_single() {
        app_exit.send(AppExit);
    }
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
        transform: Transform::from_translation(tilemap_to_global(5, 5)),
        ..default()
    })
    .insert(Head { body: Vec::new() })
    .insert(Velocity { vector: Vec3::new(CELL_SIZE, 0., 0.) })
    .insert(MoveTimer { timer: Timer::from_seconds(SECONDS_BETWEEN_MOVES, TimerMode::Repeating) });
}


fn move_snake(
    time: Res<Time>,
    mut heads: Query<(&Head, &Velocity, &mut MoveTimer, &mut Transform)>, 
    mut tail_transform_query: Query<&mut Transform, (With<Tail>, Without<Head>)>
) {
    for (head, velocity, mut move_timer, mut head_transform) in &mut heads {
        move_timer.timer.tick(time.delta());

        if move_timer.timer.finished() {
            // form an iterator that both yields the element and the element preceeding it
            let iter = head.body.iter().cloned().rev();
            let next = iter.clone().skip(1).map(|e| Some(e)).chain(std::iter::once(None));

            for (entity, next_entity) in iter.zip(next) {
                let translation = next_entity.map(|e| tail_transform_query.get(e).unwrap()).unwrap_or(&head_transform).translation;
                let mut tail_transform = tail_transform_query.get_mut(entity).unwrap();

                if tail_transform.translation != translation {
                    tail_transform.translation = translation;
                }
            }

            head_transform.translation += velocity.vector;
        }
    }
}

fn handle_spawn_tail(
    mut commands: Commands, 
    mut heads: Query<(&mut Head, &Transform)>,
    tail_transforms: Query<&Transform, With<Tail>>,
    mut tail_event: EventReader<SpawnTail>
) {
    for event in tail_event.iter() {
        let (mut head, head_transform) = heads.get_mut(event.0).unwrap();

        let new_transform = head.body.last()
            .map(|e| tail_transforms.get(e.clone()).unwrap())
            .unwrap_or(head_transform);

        let entity = commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: TAIL_COLOR, 
                custom_size: Vec2::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN).into(),
                ..default()
            },
            transform: *new_transform,
            ..default()
        })
        .insert(Tail)
        .id();

        head.body.push(entity);
    }
}

fn snake_input(
    mut heads: Query<(&mut Velocity, &Head, &Transform)>,
    cells: Query<&Transform, With<Tail>>,
    keys: Res<Input<KeyCode>>
) {
    let new_velocity = {
        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            Vec3::new(-CELL_SIZE, 0., 0.).into()
        } else if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            Vec3::new(CELL_SIZE, 0., 0.).into()
        } else if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            Vec3::new(0., CELL_SIZE, 0.).into()
        } else if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            Vec3::new(0., -CELL_SIZE, 0.).into()
        } else { None }
    };

    if let Some(v) = new_velocity {
        for (mut velocity, head, transform) in &mut heads {
            let prev_transform = head.body.first().map(|e| cells.get(e.clone()).unwrap());
            if prev_transform.map_or(true, |t| t.translation - v != transform.translation) {
                velocity.vector = v;
            }
        }
    }
}

fn snake_collision_check(
    heads: Query<(&Head, &Transform)>,
    tails: Query<&Transform, With<Tail>>,
    mut app_state: ResMut<NextState<AppState>>
) {
    for (head, head_transform) in &heads {
        let position = head_transform.translation;

        if position.x < -WIDTH / 2.
            || position.x > WIDTH / 2.
            || position.y < -HEIGHT / 2.
            || position.y > HEIGHT / 2.
        {
            app_state.set(AppState::LoseScreen);
        }

        if head.body.len() >= 3 {
            for tail_position in head.body.iter().cloned().map(|e| tails.get(e).unwrap().translation) {
                if tail_position == position {
                    app_state.set(AppState::LoseScreen);
                }
            }
        }
    }
}


fn spawn_apple(commands: &mut Commands, position: Vec3) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: APPLE_COLOR,
            custom_size: Vec2::new(CELL_SIZE - CELL_MARGIN, CELL_SIZE - CELL_MARGIN).into(),
            ..default()
        },
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Apple);
}

fn spawn_first_apple(mut commands: Commands) {
    spawn_apple(&mut commands, tilemap_to_global(10, 3))
}

fn apple_collision(
    mut commands: Commands,
    apples: Query<(Entity, &Transform), With<Apple>>,
    heads: Query<(Entity, &Transform), With<Head>>,
    mut apple_events: EventWriter<AppleEaten>,
    mut tail_events: EventWriter<SpawnTail>
) {
    for (apple_entity, apple_transform) in &apples {
        for (head_entity, head_transform) in &heads {
            if head_transform.translation == apple_transform.translation {
                commands.entity(apple_entity).despawn_recursive();
                apple_events.send(AppleEaten);
                tail_events.send(SpawnTail(head_entity));
            }
        }
    }
}

fn handle_apple_eaten(
    mut commands: Commands,
    transforms: Query<&Transform, Or<(With<Apple>, With<Tail>, With<Head>)>>,
    mut apple_events: EventReader<AppleEaten>,
    mut app_state: ResMut<NextState<AppState>>
) {
    if !apple_events.is_empty() {
        let positions = (0..BOARD_WIDTH as i32)
            .map(|x| (0..BOARD_HEIGHT as i32).map(move |y| tilemap_to_global(x, y)))
            .flatten()
            .filter(|v| !transforms.iter().any(|t| t.translation == *v))
            .collect::<Vec<Vec3>>();

        let mut rng = rand::thread_rng();

        for _ in apple_events.iter() {
            if let Some(apple_position) = positions.choose(&mut rng) {
                spawn_apple(&mut commands, *apple_position);
            } else {
                // We ran out of spots to spawn an apple
                app_state.set(AppState::WinScreen);
            }
        }
    }
}

fn red_snake(mut sprites: Query<&mut Sprite, Or<(With<Head>, With<Tail>)>>) {
    for mut sprite in &mut sprites {
        sprite.color = DEAD_SNAKE_COLOR
    }
}
