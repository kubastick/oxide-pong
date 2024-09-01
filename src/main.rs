use bevy::prelude::*;


fn main() {
    let mut bevy_app = App::new();

    bevy_app.add_plugins(DefaultPlugins);
    bevy_app.add_systems(Startup, setup);

    bevy_app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    // Hello world
    commands.spawn((
        OxidePongUI,
        TextBundle::from_sections([
            TextSection::new(
                "Hello world",
                TextStyle {
                    font_size: 20.0,
                    color: Color::srgb_u8(255,255,255),
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
}

#[derive(Component)]
struct OxidePongUI;