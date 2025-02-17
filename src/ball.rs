use crate::maths::*;

pub const GRAVITY: real = 1000.0;

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
}