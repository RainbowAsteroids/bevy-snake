use bevy::prelude::*;

use crate::*;

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<AppleEaten>()
        .add_system(spawn_first_apple.in_schedule(OnEnter(AppState::Playing)))
        .add_system(apple_collision.in_set(OnUpdate(AppState::Playing)))
        .add_system(handle_apple_eaten.in_set(OnUpdate(AppState::Playing)));
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
    mut app_state: ResMut<NextState<AppState>>,
    mut score: ResMut<ScoreManager>
) {
    if !apple_events.is_empty() {
        score.score += 1;

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
