#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        let e: [f64; 3] = [0.0, 0.0, 0.0];
        Self { e }
    }

    pub fn x(self) -> f64 {self.e[0]}
    pub fn y(self) -> f64 {self.e[1]}
    pub fn z(self) -> f64 {self.e[2]}
    pub fn length(self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
    }

    pub fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    
}



use std::{ops::Add, ops::Sub};


// impl dot for Vec3 {
pub fn dot (a: Vec3, b:Vec3) -> f64 {
    a.x()*b.x() + a.y()*b.y() + a.z()*b.z()
}

// }

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other : Vec3) -> Vec3 {
        let e : [f64; 3] = [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]];
        Vec3{e}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other : Vec3) -> Vec3 {
        let e : [f64; 3] = [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]];
        Vec3{e}
    }
}
use std::ops::{Index, IndexMut, Neg};

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        let e: [f64; 3] = [-self.e[0], -self.e[1], -self.e[2]];
        
        Vec3{e}
    }
}

use std::ops::{Mul, Div};

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        let e : [f64; 3] = [self.x() * rhs, self.y() * rhs, self.z() * rhs];
        Vec3{ e }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let e : [f64; 3] = [rhs.x() * self, rhs.y() * self, rhs.z() * self];
        Vec3{ e }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        let e : [f64; 3] = [self.x() / rhs, self.y() / rhs, self.z() / rhs];
        Vec3{ e }
    }
}

impl Div<i32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Vec3 {
        let e : [f64; 3] = [self.x() / rhs as f64, self.y() / rhs as f64, self.z() / rhs as f64];
        Vec3{ e }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return  v / v.length();
}




