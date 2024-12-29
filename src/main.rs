use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowResolution};

const BOMBER_MAN_PLAYER_SPRITE: &str = "BomberManSprite_0.png";
const PLAYER_SIZE: (f32, f32) = (16.0, 16.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bomberman Game(Extreme Mode)".to_string(), // Set the window title
                resolution: WindowResolution::new(1280.0, 720.0),  // Set the resolution
                mode: WindowMode::Windowed,                        // Windowed mode
                decorations: true,                                 // Enable window decorations
                transparent: false,                                // Disable transparency
                ..default()                                        // Use default values for the rest
            }),
            ..default()
        }))
        // .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Add an example player sprite (for context)
    commands.spawn(SpriteBundle {
        texture: asset_server.load(BOMBER_MAN_PLAYER_SPRITE),
        transform: Transform {
            scale: Vec3::new(2.0, 2.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
