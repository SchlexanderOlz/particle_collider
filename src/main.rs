use bevy::prelude::*;

mod physics;
mod simulation;
use physics::Interact;
use simulation::{Particle, ParticleCollider};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ParticleCollider))
        .run();
}
