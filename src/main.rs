mod wall;

use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::prelude::*;
use iyes_perf_ui::entries::{PerfUiBundle, PerfUiFixedTimeEntries, PerfUiSystemEntries};
use iyes_perf_ui::PerfUiPlugin;
use std::f64::consts::PI;
use crate::wall::{WallBundle, WallLocation};

const PADDLE_SPEED: f32 = 500.0;
const BALL_BASE_SPEED: f32 = 400.0;

fn main() {
    let mut bevy_app = App::new();

    bevy_app.add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Oxide Pong".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }));

    #[cfg(debug_assertions)]
    {
        bevy_app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        bevy_app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        bevy_app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);

        bevy_app.add_plugins(PerfUiPlugin);
    }

    bevy_app.insert_resource(Time::<Fixed>::from_hz(500.0));

    bevy_app.add_systems(Startup, setup);
    bevy_app.add_systems(FixedUpdate, (
        move_paddle,
        apply_velocity,
        check_and_handle_collisions,
    ));

    bevy_app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    #[cfg(debug_assertions)]
    {
        commands.spawn((
            PerfUiBundle::default(),
            PerfUiSystemEntries::default(),
            PerfUiFixedTimeEntries::default(),
        ));
    }

    let paddle_handle: Handle<Image> = asset_server.load("paddle/paddle.png");
    let ball_handle: Handle<Image> = asset_server.load("ball/ball.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Right));

    // Hello world
    commands.spawn((
        OxidePongUI,
        TextBundle::from_sections([
            TextSection::new(
                "Hello world",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb_u8(255, 255, 255),
                    ..default()
                },
            ),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(16.0),
                left: Val::Px(16.0),
                ..default()
            }),
    ));

    commands.spawn((
        Paddle,
        Collider,
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -300.0, 0.0),
                scale: (Vec2::new(64., 16.) * 2.0).extend(1.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            texture: paddle_handle,
            ..default()
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 100.0, 0.0),
                scale: Vec2::new(1.0, 1.0).extend(1.0),
                ..default()
            },
            texture: ball_handle,
            ..default()
        },
        Velocity::new(Vec2::new(0.0, BALL_BASE_SPEED * -1.)),
        Ball,
    ));
}

fn move_paddle(
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

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn check_and_handle_collisions(
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(Entity, &Transform, Option<&Paddle>, &Collider), With<Collider>>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_bounding_circle = BoundingCircle::new(ball_transform.translation.truncate(), 1.);

    for (_, collider_transform, paddle, _) in &collider_query {
        let collision = ball_collision(
            ball_bounding_circle,
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.,
            ),
        );

        if let Some(collision) = collision {
            println!("Is collision with paddle: {:?}", paddle.is_some());
            println!("Collision detected with {:?}!", collision);

            if paddle.is_some() {
                // Detected collision with paddle, calculate output angle and apply it to velocity
                let paddle_bounding_box = Aabb2d::new(
                    collider_transform.translation.truncate(),
                    collider_transform.scale.truncate() / 2.,
                );
                dbg!(paddle_bounding_box);
                dbg!(ball_transform.translation);

                let closest_bounding_box_point = paddle_bounding_box.closest_point(ball_transform.translation.truncate());
                dbg!(closest_bounding_box_point);


                let relative_closest_bounding_box_point = Vec2 {
                    x: closest_bounding_box_point.x - paddle_bounding_box.min.x,
                    y: closest_bounding_box_point.y - paddle_bounding_box.min.y,
                };
                dbg!(relative_closest_bounding_box_point);

                let x_axis_center_offset_percentage = relative_closest_bounding_box_point.x / collider_transform.scale.x;
                dbg!(x_axis_center_offset_percentage);

                let normalized_angle_percentage = (x_axis_center_offset_percentage - 0.5) * 2.;
                dbg!(normalized_angle_percentage);
                let relative_output_angle = 60. * normalized_angle_percentage;
                dbg!(relative_output_angle);

                let absolute_output_angle = 90. + (relative_output_angle * -1.);

                println!("Output angle degrees: ${absolute_output_angle}");

                let radian_output_angle = (absolute_output_angle * PI as f32) / 180.;
                dbg!(radian_output_angle);

                let output_velocity = Vec2::from_angle(radian_output_angle) * Vec2::splat(BALL_BASE_SPEED);

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
                    println!("Bouncing x");
                    ball_velocity.x = -ball_velocity.x;
                }

                if bounce_y {
                    println!("Bouncing y");
                    ball_velocity.y = -ball_velocity.y;
                }
            }

            println!("Current velocity: X: {} Y: {}", ball_velocity.x, ball_velocity.y);
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


#[derive(Component)]
struct OxidePongUI;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

impl Velocity {
    fn new(velocity: Vec2) -> Velocity {
        Velocity(velocity)
    }
}