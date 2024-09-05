mod ball;
mod brick;
mod paddle;
mod physics;
mod wall;

use crate::ball::BallBundle;
use crate::brick::*;
use crate::paddle::*;
use crate::physics::*;
use crate::wall::*;
use bevy::prelude::*;
use iyes_perf_ui::entries::{PerfUiBundle, PerfUiFixedTimeEntries, PerfUiSystemEntries};
use iyes_perf_ui::PerfUiPlugin;

const PADDLE_SPEED: f32 = 500.0;
const BALL_BASE_SPEED: f32 = 400.0;

fn main() {
    let mut bevy_app = App::new();

    bevy_app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Oxide Pong".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
    );

    #[cfg(debug_assertions)]
    {
        bevy_app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        bevy_app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);
        bevy_app.add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin);

        bevy_app.add_plugins(PerfUiPlugin);
    }

    bevy_app.insert_resource(Time::<Fixed>::from_hz(500.0));

    bevy_app.add_systems(Startup, setup);
    bevy_app.add_systems(
        FixedUpdate,
        (move_paddle, apply_velocity, check_and_handle_collisions),
    );

    bevy_app.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    let level_bg_handle: Handle<Image> = asset_server.load("levels/bg1.png");
    let brick_handle: Handle<Image> = asset_server.load("bricks/crusher.png");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Top));
    commands.spawn(WallBundle::new(WallLocation::Right));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-500.0, 500., -1.0),
            scale: (Vec2::new(2000., 2000.) * 2.0).extend(1.0),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        texture: level_bg_handle,
        ..default()
    });

    commands.spawn(PaddleBundle::new(paddle_handle));
    commands.spawn(BallBundle::new(ball_handle));

    for x in 0..17 {
        for y in 0..10 {
            let x_position = -400. + x as f32 * 50.;
            let y_position = 280. - y as f32 * 20.;
            let vec3_position = Vec3::new(x_position, y_position, 0.);

            commands.spawn(BrickBundle::new(vec3_position, brick_handle.clone()));
        }
    }
}
