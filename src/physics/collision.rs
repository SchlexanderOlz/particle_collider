use super::{Line, Point};

#[derive(Debug)]
pub struct Collision<'a> {
    line_a: Line<'a>,
    line_b: Line<'a>,
    hit: f32,
}

impl<'a> Collision<'a> {
    pub fn new(line_a: Line<'a>, line_b: Line<'a>, hit: f32) -> Collision<'a> {
        Self {
            line_a,
            line_b,
            hit,
        }
    }

    pub fn pos(&self) -> Point {
        Point {
            x: self.hit,
            y: self.line_a.at(self.hit),
        }
    }

    pub fn angle(&self) -> f32 {
        (self.line_a.angle() - self.line_b.angle()).abs()
    }
}
