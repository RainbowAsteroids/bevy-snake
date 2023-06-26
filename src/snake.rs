use bevy::prelude::*;

use crate::*;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<SpawnTail>()
        .add_system(spawn_snake.in_schedule(OnEnter(AppState::Playing)))
        .add_system(move_snake.in_set(OnUpdate(AppState::Playing)))
        .add_system(snake_input.before(move_snake).in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_spawn_tail.in_set(OnUpdate(AppState::Playing)))
        .add_system(snake_collision_check.in_set(OnUpdate(AppState::Playing)))
        .add_system(red_snake.in_schedule(OnEnter(AppState::LoseScreen)));
    }
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

fn red_snake(mut sprites: Query<&mut Sprite, Or<(With<Head>, With<Tail>)>>) {
    for mut sprite in &mut sprites {
        sprite.color = DEAD_SNAKE_COLOR
    }
}
