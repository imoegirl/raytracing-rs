
use std::ops::{Add, Sub, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::PartialEq;


#[derive(Debug, Copy, Clone)]
pub struct Vector3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        Self{ x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    pub fn one() -> Self {
        Self{ x: 1.0, y: 1.0, z: 1.0, w: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn random() -> Self {
        Vector3 {
            x: crate::random_double(),
            y: crate::random_double(),
            z: crate::random_double(),
            w: 0.0,
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vector3 {
            x: crate::random_double_of_range(min, max),
            y: crate::random_double_of_range(min, max),
            z: crate::random_double_of_range(min, max),
            w: 0.0
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Self {
        let a = crate::random_double_of_range(0.0, std::f64::consts::PI);
        let z = crate::random_double_of_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
            w: 0.0
        }
    }

    pub fn random_in_hemisphere(normal: Vector3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        }else{
            -in_unit_sphere
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let num = self.length();
        let nx = self.x / num;
        let ny = self.y / num;
        let nz = self.z / num;
        let w = self.w;
        

        Self {
            x: if nx.is_finite() { nx } else { 0.0 },
            y: if ny.is_finite() { ny } else { 0.0 },
            z: if nz.is_finite() { nz } else { 0.0 },
            w: w
        }
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
            w: u.w
        }
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w
        }
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, num: f64) -> Self {
        Self {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
            w: self.w
        }
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, num: f64) -> Self {
        let nx = self.x / num;
        let ny = self.y / num;
        let nz = self.z / num;
        let w = self.w;

        Self {
            x: if nx.is_finite() { nx } else { 0.0 },
            y: if ny.is_finite() { ny } else { 0.0 },
            z: if nz.is_finite() { nz } else { 0.0 },
            w: w
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w
        }
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, num: f64) {
        *self = Self {
            x: self.x * num,
            y: self.y * num,
            z: self.z * num,
            w: self.w
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, num: f64){
        let nx = self.x / num;
        let ny = self.y / num;
        let nz = self.z / num;
        let w = self.w;

        *self = Self {
            x: if nx.is_finite() { nx } else { 0.0 },
            y: if ny.is_finite() { ny } else { 0.0 },
            z: if nz.is_finite() { nz } else { 0.0 },
            w: w
        }
    }
}


impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z &&
        self.w == other.w
    }
}


// impl <'a, 'b> Add<&'b Vector3> for &'a Vector3 {
//     type Output = Vector3;

//     fn add(self, other: &'b Vector3) -> Vector3 {
//         Vector3 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//             w: self.w
//         }
//     }
// }

// impl Mul<f64> for &Vector3 {
//     type Output = Vector3;

//     fn mul(self, num: f64) -> Vector3 {
//         Vector3 {
//             x: self.x * num,
//             y: self.y * num,
//             z: self.z * num,
//             w: self.w
//         }
//     }
// }