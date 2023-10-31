use self::collision::Collision;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

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

    pub fn rightest(&self) -> &'a Point {
        &self.b
    }

    pub fn leftest(&self) -> &'a Point {
        &self.a
    }

    pub fn get_steepness(&self) -> f32 {
        ((self.b.y - self.a.y) / (self.b.x - self.a.x)) as f32
    }

    pub fn get_base(&self) -> f32 {
        self.a.y
    }

    pub fn has_collision(self, other: Line<'a>) -> Option<Collision> {
        if self.a.x > other.b.x {
            return None;
        }

        if self.b.x < other.a.x {
            return None;
        }

        // Calculate the linear-growth of the line
        let self_inc = self.get_steepness();
        let other_inc = other.get_steepness();

        // Calculate the area where both lines are definied
        let def_max = self.b.x.min(other.b.x);
        let def_min = self.a.x.max(other.a.x);

        if self_inc == other_inc {
            if self.a.y == other.a.y {
                return Some(Collision::new(self, other, 0.0));
            }
            return None;
        }

        let hit = (self.a.y - other.a.y) as f32 / (self_inc - other_inc);
        if def_max < hit || def_min > hit {
            return None;
        }
        Some(Collision::new(self, other, hit))
    }

    pub fn at(&self, x: f32) -> f32 {
        self.get_steepness() * x + self.get_base()
    }

    pub fn has_point_collision(&self, point: Point) {}
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
}

pub trait Interact<'a>: Move {
    fn collide(&mut self, other: Vector2D);
    fn has_collision(&'a self, other: &'a impl Move) -> Option<Collision>;
    fn get_collision_force(&self, other: &impl Move) -> Vector2D;
    fn pos(&self) -> Point;
    fn bounce(&mut self);
}
