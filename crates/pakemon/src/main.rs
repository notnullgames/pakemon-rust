use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Pakémon".into(),
            width: 320.0,
            height: 240.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_scene_system.system())
        .run();
}

fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = asset_server.load("intro.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    asset_server.watch_for_changes().unwrap();
}
