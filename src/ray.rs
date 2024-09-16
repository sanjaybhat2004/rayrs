use crate::vec;

use vec::Vec3 as Point3;
use vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig : Point3,
    pub dir: Vec3,
}

impl Ray {

    pub(crate) fn origin(self) -> Point3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn at(self, t: f64) -> Vec3{
        self.orig + self.dir*t
    }
}


