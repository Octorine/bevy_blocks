use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn enter_system_set() -> SystemSet {
    SystemSet::on_enter(crate::state::GameState::Level).with_system(setup_level)
}

pub fn update_system_set() -> SystemSet {
    SystemSet::on_update(crate::state::GameState::Level)
        .with_system(ball_movement_system)
        .with_system(paddle_movement_system)
        .with_system(ball_collision_system)
        .with_system(ball_boundary_system)
        .with_system(initial_pause_check)
}
pub fn exit_system_set() -> SystemSet {
    SystemSet::on_exit(crate::state::GameState::Level).with_system(teardown_system)
}
pub fn paused_update_system_set() -> SystemSet {
    SystemSet::on_inactive_update(crate::state::GameState::Level)
}
pub fn initial_pause_check(
    mut state: ResMut<State<crate::state::GameState>>,
    mut is_new: ResMut<BrandNewLevel>,
) {
    if is_new.0 {
        state
            .push(crate::state::GameState::PauseMenu)
            .expect("Failed to enter pause menu");
        is_new.0 = false;
    }
}
pub struct BrandNewLevel(bool);

fn teardown_system(
    mut commands: Commands,
    ball_query: Query<(&Ball, Entity)>,
    collider_query: Query<(&Collider, Entity)>,
    ui_query: Query<(&Node, Entity)>,
) {
    for (_, be) in ball_query.iter() {
        commands.entity(be).despawn();
    }
    for (_, ce) in collider_query.iter() {
        commands.entity(ce).despawn();
    }
    for (_, uie) in ui_query.iter() {
        commands.entity(uie).despawn();
    }
}

fn setup_level(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut state: ResMut<State<crate::state::GameState>>,
    atlases: ResMut<Assets<TextureAtlas>>,
    levels: Res<Vec<crate::level::Level>>,
    mut score: ResMut<Score>,
) {
    let atlas = crate::sprite_sheet::build_sprite_sheet(&mut asset_server, atlases);
    if score.current_level >= levels.len() {
        state.set(crate::state::GameState::GameOver);
    } else {
        let new_level = &levels[score.current_level];
        crate::level::add_bricks(&mut commands, &mut score, new_level, atlas.clone());
        setup_ball_and_paddle(&mut commands, atlas);
        setup_level_ui(&mut commands, asset_server, &*score);
        commands.insert_resource(BrandNewLevel(true));
    }
}
#[derive(Component)]
pub struct Paddle {
    speed: f32,
}

#[derive(Component)]
pub struct Ball {
    velocity: Vec3,
    size: Vec2,
}

#[derive(Component)]
pub enum Collider {
    Scorable { size: Vec2 },
    Paddle { size: Vec2 },
}
impl Collider {
    pub fn get_size(&self) -> Vec2 {
        match &self {
            Collider::Scorable { size: s } => *s,
            Collider::Paddle { size: s } => *s,
        }
    }
}

#[derive(Component)]
pub struct Score {
    pub current_level: usize,
    pub bricks_left: usize,
    pub points: i32,
    pub lives: i32,
}
impl Score {
    pub fn new() -> Score {
        Score {
            points: 0,
            bricks_left: 0,
            lives: 3,
            current_level: 0,
        }
    }
}
#[derive(Component)]
pub struct PointsText;

#[derive(Component)]
pub struct LivesText;

pub fn setup_level_ui(commands: &mut Commands, asset_server: Res<AssetServer>, score: &Score) {
    let text_style = TextStyle {
        font: asset_server.load("font/FiraSans-Light.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.71, 0.8, 0.4),
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::SpaceBetween,
                size: Size {
                    height: Val::Px(40.0),
                    width: Val::Percent(100.0),
                },

                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 1.,
                green: 1.,
                blue: 1.,
                alpha: 0.0,
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("Score: {}", score.points),
                        text_style.clone(),
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Left,
                        },
                    ),
                    style: Style {
                        flex_grow: 0.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PointsText);

            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        format!("Lives: {}", score.lives),
                        text_style,
                        TextAlignment {
                            vertical: VerticalAlign::Top,
                            horizontal: HorizontalAlign::Right,
                        },
                    ),
                    style: Style {
                        flex_grow: 0.0,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(LivesText);
        });
}

pub fn setup_ball_and_paddle(commands: &mut Commands, atlas: Handle<TextureAtlas>) {
    // paddle
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 1 as usize,
                ..Default::default()
            },

            texture_atlas: atlas.clone(),
            transform: Transform::from_xyz(0.0, (-SCREEN_HEIGHT + 40.) / 2., 0.0),
            ..Default::default()
        })
        .insert(Paddle { speed: 500.0 })
        .insert(Collider::Paddle {
            size: Vec2::new(162.0, 30.0),
        });
    // ball
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(0.0, -250.0, 1.0),
            sprite: TextureAtlasSprite {
                index: 0 as usize,
                ..Default::default()
            },
            texture_atlas: atlas.clone(),
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, 0.5, 0.0).normalize(),
            size: Vec2::new(30.0, 30.0),
        });
}

pub fn paddle_movement_system(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut state: ResMut<State<crate::state::GameState>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    let (paddle, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::P) {
        state
            .push(crate::state::GameState::PauseMenu)
            .expect("Failed to open Pause Menu");
    }

    let horizontal_limit = (SCREEN_WIDTH - 162.) / 2.;
    let translation = &mut transform.translation;
    // move the paddle horizontally
    translation.x += direction * paddle.speed * time.delta_seconds();
    // bound the paddle within the walls
    translation.x = translation.x.min(horizontal_limit).max(-horizontal_limit);
}

pub fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    let (ball, mut transform) = ball_query.single_mut();
    transform.translation += ball.velocity * time.delta_seconds();
}
pub fn ball_boundary_system(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut state: ResMut<State<crate::state::GameState>>,
    mut lives_txt_query: Query<(&mut Text, &LivesText)>,
    mut score: ResMut<Score>,
) {
    let horizontal = SCREEN_WIDTH / 2. - 15.0;
    let vertical = (SCREEN_HEIGHT - 30.) / 2.;
    let (mut ball, mut transform) = ball_query.single_mut();
    if transform.translation.x < -horizontal || transform.translation.x > horizontal {
        transform.translation.x = transform.translation.x.min(horizontal).max(-horizontal);
        ball.velocity.x *= -1.0;
    }
    if transform.translation.y > vertical {
        transform.translation.y = vertical;
        ball.velocity.y *= -1.0;
    }
    if transform.translation.y < -vertical {
        score.lives -= 1;
        if score.lives <= 0 {
            state
                .set(crate::state::GameState::MainMenu)
                .expect("Failed to open main menu");
        } else {
            let (mut lives_text, _) = lives_txt_query.get_single_mut().unwrap();
            lives_text.sections[0].value = format!("Lives: {}", &score.lives);
            ball.velocity = 400.0 * Vec3::new(0.5, 0.5, 0.0).normalize();

            *transform = Transform::from_xyz(0.0, -250.0, 1.0);
            state
                .push(crate::state::GameState::PauseMenu)
                .expect("Failed to open pause menu");
        }
    }
}
pub fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    mut state: ResMut<State<crate::state::GameState>>,
    mut score: ResMut<Score>,
    mut points_txt_query: Query<(&mut Text, &PointsText)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    let (mut ball, ball_transform) = ball_query.single_mut();
    let ball_size = ball.size;
    let velocity = &mut ball.velocity;
    let (mut points_text, _) = points_txt_query.get_single_mut().unwrap();

    // check collision with walls
    for (collider_entity, collider, transform) in collider_query.iter() {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            collider.get_size(),
        );
        if let Some(collision) = collision {
            // scorable colliders should be despawned and increment the scoreboard on collision
            if let Collider::Scorable { size: _ } = *collider {
                commands.entity(collider_entity).despawn();
                score.points += 1;
                score.bricks_left = score.bricks_left.saturating_sub(1);
                if score.bricks_left == 0 {
                    score.current_level += 1;
                    state.restart();
                }
                points_text.as_mut().sections[0].value = format!("Score: {}", score.points);
            }

            // reflect the ball when it collides
            let mut reflect_x = false;
            let mut reflect_y = false;

            // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = velocity.x > 0.0,
                Collision::Right => reflect_x = velocity.x < 0.0,
                Collision::Top => reflect_y = velocity.y < 0.0,
                Collision::Bottom => reflect_y = velocity.y > 0.0,
                Collision::Inside => {
                    reflect_y = true;
                    reflect_x = true
                }
            }

            // reflect velocity on the x-axis if we hit something on the x-axis
            if reflect_x {
                velocity.x = -velocity.x;
            }

            // reflect velocity on the y-axis if we hit something on the y-axis
            if reflect_y {
                velocity.y = -velocity.y;
            }

            // break if this collide is on a solid, otherwise continue check whether a solid is
            // also in collision
        }
    }
}
