use core::num::{Zero, One, Algebraic, abs};
use core::rand::{Rand, Rng, RngUtil};
use core::cmp::ApproxEq;
use traits::basis::Basis;
use traits::cross::Cross;
use traits::dim::Dim;
use traits::dot::Dot;
use traits::norm::Norm;
use traits::translation::Translation;
use traits::workarounds::scalar_op::{ScalarMul, ScalarDiv, ScalarAdd, ScalarSub};

#[deriving(Eq)]
pub struct Vec3<T>
{
  x : T,
  y : T,
  z : T
}

pub fn vec3<T:Copy>(x: T, y: T, z: T) -> Vec3<T>
{ Vec3 {x: x, y: y, z: z} }

impl<T> Dim for Vec3<T>
{
  fn dim() -> uint
  { 3 }
}

impl<T:Copy + Add<T,T>> Add<Vec3<T>, Vec3<T>> for Vec3<T>
{
  fn add(&self, other: &Vec3<T>) -> Vec3<T>
  { vec3(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl<T:Copy + Sub<T,T>> Sub<Vec3<T>, Vec3<T>> for Vec3<T>
{
  fn sub(&self, other: &Vec3<T>) -> Vec3<T>
  { vec3(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl<T: Copy + Mul<T, T>>
ScalarMul<T> for Vec3<T>
{
  fn scalar_mul(&self, s: &T) -> Vec3<T>
  { Vec3 { x: self.x * *s, y: self.y * *s, z: self.z * *s } }

  fn scalar_mul_inplace(&mut self, s: &T)
  {
    self.x *= *s;
    self.y *= *s;
    self.z *= *s;
  }
}


impl<T: Copy + Div<T, T>>
ScalarDiv<T> for Vec3<T>
{
  fn scalar_div(&self, s: &T) -> Vec3<T>
  { Vec3 { x: self.x / *s, y: self.y / *s, z: self.z / *s } }

  fn scalar_div_inplace(&mut self, s: &T)
  {
    self.x /= *s;
    self.y /= *s;
    self.z /= *s;
  }
}

impl<T: Copy + Add<T, T>>
ScalarAdd<T> for Vec3<T>
{
  fn scalar_add(&self, s: &T) -> Vec3<T>
  { Vec3 { x: self.x + *s, y: self.y + *s, z: self.z + *s } }

  fn scalar_add_inplace(&mut self, s: &T)
  {
    self.x += *s;
    self.y += *s;
    self.z += *s;
  }
}

impl<T: Copy + Sub<T, T>>
ScalarSub<T> for Vec3<T>
{
  fn scalar_sub(&self, s: &T) -> Vec3<T>
  { Vec3 { x: self.x - *s, y: self.y - *s, z: self.z - *s } }

  fn scalar_sub_inplace(&mut self, s: &T)
  {
    self.x -= *s;
    self.y -= *s;
    self.z -= *s;
  }
}

impl<T: Copy + Add<T, T>> Translation<Vec3<T>> for Vec3<T>
{
  fn translation(&self) -> Vec3<T>
  { *self }

  fn translated(&self, t: &Vec3<T>) -> Vec3<T>
  { self + *t }

  fn translate(&mut self, t: &Vec3<T>)
  { *self += *t; }
}



impl<T:Copy + Neg<T>> Neg<Vec3<T>> for Vec3<T>
{
  fn neg(&self) -> Vec3<T>
  { vec3(-self.x, -self.y, -self.z) }
}

impl<T:Copy + Mul<T, T> + Add<T, T> + Algebraic> Dot<T> for Vec3<T>
{
  fn dot(&self, other : &Vec3<T>) -> T
  { self.x * other.x + self.y * other.y + self.z * other.z } 
}

impl<T:Copy + Mul<T, T> + Add<T, T> + Div<T, T> + Algebraic>
Norm<T> for Vec3<T>
{
  fn sqnorm(&self) -> T
  { self.dot(self) }

  fn norm(&self) -> T
  { self.sqnorm().sqrt() }

  fn normalized(&self) -> Vec3<T>
  {
    let l = self.norm();

    vec3(self.x / l, self.y / l, self.z / l)
  }

  fn normalize(&mut self) -> T
  {
    let l = self.norm();

    self.x /= l;
    self.y /= l;
    self.z /= l;

    l
  }
}

impl<T:Copy + Mul<T, T> + Sub<T, T>> Cross<Vec3<T>> for Vec3<T>
{
  fn cross(&self, other : &Vec3<T>) -> Vec3<T>
  {
    vec3(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x
    )
  }
}

impl<T:Copy + Zero> Zero for Vec3<T>
{
  fn zero() -> Vec3<T>
  {
    let _0 = Zero::zero();
    vec3(_0, _0, _0)
  }

  fn is_zero(&self) -> bool
  { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() }
}

impl<T: Copy + One + Zero + Neg<T> + Ord + Mul<T, T> + Sub<T, T> + Add<T, T> +
        Div<T, T> + Algebraic>
Basis for Vec3<T>
{
  fn canonical_basis() -> ~[Vec3<T>]
  {
    // FIXME: this should be static
    ~[ vec3(One::one(), Zero::zero(), Zero::zero()),
       vec3(Zero::zero(), One::one(), Zero::zero()),
       vec3(Zero::zero(), Zero::zero(), One::one()) ]
  }

  fn orthogonal_subspace_basis(&self) -> ~[Vec3<T>]
  {
      let a = 
        if (abs(self.x) > abs(self.y))
        { vec3(self.z, Zero::zero(), -self.x).normalized() }
        else
        { vec3(Zero::zero(), -self.z, self.y).normalized() };

      ~[ a, a.cross(self) ]
  }
}

impl<T:ApproxEq<T>> ApproxEq<T> for Vec3<T>
{
  fn approx_epsilon() -> T
  { ApproxEq::approx_epsilon::<T, T>() }

  fn approx_eq(&self, other: &Vec3<T>) -> bool
  {
    self.x.approx_eq(&other.x) &&
    self.y.approx_eq(&other.y) &&
    self.z.approx_eq(&other.z)
  }

  fn approx_eq_eps(&self, other: &Vec3<T>, epsilon: &T) -> bool
  {
    self.x.approx_eq_eps(&other.x, epsilon) &&
    self.y.approx_eq_eps(&other.y, epsilon) &&
    self.z.approx_eq_eps(&other.z, epsilon)
  }
}

impl<T:Copy + Rand> Rand for Vec3<T>
{
  fn rand<R: Rng>(rng: &mut R) -> Vec3<T>
  { vec3(rng.gen(), rng.gen(), rng.gen()) }
}

impl<T:ToStr> ToStr for Vec3<T>
{
  fn to_str(&self) -> ~str
  {
    ~"Vec3 "
    + "{ x : " + self.x.to_str()
    + ", y : " + self.y.to_str()
    + ", z : " + self.z.to_str()
    + " }"
  }
}