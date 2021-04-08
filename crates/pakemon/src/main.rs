use bevy::{prelude::*};
use bevy_retro::*;

// Create a stage label that will be used for our game logic stage
#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pakémon".into(),
            width: 320.0,
            height: 240.0,
            ..Default::default()
        })
        .add_plugins(RetroPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _scene_graph: ResMut<SceneGraph>,
) {
    // Load our sprites
    let title_image = asset_server.load("title.png");

    // Spawn the camera
    commands.spawn().insert_bundle(CameraBundle {
        camera: Camera {
            // Set our camera to have a fixed height and an auto-resized width
            size: CameraSize::FixedHeight(240),
            background_color: Color::new(0.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn the title-logo
    commands.spawn().insert_bundle(SpriteBundle {
        image: title_image,
        position: Position::new(0, 0, -1),
        sprite: Sprite {
            ..Default::default()
        },
        ..Default::default()
    });
}

