use core::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use ggmath::Scalar;

pub trait Num:
    Scalar
    + Debug
    + PartialEq
    + PartialOrd
    + Default
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
}

impl Num for f32 {}
