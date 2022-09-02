use std::ops;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    // Do I need an overload here?
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        let e = [e0, e1, e2];
        Vec3 { e }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /// Multiplies parts of two Vec3 objects
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::{Vec3};
    /// let res = Vec3::new(1.0, 2.0, 3.0).dot(Vec3::new(4.0, 5.0, 6.0));
    /// assert_eq!(res, 32.0);
    /// ```
    pub fn dot(&self, v: Vec3) -> f64 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Multiples parts of two Vec3 objects and adds the parts together
    ///
    /// # Examples
    /// ```
    /// use ray_tracer::{Vec3};
    /// let src = Vec3::new(6.0, 9.0, 12.0);
    /// assert_eq!(src.length_squared(), 261.0);
    /// ```
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn to_color_string(&self) -> String {
        format!(
            "{} {} {}",
            (255.999 * self.x()) as u8,
            (255.999 * self.y()) as u8,
            (255.999 * self.z()) as u8
        )
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

#[test]
fn can_negate_vec3() {
    let src = Vec3::new(1.0, 2.0, 3.0);
    let res = Vec3::new(-1.0, -2.0, -3.0);
    assert_eq!(-src, res);
    assert_eq!(src[0], 1.0);
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        };
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        };
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1_f64 / other
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

#[test]
fn can_add_vec3() {
    let res = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(res, Vec3::new(5.0, 7.0, 9.0));
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

#[test]
fn can_subtract_vec3() {
    let res = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 8.0, 16.0);
    assert_eq!(res, Vec3::new(-3.0, -6.0, -13.0));
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

#[test]
fn can_multiply_vec3_by_vec3() {
    let res = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(res, Vec3::new(4.0, 10.0, 18.0));
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Self::Output {
        Vec3 {
            e: [other * self.e[0], other * self.e[1], other * self.e[2]],
        }
    }
}

#[test]
fn can_multiply_vec3_by_f64() {
    let res = Vec3::new(1.0, 2.0, 3.0) * 3.0;
    assert_eq!(res, Vec3::new(3.0, 6.0, 9.0));
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

#[test]
fn can_multiply_f64_by_vec3() {
    let res = 4.0 * Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(res, Vec3::new(4.0, 8.0, 12.0));
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output {
        (1_f64 / other) * self
    }
}

#[test]
fn can_divide_vec3_by_f64() {
    let res = Vec3::new(3.0, 6.0, 9.0) / 3.0;
    assert_eq!(res, Vec3::new(1.0, 2.0, 3.0));
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}