use crate::physics::{Interact, Move, Point, Vector2D};
use bevy::prelude::*;

use self::particle::Particle;

pub mod particle;

pub struct ParticleCollider;

impl Plugin for ParticleCollider {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForceTimer(Timer::from_seconds(
            1.0 / 60.0,
            TimerMode::Repeating,
        )))
        .insert_resource(TickSpeed(0.1))
        .add_systems(Startup, spawn_particles)
        .add_systems(Update, move_particles);
    }
}

fn spawn_particles(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    const PARTICLE_SIZE: f32 = 20.0;
    for _ in 0..50 {
        let point = Point {
            x: -300.0 + rand::random::<u32>() as f32 % 600.0,
            y: -300.0 + rand::random::<u32>() as f32 % 600.0,
        };
        let force = Vector2D::from_parts(
            (-50 + rand::random::<i64>() % 100) as f64,
            (-50 + rand::random::<i64>() % 100) as f64,
        );
        commands.spawn((
            Particle::new(
                point,
                force,
                1.0 + rand::random::<u32>() as f64 % 50.0,
                PARTICLE_SIZE,
            ),
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

                if !particles[i]
                    .0
                    .collision_with(particles[y].0.as_ref())
                    .is_empty()
                {
                    println!("\n\nCollision happened!\n\n");
                    let force = particles[i].0.get_collision_force(particles[y].0.as_ref());
                    println!("Force {}", particles[i].0.get_force().get_x());
                    particles[i].0.collide(force);
                }
            }
        }

        for (mut particle, mut transform) in particles {
            let pos = particle.pos();

            let force = particle.get_force();
            if pos.x < -300.0 || pos.x > 300.0 {
                particle.collide(Vector2D::from_parts(force.get_x() * 2.0, 0.0));
            }

            if pos.y > 300.0 || pos.y < -300.0 {
                particle.collide(Vector2D::from_parts(0.0, force.get_y() * 2.0))
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
