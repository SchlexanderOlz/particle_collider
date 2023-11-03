use self::collision::Collision;
use std::cmp::PartialOrd;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub mod collision;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn from_points(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    // Returns the right-most x-coordinate
    #[inline]
    pub fn rightest(&self) -> f32 {
        self.a.x.max(self.b.x).max(self.c.x)
    }

    // Returns the left-most x-coordinate
    #[inline]
    pub fn leftest(&self) -> f32 {
        self.a.x.min(self.b.x).min(self.c.x)
    }

    // Returns the highest y-coordinate
    #[inline]
    pub fn upest(&self) -> f32 {
        self.a.y.max(self.b.y).max(self.c.y)
    }

    // Returns the lowest y-coordinate
    #[inline]
    pub fn lowest(&self) -> f32 {
        self.a.y.min(self.b.y).min(self.c.y)
    }

    #[inline]
    pub fn points(&self) -> [&Point; 3] {
        [&self.a, &self.b, &self.c]
    }

    #[inline]
    pub fn points_mut(&mut self) -> [&mut Point; 3] {
        [&mut self.a, &mut self.b, &mut self.c]
    }

    #[inline]
    pub fn lines(&self) -> [Line; 3] {
        [
            Line::from_points(&self.a, &self.b),
            Line::from_points(&self.b, &self.c),
            Line::from_points(&self.c, &self.a),
        ]
    }

    pub fn get_collisions<'a>(&'a self, other: &'a Self) -> Vec<Collision<'a>> {
        let lines = self.lines();
        let mesh = other.points();

        let mut collisions: Vec<Collision<'a>> = Vec::new();

        for point in mesh {
            // This calculates if the point is inside of this triangle by using
            // barycentric coordinates
            let alpha = ((self.b.y - self.c.y) * (point.x - self.c.x)
                + (self.c.x - self.b.x) * (point.y - self.c.y))
                / ((self.b.y - self.c.y) * (self.a.x - self.c.x)
                    + (self.c.x - self.b.x) * (self.a.y - self.c.y));
            let beta = ((self.c.y - self.a.y) * (point.x - self.c.x)
                + (self.a.x - self.c.x) * (point.y - self.c.y))
                / ((self.b.y - self.c.y) * (self.a.x - self.c.x)
                    + (self.c.x - self.b.x) * (self.a.y - self.c.y));
            let gamma = 1.0 - alpha - beta;

            let is_inside = alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0;

            if is_inside {
                continue;
            }

            for line in other.lines() {
                let mut check = |point_line| match line.collision_with(point_line) {
                    None => (),
                    Some(coll) => collisions.push(coll),
                };

                for line in lines {
                    check(line);
                }
            }
        }
        collisions
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
    #[inline]
    pub fn rightest(&self) -> f32 {
        self.b.x
    }

    // Returns the left-most x-coordinate
    #[inline]
    pub fn leftest(&self) -> f32 {
        self.a.x
    }

    // Returns the highest y-coordinate
    #[inline]
    pub fn upest(&self) -> f32 {
        f32::max(self.a.y, self.b.y)
    }

    // Returns the lowest y-coordinate
    #[inline]
    pub fn lowest(&self) -> f32 {
        f32::min(self.a.y, self.b.y)
    }

    #[inline]
    pub fn get_steepness(&self) -> Result<f32, ()> {
        if (self.rightest() - self.leftest()).floor() == 0.0 {
            return Err(());
        }
        Ok((self.b.y - self.a.y) / (self.rightest() - self.leftest()))
    }

    #[inline]
    pub fn get_base(&self) -> f32 {
        self.a.y
    }

    pub fn angle(&self) -> f32 {
        if (self.rightest() - self.leftest()).floor() == 0.0 {
            return 90.0;
        }
        ((self.b.y - self.a.y) / (self.rightest() - self.leftest()))
            .abs()
            .atan()
            .to_degrees()
    }

    pub fn collision_with(self, other: Line<'a>) -> Option<Collision> {
        if self.leftest() > other.rightest()
            || self.rightest() < other.leftest()
            || self.upest() < other.lowest()
            || self.lowest() > other.upest()
        {
            return None;
        }

        // Calculate the linear-growth of the line
        let self_inc = self.get_steepness();
        let other_inc = other.get_steepness();

        if self_inc.is_err() && other_inc.is_err() {
            if self.a.x.round() == other.a.x.round() {
                return Some(Collision::new(self, other, self.a.x));
            }
            return None;
        }

        // Calculate the len where both lines are definied
        let def_max = self.rightest().min(other.rightest());
        let def_min = self.leftest().max(other.leftest());
        let def_area = def_min - def_max;

        if def_max.round() == def_min.round() {
            return Some(Collision::new(self, other, self.b.x));
        }

        if self_inc.is_err() || other_inc.is_err() {
            return None;
        }

        // This should be floored but causes problems currently
        if self_inc.unwrap().round() == other_inc.unwrap().round() {
            if self.a.y.floor() == other.a.y.floor() {
                return Some(Collision::new(self, other, self.a.x));
            }
            return None;
        }

        let hit = (self.a.y - other.a.y) / (self_inc.unwrap() - other_inc.unwrap());
        println!("{}", hit);
        println!("{}", def_area.floor());

        if hit < def_area.floor() || hit > 0.0 {
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn mul(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn angle(&self) -> f64 {
        f64::atan(self.x / self.y).to_degrees()
    }

    pub fn as_polar(&self) -> (f64, f64) {
        ((self.x.powi(2) + self.y.powi(2)).sqrt(), self.angle())
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
    fn get_mesh(&self) -> &[Triangle];
}

pub trait Move: Shape {
    fn get_mass(&self) -> f64;
    fn get_force(&self) -> Vector2D;
    fn get_force_ref_mut(&mut self) -> &mut Vector2D;
    fn mov(&mut self, tick: f64);
    fn get_speed(&self) -> Vector2D;
    fn set_force(&mut self, force: Vector2D);
    fn set_position(&mut self, pos: Point);
    fn apply_force(&mut self, other: Vector2D);
}

pub trait Interact<'a>: Move {
    fn collide(&mut self, other: &'a mut impl Move);
    fn collision_with(&'a self, other: &'a impl Move) -> Vec<Collision>;
    fn pos(&self) -> Point;
    fn bounce(&mut self);
}
