use core::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign, SubAssign, Neg};
use core::fmt::{Display, Formatter, Result, UpperHex};
use core::cmp::{min, max};
use unroll::unroll_for_loops;
#[derive(Clone, Copy)]
pub struct Vec3(pub [f32; 3]);
pub type Point = Vec3;
pub type Color = Vec3;
impl Vec3 {
    #[inline]
    pub fn new(x : f32, y : f32, z : f32) -> Self {
        Self([x, y, z])
    }
    #[inline]
    #[unroll_for_loops]
    pub fn dot(&self, other : &Self) -> f32 {
        let mut result = 0.0;
        for i in 0..3 {
            result += self.0[i] * other.0[i];
        }
        result
    }
    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }
    pub fn normalize(&self) -> Self {
        let inv_len = self.len().recip();
        self * inv_len
    }
    #[inline]
    pub fn cross(&self, other : &Self) -> Self {
        Self (
            [self.0[1] * other.0[2] - self.0[2] * other.0[1],
             self.0[2] * other.0[0] - self.0[0] * other.0[2],
             self.0[0] * other.0[1] - self.0[1] * other.0[0]
            ]
        )
    }
    #[inline]
    pub fn x(&self) -> f32 {
        self.0[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.0[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.0[2]
    }
    pub fn clamp(&self, lower : f32, upper : f32) -> Self {
        let mut result = [0.0; 3];
        for i in 0..3 {
            result[i] = lower.max(self.0[i].min(upper));
        }
        Self(result)
    }
    pub fn to_rgb(&self) -> (u8, u8, u8){ 
        let scaled = self.clamp(0.0, 1.0) * 255.0;
        let r : u8 = unsafe { scaled.x().round().to_int_unchecked()};
        let g : u8 = unsafe { scaled.y().round().to_int_unchecked()};
        let b : u8 = unsafe { scaled.z().round().to_int_unchecked()};
        return (r, g, b);
    }
}
//print as color
impl UpperHex for Vec3 {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        let rgb = self.to_rgb();
        write!(f, "0x{:X}{:X}{:X}", rgb.0, rgb.1, rgb.2)
    }
}
//print as vector
impl Display for Vec3 {
    fn fmt(&self, f : &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.0[0], self.0[1], self.0[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    #[unroll_for_loops]
    fn neg(self) -> Self {
        let mut result = [0.0; 3];
        for i in 0..3 {
            result[i] = self.0[i];
        }
        Vec3(result)
    }  
}

macro_rules! impl_bin_op_vector {
    ($t:ident, $f:ident, $o:tt) => {
        impl$t<Vec3> for Vec3 {
            type Output = Vec3;
            #[inline]
            #[unroll_for_loops]
            fn $f(self, other : Self) -> Self::Output {
                let mut result = [0.0; 3];
                for i in 0..3 {
                    result[i] = self.0[i] $o other.0[i];
                }
                return Vec3(result)
            }
        }
        impl<'a> $t<&'a Vec3> for &'a Vec3 {
            type Output = Vec3;
            #[inline]
            #[unroll_for_loops]
            fn $f(self, other : Self) -> Self::Output {
                let mut result = [0.0; 3];
                for i in 0..3 {
                    result[i] = self.0[i] $o other.0[i];
                }
                return Vec3(result)
            }
        }
        impl$t<f32> for Vec3 {
            type Output = Vec3;
            #[inline]
            #[unroll_for_loops]
            fn $f(self, other : f32) -> Self::Output {
                let mut result = [0.0; 3];
                for i in 0..3 {
                    result[i] = self.0[i] $o other;
                }
                return Vec3(result)
            }
        }
        impl<'a> $t<f32> for &'a Vec3 {
            type Output = Vec3;
            #[inline]
            #[unroll_for_loops]
            fn $f(self, other : f32) -> Self::Output {
                let mut result = [0.0; 3];
                for i in 0..3 {
                    result[i] = self.0[i] $o other;
                }
                return Vec3(result)
            }
        }
    }
}

macro_rules! impl_bin_op_assign_vector {
    ($t:ident, $f:ident, $o:tt) => {
        impl$t<Vec3> for Vec3 { 
            #[inline]
            #[unroll_for_loops]
            fn $f(&mut self, other : Self) {
                for i in 0..3 {
                   self.0[i] $o other.0[i];
                }
            }
        }
        impl $t<f32> for Vec3 {
            #[inline]
            #[unroll_for_loops]
            fn $f(&mut self, other : f32) {
                for i in 0..3 {
                    self.0[i] $o other;
                }
            }
        }
    }
}

impl_bin_op_vector!{Add, add, +}
impl_bin_op_vector!{Sub, sub, -}
impl_bin_op_vector!{Mul, mul, *}
impl_bin_op_vector!{Div, div, /}
impl_bin_op_assign_vector!{AddAssign, add_assign, +=}
impl_bin_op_assign_vector!{SubAssign, sub_assign, -=}
impl_bin_op_assign_vector!{MulAssign, mul_assign, *=}
impl_bin_op_assign_vector!{DivAssign, div_assign, /=}
