use crate::physics::Velocity;
use crate::BALL_BASE_SPEED;
use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    velocity: Velocity,
    sprite_bundle: SpriteBundle,
}

impl BallBundle {
    pub fn new(ball_image_handle: Handle<Image>) -> BallBundle {
        BallBundle {
            ball: Ball,
            velocity: Velocity::new(Vec2::new(0.0, BALL_BASE_SPEED * -1.)),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0., 100.0, 0.0),
                    scale: Vec2::new(1.0, 1.0).extend(1.0),
                    ..default()
                },
                texture: ball_image_handle,
                ..default()
            },
        }
    }
}
