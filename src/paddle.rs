use crate::physics::Collider;
use crate::PADDLE_SPEED;
use bevy::input::ButtonInput;
use bevy::prelude::*;

#[derive(Component)]
pub struct Paddle;
pub fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_paddle_position =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    paddle_transform.translation.x = new_paddle_position;
}

#[derive(bevy::prelude::Bundle)]
pub struct PaddleBundle {
    paddle: Paddle,
    collider: Collider,
    sprite_bundle: SpriteBundle,
}

impl PaddleBundle {
    pub fn new(paddle_image_handle: Handle<Image>) -> PaddleBundle {
        PaddleBundle {
            paddle: Paddle,
            collider: Collider,
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -300.0, 0.0),
                    scale: (Vec2::new(64., 16.) * 2.0).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                texture: paddle_image_handle,
                ..default()
            },
        }
    }
}
