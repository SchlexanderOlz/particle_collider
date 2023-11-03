use self::collision::Collision;
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
            x: self.x.round(),
            y: self.y.round(),
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
    pub fn new(a: Point, b: Point, c: Point) -> Self {
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
            Line::new(&self.a, &self.b),
            Line::new(&self.b, &self.c),
            Line::new(&self.c, &self.a),
        ]
    }

    pub fn get_collisions<'a>(&'a self, other: &'a Self) -> Vec<Collision<'a>> {
        let lines = self.lines();

        // TODO: Return some object here which has the option to get all collisions
        let mut collisions: Vec<Collision<'a>> = Vec::new();

        for point in other.points() {
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

            // Check if the point is on the lines or inside the triangle
            let is_inside = alpha >= 0.0
                && beta >= 0.0
                && gamma >= 0.0
                && alpha <= 1.0
                && beta <= 1.0
                && gamma <= 1.0;

            if is_inside {
                continue;
            }

            for line in other.lines() {
                for point_line in lines {
                    if let Some(collision) = line.collision_with(point_line) {
                        collisions.push(collision)
                    }
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
    pub fn new(a: &'a Point, b: &'a Point) -> Line<'a> {
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
        if self.rightest() - self.leftest() == 0.0 {
            return Err(());
        }
        Ok((self.b.y - self.a.y) / (self.rightest() - self.leftest()))
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
            if self.a.y.round() == other.a.y.round() {
                return Some(Collision::new(self, other, self.a.x));
            }
            return None;
        }

        let hit = (self.a.y - other.a.y) / (self_inc.unwrap() - other_inc.unwrap());
        if hit < def_area.round() || hit > 0.0 {
            return None;
        }
        Some(Collision::new(self, other, hit))
    }

    #[inline]
    pub fn at(&self, x: f32) -> f32 {
        self.get_steepness().unwrap_or(0.0) * x + self.lowest()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn get_x(&self) -> f64 {
        self.x.clone()
    }

    #[inline]
    pub fn get_y(&self) -> f64 {
        self.y.clone()
    }

    #[inline]
    pub fn div(&self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    #[inline]
    pub fn mul(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    #[inline]
    pub fn get_angle(&self) -> f64 {
        f64::atan(self.x / self.y).to_degrees()
    }

    // Returns the vector in polar-form. The first element in the tuple is the
    // total vector-value and the second element is the angle
    #[inline]
    pub fn as_polar(&self) -> (f64, f64) {
        ((self.x.powi(2) + self.y.powi(2)).sqrt(), self.get_angle())
    }

    #[inline]
    pub fn as_speed(&self, mass: f64) -> Self {
        Self {
            x: self.x / mass,
            y: self.y / mass,
        }
    }
}

impl Add for Vector2D {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2D {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector2D {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2D {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vector2D {
    type Output = Self;
    #[inline]
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

    #[inline]
    fn get_speed(&self) -> Vector2D {
        self.get_force().as_speed(self.get_mass())
    }
    fn set_force(&mut self, force: Vector2D);
    fn set_position(&mut self, pos: Point);
    fn get_pos(&self) -> Point;
}

pub trait Interact<'a>: Move {
    fn collide(&mut self, other: &'a mut impl Move) {
        let diff = self.get_speed() - other.get_speed();
        let v2 = -diff.div(other.get_mass() / self.get_mass());
        let v1 = diff + v2;

        self.set_force(other.get_force() - v1.mul(self.get_mass()));
        other.set_force(-self.get_force() + v2.mul(other.get_mass()));
    }

    fn collision_with(&'a self, other: &'a impl Move) -> Vec<Collision> {
        let mut collisions = Vec::new();
        for triangle in self.get_mesh() {
            for other_triangle in other.get_mesh() {
                let triangle_collisions = triangle.get_collisions(other_triangle);
                collisions.extend(triangle_collisions);
            }
        }
        collisions
    }
}
