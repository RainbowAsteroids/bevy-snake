use bevy::prelude::*;

use crate::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
   fn build(&self, app: &mut App) {
       app
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
       .add_system(despawn_screen.in_schedule(OnExit(AppState::StartScreen)));
   } 
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
    mut score: ResMut<ScoreManager>
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
                size: Size::new(Val::Percent(45.), Val::Percent(40.)),
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

            if let AppState::LoseScreen = app_state.0 {
                commands.spawn(TextBundle::from_section(
                    format!("Score: {}", score.score),
                    TextStyle {
                        font_size: 25.,
                        color: TEXT_COLOR.into(),
                        font: asset_server.load("source-code-pro.ttf")
                    }).with_style(Style {
                        margin: UiRect::new(Val::Px(0.), Val::Px(10.), Val::Px(0.), Val::Px(10.)),
                        ..default()
                    })
                );

                commands.spawn(TextBundle::from_section(
                    format!("High score: {}", score.high_score),
                    TextStyle {
                        font_size: 25.,
                        color: TEXT_COLOR.into(),
                        font: asset_server.load("source-code-pro.ttf")
                    }).with_style(Style {
                        margin: UiRect::new(Val::Px(0.), Val::Px(10.), Val::Px(0.), Val::Px(10.)),
                        ..default()
                    })
                );

            }

            make_button(commands, app_state.0.play_button_title(), &asset_server, PlayButton);
            make_button(commands, "Quit", &asset_server, QuitButton);
        });
    });
    score.sync();
}

fn despawn_screen(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    commands.entity(menu.single()).despawn_recursive();
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
