use crate::physics::{Interact, Line, Move, Point, Shape, Vector2D};
use bevy::prelude::*;

pub struct ParticleCollider;

impl Plugin for ParticleCollider {
    fn build(&self, app: &mut App) {
        app.insert_resource(ForceTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(TickSpeed(1.0))
            .add_systems(Startup, spawn_particles)
            .add_systems(Update, move_particles);
    }
}

fn spawn_particles(mut commands: Commands) {
    for i in 0..100 {
        let point = Point {
            x: 10.0 + i as f32,
            y: 10.0,
        };
        let force = Vector2D::from_parts(10.0, 10.0);
        commands.spawn(Particle::new(point, force, 1.0));
    }
}

fn move_particles(
    time: Res<Time>,
    mut timer: ResMut<ForceTimer>,
    tick_speed: Res<TickSpeed>,
    mut query: Query<&mut Particle>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut particle in &mut query {
            particle.mov(tick_speed.0);
        }
    }
}

#[derive(Resource)]
struct TickSpeed(f64);

#[derive(Resource)]
struct ForceTimer(Timer);

#[derive(Component)]
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
}

impl Interact for Particle {
    fn collide(&mut self, other: &mut impl Move) {
        let mut total = self.get_force() - other.get_force();
        self.force -= total.div(2.0);
        *other.get_force_ref_mut() -= total.div(2.0);
    }

    fn pos(&self) -> Point {
        self.pos.clone()
    }
}
