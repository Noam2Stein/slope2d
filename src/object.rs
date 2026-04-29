use thunderdome::Arena;

use crate::{AabbCollider, Body, LineCollider, World};

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

impl Object for AabbCollider {
    fn storage(world: &World) -> &Arena<Self> {
        &world.aabb_colliders
    }

    fn storage_mut(world: &mut World) -> &mut Arena<Self> {
        &mut world.aabb_colliders
    }
}

impl Object for LineCollider {
    fn storage(world: &World) -> &Arena<Self> {
        &world.line_colliders
    }

    fn storage_mut(world: &mut World) -> &mut Arena<Self> {
        &mut world.line_colliders
    }
}
