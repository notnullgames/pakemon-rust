use bevy::prelude::*;

/// This example illustrates loading and saving scenes from files
fn main() {
    App::build()
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_scene_system.system())
        .add_system(print_system.system())
        .run();
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Reflect, Default)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
}

fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    let scene_handle: Handle<DynamicScene> = asset_server.load("intro.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    asset_server.watch_for_changes().unwrap();
}

fn print_system(query: Query<(Entity, &ComponentA), Changed<ComponentA>>) {
    for (entity, component_a) in query.iter() {
        println!("  Entity({})", entity.id());
        println!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
}
