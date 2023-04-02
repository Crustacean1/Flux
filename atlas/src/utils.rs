use std::ops;

pub struct Vec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
