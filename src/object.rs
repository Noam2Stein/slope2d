use thunderdome::Arena;

use crate::{Body, Collider, World};

pub trait Object: Sized {
    fn storage(world: &World) -> &Arena<Self>;

    fn storage_mut(world: &mut World) -> &mut Arena<Self>;
}

impl Object for Body {
    fn storage(world: &World) -> &Arena<Self> {
        &world.bodies
    }

    fn storage_mut(world: &mut World) -> &mut Arena<Self> {
        &mut world.bodies
    }
}

impl Object for Collider {
    fn storage(world: &World) -> &Arena<Self> {
        &world.colliders
    }

    fn storage_mut(world: &mut World) -> &mut Arena<Self> {
        &mut world.colliders
    }
}
