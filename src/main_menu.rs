use bevy::prelude::*;
use bevy::app::AppExit;
use crate::state::GameState;

#[derive(Component)]
struct PauseUI;

pub fn enter_system_set() -> SystemSet {
    SystemSet::on_enter(crate::state::GameState::MainMenu).with_system(setup_main_menu)
}

pub fn update_system_set() -> SystemSet {
    SystemSet::on_update(GameState::MainMenu).with_system(menu_update)
}

pub fn exit_system_set() -> SystemSet {
    SystemSet::on_exit(GameState::MainMenu).with_system(teardown)
}
pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                    "Quit",
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

                    parent
                        .spawn_bundle(ButtonBundle::default())
                        .with_children(|btn| {
                            btn.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Start",
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
                });
        });
}

fn menu_update(
    mut commands: Commands,
    mut state: ResMut<State<crate::state::GameState>>,
    mut interaction_query: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if text.sections[0].value == "Quit" {
                    exit.send(AppExit);
                }
                else if text.sections[0].value == "Start" {
                        state.set(GameState::Level).expect("Failed to start level");
                        commands.insert_resource(crate::gameplay::Score::new());
                }
                else { () }},

            _ => (),
        }
    }
}

fn teardown(mut commands: Commands, menu_nodes: Query<(Entity, &Node)>) {
    for (e, _n) in menu_nodes.iter() {
        commands.entity(e).despawn();
    }
}
