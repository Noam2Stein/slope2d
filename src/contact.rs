use ggmath::Vec2;

use crate::Num;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Contact<T>
where
    T: Num,
{
    pub point: Vec2<T>,
    pub normal: Vec2<T>,
}
