use alloc::vec::Vec;
use ggmath::Vec2;

use crate::{Collider, Contact, Num};

#[derive(Debug, Clone, Default)]
pub struct Body<T, K>
where
    T: Num,
{
    pub kind: K,
    pub colliders: Vec<Collider<T>>,
    pub position: Vec2<T>,
    pub rotation: Vec2<T>,
    pub velocity: Vec2<T>,
    pub contacts: Vec<Contact<T>>,
}

impl<T, K> Body<T, K>
where
    T: Num,
{
    pub fn with_kind(kind: K) -> Self {
        Self {
            kind,
            colliders: Vec::new(),
            position: Vec2::ZERO,
            rotation: Vec2::X,
            velocity: Vec2::ZERO,
            contacts: Vec::new(),
        }
    }
}
