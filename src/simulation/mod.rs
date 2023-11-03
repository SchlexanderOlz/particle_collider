use self::particle::Particle;
use crate::physics::{Interact, Move, Point, Vector2D};
use bevy::prelude::*;
use rand::Rng;
use std::cell::RefCell;

pub mod particle;

const GAME_SIZE: f32 = 300.0;

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
    let mut random = rand::thread_rng();
    for _ in 0..100 {
        let mut rand = || -GAME_SIZE + random.gen::<u32>() as f32 % GAME_SIZE * 2.0;

        let point = Point {
            x: rand(),
            y: rand(),
        };

        let mut rand = || (-50 + random.gen::<i64>() % 100) as f64;

        let force = Vector2D::new(rand(), rand());
        let particle_size = 10.0 + random.gen::<u32>() as f32 % 5.0;
        commands.spawn((
            Particle::new(point, force, particle_size as f64, particle_size),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.10, 0.0, 0.75),
                    custom_size: Some(Vec2::new(particle_size, particle_size)),
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
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let view: Vec<_> = query.iter_mut().map(|x| RefCell::new(x.0)).collect();

    for i in 0..view.len() {
        for j in i + 1..view.len() {
            let other = &view[i];
            let particle = &view[j];

            if particle.borrow().as_ref() == other.borrow().as_ref() {
                continue;
            }

            // TODO: Add advanced collision checking here
            if !particle
                .borrow()
                .collision_with(other.borrow().as_ref())
                .is_empty()
            {
                particle.borrow_mut().collide(other.borrow_mut().as_mut());
            }
        }
    }

    for (mut particle, mut transform) in &mut query {
        let pos = particle.get_pos();

        let force = particle.get_force();
        if pos.x < -GAME_SIZE || pos.x > GAME_SIZE {
            particle.set_force(Vector2D::new(-force.get_x(), force.get_y()));
        }

        if pos.y > GAME_SIZE || pos.y < -GAME_SIZE {
            particle.set_force(Vector2D::new(force.get_x(), -force.get_y()));
        }
        particle.mov(tick_speed.0);
        let pos = particle.get_pos();

        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

#[derive(Resource)]
struct TickSpeed(f64);

#[derive(Resource)]
struct ForceTimer(Timer);
