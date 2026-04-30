use core::fmt::Debug;

use crate::Num;

pub trait Configuration {
    type Num: Num;
    type Tag: Debug + Copy;

    fn reaction(body: Self::Tag, other: Self::Tag) -> Reaction;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reaction {
    Ignore,
    Stop,
}
