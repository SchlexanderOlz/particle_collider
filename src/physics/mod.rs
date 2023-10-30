use std::ops::{Add, AddAssign};

use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Component)]
pub struct Particle {
    pos: Point,
}

impl Particle {
    pub fn new(pos: Point) -> Self {
        Self { pos }
    }
}

#[derive(Debug)]
pub struct Vector2D {
    x_force: f32,
    y_force: f32,
}

impl Vector2D {
    pub fn new() -> Self {
        Self {
            x_force: 0.0,
            y_force: 0.0,
        }
    }

    pub fn from_movement(x_force: f32, y_force: f32) -> Self {
        Self { x_force, y_force }
    }

    pub fn from_moving_object(mass: f32, speed: f32, angle: f32) -> Self {
        let force = mass * speed;

        let x = angle.cos() / force;
        let y = (force.powi(2) - x.powi(2)).sqrt();
        Self {
            x_force: x,
            y_force: y,
        }
    }

    pub fn get_force_x(&self) -> f32 {
        self.x_force.clone()
    }

    pub fn get_force_y(&self) -> f32 {
        self.y_force.clone()
    }

    pub fn get_force(&self) -> f32 {
        (self.x_force.powi(2) + self.y_force.powi(2)).sqrt()
    }

    pub fn get_angle(&self) -> f32 {
        todo!()
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::from_movement(self.x_force + rhs.x_force, self.y_force + rhs.y_force)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x_force += rhs.x_force;
        self.y_force += rhs.y_force;
    }
}

