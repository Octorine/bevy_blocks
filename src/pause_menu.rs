use bevy::{
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};

use crate::state::GameState;

#[derive(Component)]
struct PauseUI;

pub fn enter_system_set() -> SystemSet {
    SystemSet::on_enter(crate::state::GameState::PauseMenu).with_system(setup_pause_ui)
}

pub fn update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::PauseMenu).with_system(get_keyboard_input)
}

pub fn exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::PauseMenu).with_system(teardown)
}
pub fn setup_pause_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let text_style = TextStyle {
        font: asset_server.load("font/FiraSans-Light.ttf"),
        font_size: 60.0,
        color: Color::rgb(0.71, 0.8, 0.4),
    };
    commands
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                "Press Any Key to Continue".to_string(),
                text_style.clone(),
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            style: Style {
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                flex_grow: 1.0,
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PauseUI);
}

fn get_keyboard_input(
    mut keyboard_input: EventReader<KeyboardInput>,
    mut state: ResMut<State<GameState>>,
) {
    if keyboard_input
        .iter()
        .any(|evt| evt.state == ElementState::Pressed)
    {
        state.pop();
    }
}
fn teardown(mut commands: Commands, pause_text: Query<(Entity, &PauseUI)>) {
    if let Ok((pause_entity, _)) = pause_text.get_single() {
        commands.entity(pause_entity).despawn();
    }
}
