use crate::physics::{collision::Collision, Interact, Line, Move, Point, Shape, Vector2D};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Particle {
    pos: Point,
    mesh: Vec<Point>,
    force: Vector2D,
    mass: f64,
}

impl Particle {
    pub fn new(pos: Point, force: Vector2D, mass: f64, size: f32) -> Self {
        // Uper left
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

        Self {
            pos,
            force,
            mass,
            mesh: vec![a, b, c, d],
        }
    }
}

impl Shape for Particle {
    fn get_mesh(&self) -> Vec<Line> {
        let mut lines: Vec<Line> = self
            .mesh
            .windows(2)
            .map(|pair| Line::from_points(&pair[0], &pair[1]))
            .collect();

        lines.push(Line::from_points(
            self.mesh.first().unwrap(),
            self.mesh.last().unwrap(),
        ));
        lines
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
        let speed = self.force.as_speed(self.mass);
        self.pos.x += (speed.get_x() * tick) as f32;
        self.pos.y += (speed.get_y() * tick) as f32;
        self.mesh.iter_mut().for_each(|x| {
            x.x += (speed.get_x() * tick) as f32;
            x.y += (speed.get_y() * tick) as f32
        });
    }

    fn get_speed(&self) -> Vector2D {
        self.force.as_speed(self.mass)
    }
}

impl<'a> Interact<'a> for Particle {
    fn collide(&mut self, other: Vector2D) {
        self.force -= other;
        // *other.get_force_ref_mut() -= total.div(2.0);
    }

    fn bounce(&mut self) {
        self.force = -self.force;
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
