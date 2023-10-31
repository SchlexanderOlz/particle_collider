use self::collision::Collision;
use std::{
    io::Error,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

pub mod collision;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn approx(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line<'a> {
    pub a: &'a Point,
    pub b: &'a Point,
}

// It is guaranteed that a.x <= b.x
impl<'a> Line<'a> {
    pub fn from_points(a: &'a Point, b: &'a Point) -> Line<'a> {
        if a.x <= b.x {
            Line { a, b }
        } else {
            Line { a: b, b: a }
        }
    }

    // Returns the right-most x-coordinate
    pub fn rightest(&self) -> f32 {
        self.b.x.clone()
    }

    // Returns the left-most x-coordinate
    pub fn leftest(&self) -> f32 {
        self.a.x.clone()
    }

    // Returns the highest y-coordinate
    pub fn upest(&self) -> f32 {
        f32::max(self.a.y, self.b.y)
    }

    // Returns the lowest y-coordinate
    pub fn lowest(&self) -> f32 {
        f32::min(self.a.y, self.b.y)
    }

    pub fn get_steepness(&self) -> Result<f32, ()> {
        if self.b.x - self.a.x == 0.0 {
            return Err(());
        }
        Ok((self.b.y - self.a.y) / (self.b.x - self.a.x))
    }

    pub fn get_base(&self) -> f32 {
        self.a.y
    }

    pub fn has_collision(self, other: Line<'a>) -> Option<Collision> {
        if self.leftest() > other.rightest()
            || self.rightest() < other.leftest()
            || self.upest() < other.lowest()
            || self.lowest() > other.upest()
        {
            return None;
        }
        return Some(Collision::new(self, other, 0.0));

        // Calculate the linear-growth of the line
        let self_inc = self.get_steepness();
        let other_inc = other.get_steepness();

        if self_inc.is_err() && other_inc.is_err() {
            if self.a.x.floor() == other.a.x.floor() {
                return Some(Collision::new(self, other, self.a.x));
            }
            return None;
        }

        // Calculate the len where both lines are definied
        let def_max = self.rightest().min(other.rightest());
        let def_min = self.leftest().max(other.leftest());

        println!("Got till here {}, {}", def_min, def_max);
        if def_max.floor() == def_min.floor() {
            return Some(Collision::new(self, other, self.b.x));
        }

        if self_inc.unwrap() == other_inc.unwrap() {
            if self.a.y.floor() == other.a.y.floor() {
                return Some(Collision::new(self, other, self.a.x));
            }
            return None;
        }

        let hit = (self.a.y - other.a.y) / (self_inc.unwrap() - other_inc.unwrap());
        if def_max < hit || def_min > hit {
            return None;
        }
        Some(Collision::new(self, other, hit))
    }

    fn handle_straight(&self) {}

    pub fn at(&self, x: f32) -> f32 {
        let steepness = self.get_steepness().unwrap_or(0.0);
        steepness * x + self.get_base()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn from_parts(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x.clone()
    }

    pub fn get_y(&self) -> f64 {
        self.y.clone()
    }

    pub fn get_total(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn get_angle(&self) -> f32 {
        todo!()
    }

    pub fn div(&self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    pub fn as_speed(&self, mass: f64) -> Self {
        Self {
            x: self.x / mass,
            y: self.y / mass,
        }
    }
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::from_parts(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D::from_parts(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vector2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}
pub trait Shape {
    fn get_mesh(&self) -> Vec<Line>;
}

pub trait Move: Shape {
    fn get_force(&self) -> Vector2D;
    fn get_force_ref_mut(&mut self) -> &mut Vector2D;
    fn mov(&mut self, tick: f64);
    fn get_speed(&self) -> Vector2D;
    fn get_collision_force(&self, other: &impl Move) -> Vector2D {
        self.get_force() - other.get_force()
    }
}

pub trait Interact<'a>: Move {
    fn collide(&mut self, other: Vector2D);
    fn has_collision(&'a self, other: &'a impl Move) -> Option<Collision>;
    fn pos(&self) -> Point;
    fn bounce(&mut self);
}
