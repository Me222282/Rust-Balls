use cgmath::InnerSpace;

use crate::ball::*;
use crate::maths::*;
use std::vec::Vec;
use std::slice::Iter;

pub struct Physics
{
    balls: Vec<Ball>,
    bounds: Vec4
}

fn resolve_collisions(a: &mut Ball, b: &mut Ball)
{
    let sum_radius = a.radius + b.radius;
    let mut axis: Vec2 = a.location - b.location;
    let mut dist = axis.magnitude2();
    
    if dist >= (sum_radius * sum_radius) { return; }
    
    if dist == 0.0
    {
        axis = (a.velocity() - b.velocity()).normalize();
    }
    else
    {
        dist = dist.sqrt();
        axis /= dist;
    }
    let diff = dist - sum_radius;
    let scale = diff * 0.5;
    let offset = axis * scale;
    
    let inv = 1.0 / sum_radius;
    let mass_ratio_a = a.radius * inv;
    let mass_ratio_b = b.radius * inv;
    
    a.location -= offset * mass_ratio_a;
    b.location += offset * mass_ratio_b;
}

fn clip_to_bounds(b: &mut Ball, bounds: Vec4)
{
    let r = b.radius;
    let l = b.location;
    
    if l.x - r < bounds.x
    {
        b.location.x = bounds.x + r;
    }
    if l.x + r > bounds.y
    {
        b.location.x = bounds.y - r;
    }
    if l.y + r > bounds.z
    {
        b.location.y = bounds.z - r;
    }
    if l.y - r < bounds.w
    {
        b.location.y = bounds.w + r;
    }
}

impl Physics {
    pub fn apply_phsyics(&mut self, dt: real)
    {
        for b in self.balls.iter_mut()
        {
            clip_to_bounds(b, self.bounds);
        }
        
        let l = self.balls.len();
        for i in 0..l
        {
            let ni = i + 1;
            let (v1, v2) = self.balls.split_at_mut(ni); 
            let b1 = &mut v1[i];
            for j in 0..(l - ni)
            {
                let b2 = &mut v2[j];
                resolve_collisions(b1, b2);
            }
        }
        
        for b in self.balls.iter_mut()
        {
            b.verlet(dt);
        }
    }
    
    pub fn apply_phsyics_sub(&mut self, dt: real, sub: u8)
    {
        let dt = dt / sub as real;
        
        for _ in 0..sub
        {
            self.apply_phsyics(dt);
        }
    }
    
    pub fn new(bounds: Vec4) -> Physics
    {
        return Physics {
            balls: Vec::with_capacity(100),
            bounds: bounds
        };
    }
    
    pub fn add(&mut self, b: Ball)
    {
        self.balls.push(b);
    }
    
    pub fn count(&self) -> usize
    {
        return self.balls.len();
    }
    
    pub fn get_bounds(&self) -> Vec4
    {
        return self.bounds;
    }
    pub fn set_bounds(&mut self, bounds: Vec4)
    {
        self.bounds = bounds;
    }
}

impl<'a> IntoIterator for &'a Physics {
    type Item = &'a Ball;
    type IntoIter = Iter<'a, Ball>;

    fn into_iter(self) -> Self::IntoIter {
        return self.balls.iter();
    }
}