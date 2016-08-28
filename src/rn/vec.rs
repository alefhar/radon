use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Neg};

use num::traits::Float;

pub trait Vector<T = Self> {
  type Output;

  fn length(&self) -> T;

  fn normalize(&mut self) -> ();

  fn normalized(&self) -> Self::Output;

  fn dot(&self, v: &Self::Output) -> T;
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3<T : Float> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl <T: Float> Vec3<T> {
  pub fn new(x: T, y: T, z: T) -> Vec3<T> {
    Vec3{x: x, y: y, z: z}
  }
   
  pub fn cross(&self, v: Vec3<T>) -> Vec3<T> {
    Vec3::new(self.y * v.z - self.z * v.y, self.z * v.x - self.x * v.z, self.x * v.y - self.y * v.x) 
  }
}

impl <T: Float> Vector<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn length(&self) -> T {
    let len_sqr = self.x * self.x + self.y * self.y + self.z * self.z;
    len_sqr.sqrt()
  }

  fn normalize(&mut self) -> () {
    let length = self.length();
    self.x = self.x / length;
    self.y = self.y / length;
    self.z = self.z / length;
  }

  fn normalized(&self) -> Self::Output {
    let mut ret = Vec3::new(self.x, self.y, self.z);
    ret.normalize();
    return ret;
  }

  fn dot(&self, v: &Self::Output) -> T {
    self.x * v.x + self.y * v.y + self.z * v.z
  }
}

impl <T: Float> AddAssign for Vec3<T> {
  fn add_assign(&mut self, other: Vec3<T>) {
    self.x = self.x + other.x;
    self.y = self.y + other.y;
    self.z = self.z + other.z;
  }
}

impl <T : Float> Add for Vec3<T> {
  type Output = Vec3<T>;

  fn add(self, other: Vec3<T>) -> Self::Output {
    Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}

impl <T: Float> Neg for Vec3<T> {
  type Output = Vec3<T>;

  fn neg(self) -> Self::Output {
    Vec3::new(-self.x, -self.y, -self.z)
  }
}

impl <T: Float> SubAssign for Vec3<T> {
  fn sub_assign(&mut self, other: Vec3<T>) {
    self.x = self.x - other.x;
    self.y = self.y - other.y;
    self.z = self.z - other.z;
  }
}

impl <T: Float> Sub for Vec3<T> {
  type Output = Vec3<T>;

  fn sub(self, other: Vec3<T>) -> Self::Output {
    self + -other
  }
}

impl <T: Float> Mul<T> for Vec3<T> {
  type Output = Vec3<T>;

  fn mul(self, other: T) -> Self::Output {
    Vec3::new(self.x * other, self.y * other, self.y * other)
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Vec4<T : Float> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T
}

impl <T: Float> Vec4<T> {
  fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4{x: x, y: y, z: z, w: w}
  }
}

impl <T: Float> Vector<T> for Vec4<T> {
  type Output = Vec4<T>;

  fn length(&self) -> T {
    let len_sqr = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    len_sqr.sqrt()
  }

  fn normalize(&mut self) -> () {
    let length = self.length();
    self.x = self.x / length;
    self.y = self.y / length;
    self.z = self.z / length;
    self.w = self.w / length;
  }

  fn normalized(&self) -> Self::Output {
    let mut ret = Vec4::new(self.x, self.y, self.z, self.w);
    ret.normalize();
    return ret;
  }

  fn dot(&self, v: &Self::Output) -> T {
    self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
  }
}

impl <T: Float> AddAssign for Vec4<T> {
  fn add_assign(&mut self, other: Vec4<T>) {
    self.x = self.x + other.x;
    self.y = self.y + other.y;
    self.z = self.z + other.z;
    self.w = self.w + other.w;
  }
}

impl <T : Float> Add for Vec4<T> {
  type Output = Vec4<T>;

  fn add(self, other: Vec4<T>) -> Self::Output {
    Vec4::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
  }
}

impl <T: Float> Neg for Vec4<T> {
  type Output = Vec4<T>;

  fn neg(self) -> Self::Output {
    Vec4::new(-self.x, -self.y, -self.z, -self.w)
  }
}

impl <T: Float> SubAssign for Vec4<T> {
  fn sub_assign(&mut self, other: Vec4<T>) {
    self.x = self.x - other.x;
    self.y = self.y - other.y;
    self.z = self.z - other.z;
    self.w = self.w - other.w;
  }
}

impl <T: Float> Sub for Vec4<T> {
  type Output = Vec4<T>;

  fn sub(self, other: Vec4<T>) -> Self::Output {
    self + -other
  }
}

impl <T: Float> Mul<T> for Vec4<T> {
  type Output = Vec4<T>;

  fn mul(self, other: T) -> Self::Output {
    Vec4::new(self.x * other, self.y * other, self.y * other, self.w * other)
  }
}
