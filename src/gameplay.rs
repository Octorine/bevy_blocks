use bevy::{
    app::AppExit,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
const TIME_STEP: f32 = 1.0 / 60.0;
pub struct Paddle {
    speed: f32,
}

pub struct Ball {
    velocity: Vec3,
    size: Vec2,
}

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

pub fn setup_ball_and_paddle(commands: &mut Commands, atlas: Handle<TextureAtlas>) {
    // paddle
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: atlas.clone(),

            transform: Transform::from_xyz(0.0, (-SCREEN_HEIGHT + 40.) / 2., 0.0),
            sprite: TextureAtlasSprite {
                index: 1 as u32,
                ..Default::default()
            },
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
                index: 0 as u32,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ball {
            velocity: 400.0 * Vec3::new(0.5, 0.5, 0.0).normalize(),
            size: Vec2::new(30.0, 30.0),
        });
}

pub fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((paddle, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let horizontal_limit = (SCREEN_WIDTH - 162.) / 2.;
        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += direction * paddle.speed * TIME_STEP;
        // bound the paddle within the walls
        translation.x = translation.x.min(horizontal_limit).max(-horizontal_limit);
    }
}

pub fn ball_movement_system(mut ball_query: Query<(&Ball, &mut Transform)>) {
    if let Ok((ball, mut transform)) = ball_query.single_mut() {
        transform.translation += ball.velocity * TIME_STEP;
    }
}
pub fn ball_boundary_system(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut exit: EventWriter<AppExit>,
) {
    let horizontal = SCREEN_WIDTH / 2. - 30.0;
    let vertical = (SCREEN_HEIGHT - 30.) / 2.;
    if let Ok((mut ball, mut transform)) = ball_query.single_mut() {
        if transform.translation.x < -horizontal || transform.translation.x > horizontal {
            transform.translation.x = transform.translation.x.min(horizontal).max(-horizontal);
            ball.velocity.x *= -1.0;
        }
        if transform.translation.y > vertical {
            transform.translation.y = vertical;
            ball.velocity.y *= -1.0;
        }
        if transform.translation.y < -vertical {
            exit.send(AppExit);
        }
    }
}
pub fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &Transform)>,
    collider_query: Query<(Entity, &Collider, &Transform)>,
) {
    if let Ok((mut ball, ball_transform)) = ball_query.single_mut() {
        let ball_size = ball.size;
        let velocity = &mut ball.velocity;

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
}
