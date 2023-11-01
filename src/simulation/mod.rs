use crate::physics::{Interact, Move, Point, Vector2D};
use bevy::prelude::*;

use self::particle::Particle;

pub mod particle;

pub struct ParticleCollider;

impl Plugin for ParticleCollider {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForceTimer(Timer::from_seconds(0.001, TimerMode::Repeating)))
            .insert_resource(TickSpeed(0.1))
            .add_systems(Startup, spawn_particles)
            .add_systems(Update, move_particles);
    }
}

fn spawn_particles(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    const PARTICLE_SIZE: f32 = 10.0;
    for _ in 0..5 {
        let point = Point {
            x: rand::random::<u32>() as f32 % 500.0,
            y: 0.0,
        };
        let force = Vector2D::from_parts((-20 + rand::random::<i64>() % 40) as f64, 0.0);
        commands.spawn((
            Particle::new(point, force, 10.0, PARTICLE_SIZE),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.10, 0.0, 0.75),
                    custom_size: Some(Vec2::new(PARTICLE_SIZE, PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(point.x, point.y, 0.0)),
                ..default()
            },
        ));
    }
}

fn move_particles(
    time: Res<Time>,
    mut timer: ResMut<ForceTimer>,
    tick_speed: Res<TickSpeed>,
    mut query: Query<(&mut Particle, &mut Transform)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut particles: Vec<(Mut<'_, Particle>, Mut<'_, Transform>)> =
            query.iter_mut().collect();

        for i in 0..particles.len() {
            for y in 0..particles.len() {
                if i == y {
                    continue;
                }

                if particles[i]
                    .0
                    .has_collision(particles[y].0.as_ref())
                    .is_some()
                {
                    println!("\n\nCollision happened!\n\n");
                    let force = particles[i].0.get_collision_force(particles[y].0.as_ref());
                    println!("Force {}", particles[i].0.get_force().get_x());
                    particles[i].0.collide(Vector2D::from_parts(200.0, 0.0));
                }
            }
        }

        for (mut particle, mut transform) in particles {
            let pos = particle.pos();
            if pos.x < -500.0 || pos.y < -500.0 || pos.x > 500.0 || pos.y > 500.0 {
                let force = particle.get_force();
                particle.collide(Vector2D::from_parts(
                    -force.get_x() * 4.0,
                    -force.get_y() * 4.0,
                ));
            }
            particle.mov(tick_speed.0);
            let pos = particle.pos();

            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

#[derive(Resource)]
struct TickSpeed(f64);

#[derive(Resource)]
struct ForceTimer(Timer);
