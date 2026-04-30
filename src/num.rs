use core::{
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use ggmath::{
    Scalar,
    constants::{NegOne, One, Zero},
};

pub trait Num:
    Debug
    + PartialEq
    + PartialOrd
    + Default
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Scalar
    + Zero
    + One
    + NegOne
{
}

impl Num for f32 {}
