use crate::physics::Collider;
use bevy::prelude::*;

#[derive(Component)]
pub struct Brick;

#[derive(Bundle)]
pub struct BrickBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
    brick: Brick,
}

impl BrickBundle {
    pub fn new(translation: Vec3, image: Handle<Image>) -> BrickBundle {
        BrickBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation,
                    scale: Vec2::new(50., 20.).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                texture: image,
                ..default()
            },
            collider: Collider,
            brick: Brick,
        }
    }
}
