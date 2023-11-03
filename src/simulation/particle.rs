use crate::physics::{
    collision::Collision, Interact, Line, Move, Point, Shape, Triangle, Vector2D,
};
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

        let mesh = [
            Triangle::from_points(a, b, d),
            Triangle::from_points(a, c, d),
        ];

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
        // TODO: Change how this is done
        let speed = self.get_speed();
        self.pos.x += (speed.get_x() * tick) as f32;
        self.pos.y += (speed.get_y() * tick) as f32;

        self.mesh.iter_mut().for_each(|triangle| {
            triangle.points_mut().iter_mut().for_each(|point| {
                point.x += (speed.get_x() * tick) as f32;
                point.y += (speed.get_y() * tick) as f32;
            });
        });
    }

    fn get_speed(&self) -> Vector2D {
        self.force.as_speed(self.mass)
    }

    fn set_position(&mut self, pos: Point) {
        self.pos.x = pos.x;
        self.pos.y = pos.y;
    }

    fn get_mass(&self) -> f64 {
        self.mass
    }

    fn set_force(&mut self, force: Vector2D) {
        self.force = force
    }

    fn apply_force(&mut self, other: Vector2D) {
        self.force += other;
    }
}

impl<'a> Interact<'a> for Particle {
    fn collide(&mut self, other: &'a mut impl Move) {
        let diff = self.get_speed() - other.get_speed();
        let v1 = diff - diff.div(other.get_mass() / self.get_mass());
        let v2 = -diff + diff.div(other.get_mass() / self.get_mass());
        println!("1: {}", v1.get_x());
        println!("2: {}", v2.get_x());
        self.force = -self.force + v1.mul(self.mass);
        other.set_force(-other.get_force() + v2.mul(other.get_mass()));
    }

    fn bounce(&mut self) {
        self.force = -self.force;
    }

    fn collision_with(&'a self, other: &'a impl Move) -> Vec<Collision> {
        let mut all_collisions = Vec::new();
        for triangle in self.get_mesh() {
            for other_triangle in other.get_mesh() {
                let collisions = triangle.get_collisions(other_triangle);
                all_collisions.extend(collisions);
            }
        }
        all_collisions
    }

    fn pos(&self) -> Point {
        self.pos.clone()
    }
}
