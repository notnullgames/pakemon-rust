use bevy::prelude::*;
use bevy::render::pass::ClearColor;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pakémon".into(),
            width: 320.0,
            height: 240.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("title.png").into()),
        ..Default::default()
    });
}
