use crate::physics::{Interact, Move, Point, Shape, Triangle, Vector2D};
use bevy::prelude::*;

#[derive(Component, Clone, PartialEq)]
pub struct Particle {
    pos: Point,
    mesh: [Triangle; 2],
    force: Vector2D,
    mass: f64,
}

impl Particle {
    pub fn new(pos: Point, force: Vector2D, mass: f64, size: f32) -> Self {
        // Upper left
        let a = Point {
            x: pos.x - size / 2.0,
            y: pos.y + size / 2.0,
        };

        // Upper right
        let b = Point {
            x: pos.x + size / 2.0,
            y: pos.y + size / 2.0,
        };

        // Lower right
        let c = Point {
            x: pos.x + size / 2.0,
            y: pos.y - size / 2.0,
        };

        // Lower left
        let d = Point {
            x: pos.x - size / 2.0,
            y: pos.y - size / 2.0,
        };

        let mesh = [Triangle::new(a, b, d), Triangle::new(a, c, d)];

        Self {
            pos,
            force,
            mass,
            mesh,
        }
    }
}

impl Shape for Particle {
    fn get_mesh(&self) -> &[Triangle] {
        &self.mesh
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
        let speed = self.get_speed();

        let tick_x = (speed.get_x() * tick) as f32;
        let tick_y = (speed.get_y() * tick) as f32;

        self.pos.x += tick_x;
        self.pos.y += tick_y;

        self.mesh.iter_mut().for_each(|triangle| {
            triangle.points_mut().iter_mut().for_each(|point| {
                point.x += tick_x;
                point.y += tick_y;
            });
        });
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn set_force(&mut self, force: Vector2D) {
        self.force = force
    }

    fn get_pos(&self) -> Point {
        self.pos
    }

    fn set_position(&mut self, pos: Point) {
        self.pos.x = pos.x;
        self.pos.y = pos.y;
    }
}

impl<'a> Interact<'a> for Particle {}
