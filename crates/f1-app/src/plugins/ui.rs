//! UI plugin for controls and information display.

use bevy::prelude::*;

use crate::resources::{AppState, AvailableSessions, ComparisonDrivers, LoadSession, SelectedTurn, TurnData};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TurnData>()
            .init_resource::<SelectedTurn>()
            .init_resource::<ComparisonDrivers>()
            .add_systems(OnEnter(AppState::SessionSelect), setup_session_select_ui)
            .add_systems(Update, handle_session_select.run_if(in_state(AppState::SessionSelect)))
            .add_systems(OnExit(AppState::SessionSelect), cleanup_session_select_ui);
    }
}

#[derive(Component)]
struct SessionSelectRoot;

#[derive(Component)]
struct SessionButton(std::path::PathBuf);

fn setup_session_select_ui(mut commands: Commands, available_sessions: Res<AvailableSessions>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            SessionSelectRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Select Session"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            for session in &available_sessions.sessions {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(50.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        SessionButton(session.path.clone()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(session.name.clone()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

fn handle_session_select(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &SessionButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut load_session_writer: EventWriter<LoadSession>,
) {
    for (interaction, mut color, session_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                load_session_writer.send(LoadSession(session_button.0.clone()));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

fn cleanup_session_select_ui(mut commands: Commands, query: Query<Entity, With<SessionSelectRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
