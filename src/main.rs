use bevy::prelude::*;

mod physics;
mod simulation;
use physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugin))
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

}
