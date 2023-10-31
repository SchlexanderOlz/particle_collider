use bevy::prelude::*;

mod physics;
mod simulation;
use simulation::ParticleCollider;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ParticleCollider))
        .run();
}
