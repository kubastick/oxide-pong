use crate::ball::Ball;
use crate::brick::Brick;
use crate::paddle::Paddle;
use crate::BALL_BASE_SPEED;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Component, Deref, DerefMut, Entity, Query, Res, Time, Transform, With,
};
use std::f64::consts::PI;

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(velocity: Vec2) -> Velocity {
        Velocity(velocity)
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn check_and_handle_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<
        (
            Entity,
            &Transform,
            Option<&Paddle>,
            Option<&Brick>,
            &Collider,
        ),
        With<Collider>,
    >,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_bounding_circle = BoundingCircle::new(ball_transform.translation.truncate(), 1.);

    for (collider_entity, collider_transform, paddle, brick, _) in &collider_query {
        let collision = ball_collision(
            ball_bounding_circle,
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            if paddle.is_some() {
                // Detected collision with paddle, calculate output angle and apply it to velocity
                let paddle_bounding_box = Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_transform.scale.truncate() / 2.,
                );
                let closest_bounding_box_point =
                    paddle_bounding_box.closest_point(ball_transform.translation.truncate());
                let relative_closest_bounding_box_point = Vec2 {
                    x: closest_bounding_box_point.x - paddle_bounding_box.min.x,
                    y: closest_bounding_box_point.y - paddle_bounding_box.min.y,
                };
                let x_axis_center_offset_percentage =
                    relative_closest_bounding_box_point.x / collider_transform.scale.x;
                let normalized_angle_percentage = (x_axis_center_offset_percentage - 0.5) * 2.;
                let relative_output_angle = 60. * normalized_angle_percentage;
                let absolute_output_angle = 90. + (relative_output_angle * -1.);
                let radian_output_angle = (absolute_output_angle * PI as f32) / 180.;
                let output_velocity =
                    Vec2::from_angle(radian_output_angle) * Vec2::splat(BALL_BASE_SPEED);

                ball_velocity.x = output_velocity.x;
                ball_velocity.y = output_velocity.y;
            } else {
                let mut bounce_x = false;
                let mut bounce_y = false;

                match collision {
                    Collision::Left => bounce_x = ball_velocity.x > 0.0,
                    Collision::Right => bounce_x = ball_velocity.x < 0.0,
                    Collision::Top => bounce_y = ball_velocity.y < 0.0,
                    Collision::Bottom => bounce_y = ball_velocity.y > 0.0,
                }

                if bounce_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                if bounce_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }

            if brick.is_some() {
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
