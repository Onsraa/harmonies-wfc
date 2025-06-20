use bevy::prelude::*;
use bevy::ui::*;

#[derive(Component)]
pub struct MainMenuUI;
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct SettingsButton;
#[derive(Component)]
pub struct QuitButton;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default] MainMenu,
    InGame,
    Paused,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Quit,
}


pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg_handle = asset_server.load("images/bg.png");
    let font_handle = asset_server.load("fonts/CherryBombOne-Regular.ttf");

    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        align_self: AlignSelf::FlexStart,
        margin: UiRect::new(Val::Px(50.0), Val::Px(20.0), Val::Px(20.0), Val::Px(20.0)),
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ImageNode::new(bg_handle).with_mode(NodeImageMode::Stretch),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Harmonies"),
                TextFont {
                    font_size: 60.0,
                    font: font_handle.clone(),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Play Button
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    PlayButton,
                    MainMenuUI,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Play"),
                        TextFont {
                            font_size: 24.0,
                            font: font_handle.clone(),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Settings Button
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    SettingsButton,
                    MainMenuUI
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Settings"),
                        TextFont {
                            font_size: 24.0,
                            font: font_handle.clone(),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Quit Button
            parent
                .spawn((
                    Button,
                    button_node.clone(),
                    QuitButton,
                    MainMenuUI
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quit"),
                        TextFont {
                            font_size: 24.0,
                            font: font_handle.clone(),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}


pub fn handle_play_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::InGame);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 0.7));
            }
        }
    }
}

pub fn handle_settings_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Settings);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 0.7));
            }
        }
    }
}

pub fn handle_quit_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                next_state.set(GameState::Quit);
                exit.write(AppExit::Success);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 1.0));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgba(0.188, 0.345, 0.451, 0.7));
            }
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}
