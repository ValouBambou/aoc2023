use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2D {
    pub x: i64,
    pub y: i64,
}

impl Vec2D {
    pub fn dot(&self, rhs: &Vec2D) -> i64 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn manhattan_dist(&self, rhs: &Vec2D) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl<T: TryInto<i64>> From<(T, T)> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    fn from(value: (T, T)) -> Self {
        Vec2D {
            x: value.0.try_into().unwrap(),
            y: value.1.try_into().unwrap(),
        }
    }
}

impl Add for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: TryInto<i64>> Add<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    type Output = Vec2D;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.try_into().unwrap();
        Vec2D {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<T: TryInto<i64>> AddAssign<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.try_into().unwrap();
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: TryInto<i64>> Sub<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    type Output = Vec2D;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.try_into().unwrap();
        Vec2D {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign for Vec2D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: TryInto<i64>> SubAssign<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.try_into().unwrap();
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul for Vec2D {
    type Output = Vec2D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: TryInto<i64>> Mul<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    type Output = Vec2D;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.try_into().unwrap();
        Vec2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Vec2D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: TryInto<i64>> MulAssign<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.try_into().unwrap();
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div for Vec2D {
    type Output = Vec2D;
    fn div(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: TryInto<i64>> Div<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    type Output = Vec2D;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.try_into().unwrap();
        Vec2D {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for Vec2D {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: TryInto<i64>> DivAssign<T> for Vec2D
where
    <T as TryInto<i64>>::Error: Debug,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.try_into().unwrap();
        self.x /= rhs;
        self.y /= rhs;
    }
}
