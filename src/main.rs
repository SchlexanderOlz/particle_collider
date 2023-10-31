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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {}

#[derive(Resource)]
struct GameTimer(Timer);

fn tick_game(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<GameTimer>,
    mut query: Query<&mut Particle, With<SpriteBundle>>,
) {
    if timer.0.tick(time.delta()).just_finished() {}
}

fn init_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut Particle>,
) {
    for particle in &query {
        let pos = particle.pos();
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.10, 0.0, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
                ..default()
            },
            particle,
        ));
    }
}
