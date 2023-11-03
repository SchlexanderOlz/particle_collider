use std::cell::RefCell;

use crate::physics::{Interact, Move, Point, Vector2D};
use bevy::{prelude::*, utils::HashSet};

use self::particle::Particle;

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
    const PARTICLE_SIZE: f32 = 20.0;
    for _ in 0..5 {
        let point = Point {
            x: -GAME_SIZE + rand::random::<u32>() as f32 % GAME_SIZE * 2.0,
            y: 0.0,
        };
        let force = Vector2D::from_parts((-50 + rand::random::<i64>() % 100) as f64, 0.0);
        commands.spawn((
            Particle::new(point, force, 5.0, PARTICLE_SIZE),
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
        let view: Vec<_> = query.iter_mut().map(|x| RefCell::new(x.0)).collect();

        for i in 0..view.len() {
            for j in i + 1..view.len() {
                let other = &view[i];
                let particle = &view[j];

                if particle.borrow().as_ref() == other.borrow().as_ref() {
                    continue;
                }

                if !particle
                    .borrow()
                    .collision_with(other.borrow().as_ref())
                    .is_empty()
                {
                    println!("\n\nCollision happened!\n\n");
                    particle.borrow_mut().collide(other.borrow_mut().as_mut());
                }
            }
        }

        for (mut particle, mut transform) in &mut query {
            let pos = particle.pos();

            let force = particle.get_force();
            if pos.x < -GAME_SIZE || pos.x > GAME_SIZE {
                particle.apply_force(Vector2D::from_parts(-force.get_x() * 2.0, 0.0));
            }

            if pos.y > GAME_SIZE || pos.y < -GAME_SIZE {
                particle.apply_force(Vector2D::from_parts(0.0, -force.get_y() * 2.0))
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
