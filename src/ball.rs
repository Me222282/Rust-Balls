use rand::Rng;
use std::ops::Range;

use crate::maths::*;

pub const GRAVITY: real = 1000.0;

#[derive(Copy, Clone)]
pub struct Ball
{
    pub location: Vec2,
    old_pos: Vec2,
    pub radius: real,
    pub colour: Colour
}

impl Ball
{
    pub fn verlet(&mut self, dt: real)
    {
        let vel = self.velocity();
        self.old_pos = self.location;
        self.location += vel - Vec2::new(0.0, GRAVITY * dt * dt);
    }
    pub fn velocity(&self) -> Vec2
    {
        return self.location - self.old_pos;
    }
    
    pub fn new_location(b: Ball, l: Vec2) -> Ball
    {
        return Ball {
            location: l,
            old_pos: b.old_pos,
            radius: b.radius,
            colour: b.colour
        };
    }
    
    pub fn new(l: Vec2, r: real, c: Colour) -> Ball
    {
        return Ball {
            location: l,
            old_pos: l,
            radius: r,
            colour: c
        };
    }
    pub fn random<T: Rng>(rng: &mut T, pos: &Range<Vec2>, radius: Range<real>) -> Ball
    {
        let x = rng.random_range(pos.start.x..pos.end.x);
        let y = rng.random_range(pos.start.y..pos.end.y);
        let rad = rng.random_range(radius);
        
        let r = rng.random::<u8>();
        let g = rng.random::<u8>();
        let b = rng.random::<u8>();
        
        return Self::new(vec2(x, y), rad, colour(r, g, b));
    }
}