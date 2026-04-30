use alloc::vec::Vec;
use ggmath::Vec2;

use crate::{Collider, Configuration};

#[non_exhaustive]
pub struct Body<Cfg>
where
    Cfg: Configuration,
{
    pub tag: Cfg::Tag,
    pub colliders: Vec<Collider<Cfg>>,
    pub position: Vec2<Cfg::Num>,
    pub rotation: Vec2<Cfg::Num>,
    pub velocity: Vec2<Cfg::Num>,
}

impl<Cfg> Body<Cfg>
where
    Cfg: Configuration,
{
    pub fn new(
        tag: Cfg::Tag,
        colliders: impl IntoIterator<Item = Collider<Cfg>>,
        position: Vec2<Cfg::Num>,
        rotation: Vec2<Cfg::Num>,
        velocity: Vec2<Cfg::Num>,
    ) -> Self {
        Self {
            tag,
            colliders: Vec::from_iter(colliders),
            position,
            rotation,
            velocity,
        }
    }
}

impl<Cfg> Clone for Body<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        Self {
            tag: self.tag,
            colliders: self.colliders.clone(),
            position: self.position,
            rotation: self.rotation,
            velocity: self.velocity,
        }
    }
}
