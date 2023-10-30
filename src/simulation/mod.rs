use bevy::prelude::*;

use crate::physics::{Interact, Line, Move, Point, Shape, Vector2D};

struct ParticleCollider;

impl Plugin for ParticleCollider {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Particle<'a> {
    pos: &'a Point,
    force: Vector2D,
    mesh: Vec<Line<'a>>,
}
impl<'a> Particle<'a> {
    pub fn new(pos: &'a Point, force: Vector2D) -> Self {
        Self {
            pos,
            force,
            mesh: vec![Line::from_points(pos, pos)],
        }
    }
}

impl Shape for Particle<'_> {
    fn get_mesh(&self) -> &Vec<Line> {
        &self.mesh
    }
}

impl Move for Particle<'_> {
    fn get_force(&self) -> Vector2D {
        self.force.clone()
    }

    fn get_force_ref_mut(&mut self) -> &mut Vector2D {
        &mut self.force
    }
}

impl Interact for Particle<'_> {
    fn collide(&mut self, other: &mut impl Move) {
        let mut total = self.get_force() - other.get_force();
        self.force -= total.div(2.0);
        *other.get_force_ref_mut() -= total.div(2.0);
    }
}
