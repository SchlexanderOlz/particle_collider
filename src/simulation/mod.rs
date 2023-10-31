use std::sync::Mutex;

use crate::physics::{collision::Collision, Interact, Line, Move, Point, Shape, Vector2D};
use bevy::prelude::*;

pub struct ParticleCollider;

impl Plugin for ParticleCollider {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForceTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
            .insert_resource(TickSpeed(1.0))
            .add_systems(Startup, spawn_particles)
            .add_systems(Update, move_particles);
    }
}

fn spawn_particles(mut commands: Commands) {
    for _ in 0..10 {
        let point = Point {
            x: rand::random::<u32>() as f32 % 300.0,
            y: rand::random::<u32>() as f32 % 300.0,
        };
        let force = Vector2D::from_parts(
            (-10 + rand::random::<i64>() % 20) as f64,
            (-5 + rand::random::<i64>() % 10) as f64,
        );
        commands.spawn((
            Particle::new(point, force, 10.0),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.10, 0.0, 0.75),
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(point.x, point.y, 0.0)),
                ..default()
            },
        ));

        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });

        commands.spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::ONE * 3.0),
            ..default()
        });
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
                    let force = particles[i].0.get_collision_force(particles[y].0.as_ref());
                    particles[i].0.collide(force);
                    particles[y].0.collide(force);
                }
            }
        }

        for (mut particle, mut transform) in particles {
            let pos = particle.pos();
            if pos.x < -500.0 || pos.y < -500.0 || pos.x > 500.0 || pos.y > 500.0 {
                let force = particle.get_force();
                particle.collide(Vector2D::from_parts(
                    force.get_x(),
                    force.get_y(),
                ));
            }
            particle.mov(tick_speed.0);

            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

#[derive(Resource)]
struct TickSpeed(f64);

#[derive(Resource)]
struct ForceTimer(Timer);

#[derive(Component, Clone, Copy)]
pub struct Particle {
    pos: Point,
    force: Vector2D,
    mass: f64,
}

impl Particle {
    pub fn new(pos: Point, force: Vector2D, mass: f64) -> Self {
        Self { pos, force, mass }
    }
}

impl Shape for Particle {
    fn get_mesh(&self) -> Vec<Line> {
        vec![Line::from_points(&self.pos, &self.pos)]
    }
}

impl Move for Particle {
    fn get_force(&self) -> Vector2D {
        self.force.clone()
    }

    fn get_force_ref_mut(&mut self) -> &mut Vector2D {
        &mut self.force
    }

    fn mov(&mut self, tick: f64) {
        let speed = self.force.as_speed(self.mass);
        self.pos.x += (speed.get_x() * tick) as f32;
        self.pos.y += (speed.get_y() * tick) as f32;
    }

    fn get_speed(&self) -> Vector2D {
        self.force.as_speed(self.mass)
    }
}

impl<'a> Interact<'a> for Particle {
    fn collide(&mut self, other: Vector2D) {
        self.force -= other.div(2.0);
        // *other.get_force_ref_mut() -= total.div(2.0);
    }

    fn get_collision_force(&self, other: &impl Move) -> Vector2D {
        self.get_force() - other.get_force()
    }

    fn has_collision(&'a self, other: &'a impl Move) -> Option<Collision> {
        for line in self.get_mesh() {
            for other_line in other.get_mesh() {
                let collision = line.clone().has_collision(other_line.clone());
                if collision.is_none() {
                    continue;
                }

                return collision;
            }
        }
        None
    }

    fn pos(&self) -> Point {
        self.pos.clone()
    }
}
