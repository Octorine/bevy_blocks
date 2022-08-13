use crate::state::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
struct PauseUI;

pub fn enter_system_set() -> SystemSet {
    SystemSet::on_enter(crate::state::GameState::GameOver).with_system(setup_game_over)
}

pub fn update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::GameOver).with_system(menu_update)
}

pub fn exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::GameOver).with_system(teardown)
}
pub fn setup_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_style = TextStyle {
        font: asset_server.load("font/FiraSans-Light.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.71, 0.8, 0.4),
    };
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceAround,
                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
                alpha: 1.0,
            }),
            ..Default::default()
        })
        .with_children(|grandparent| {
            grandparent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle::default())
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Back",
                                    text_style.clone(),
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                style: Style {
                                    flex_grow: 0.0,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Congratulations!\nYou won the game.".to_string(),
                            text_style.clone(),
                            TextAlignment {
                                vertical: VerticalAlign::Top,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn menu_update(
    mut commands: Commands,
    mut state: ResMut<State<crate::state::GameState>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                state
                    .set(GameState::MainMenu)
                    .expect("Failed to return to main menu");
            }

            _ => (),
        }
    }
}

fn teardown(mut commands: Commands, menu_nodes: Query<(Entity, &Node)>) {
    for (e, _n) in menu_nodes.iter() {
        commands.entity(e).despawn();
    }
}
