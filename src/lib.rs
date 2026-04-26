use std::marker::PhantomData;

use ggmath::Vec2;
use thunderdome::Arena;

use crate::object::Object;

mod id;
mod object;
pub use id::*;

#[derive(Debug, Clone, Default)]
pub struct World {
    bodies: Arena<Body>,
    colliders: Arena<Collider>,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub extents: Vec2<f32>,
    pub center: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub extents: Vec2<f32>,
    pub center: Vec2<f32>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn<T>(&mut self, object: T) -> Id<T>
    where
        T: Object,
    {
        Id(T::storage_mut(self).insert(object), PhantomData)
    }

    pub fn despawn<T>(&mut self, id: Id<T>)
    where
        T: Object,
    {
        T::storage_mut(self).remove(id.0);
    }

    pub fn get<T>(&self, id: Id<T>) -> &T
    where
        T: Object,
    {
        &T::storage(self)[id.0]
    }

    pub fn get_mut<T>(&mut self, id: Id<T>) -> &mut T
    where
        T: Object,
    {
        &mut T::storage_mut(self)[id.0]
    }

    pub fn update(&mut self) {
        for (_, body) in &mut self.bodies {
            let body_min = body.center - body.extents;
            let body_max = body.center + body.extents;
            let mut body_delta = body.velocity;

            for (_, collider) in &self.colliders {
                let collider_min = collider.center - collider.extents;
                let collider_max = collider.center + collider.extents;

                let collision = collider_min.x - body_max.x;
                let collision_perp = body_delta.y * collision / body_delta.x;
                if collision > -0.001
                    && collision < body_delta.x
                    && ((collider_min.y..collider_max.y).contains(&(body_min.y + collision_perp))
                        || (collider_min.y..collider_max.y)
                            .contains(&(body_max.y + collision_perp)))
                {
                    body_delta.x = collision;
                }

                let collision = collider_min.y - body_max.y;
                let collision_perp = body_delta.x * collision / body_delta.y;
                if collision > -0.001
                    && collision < body_delta.y
                    && ((collider_min.x..collider_max.x).contains(&(body_min.x + collision_perp))
                        || (collider_min.x..collider_max.x)
                            .contains(&(body_max.x + collision_perp)))
                {
                    body_delta.y = collision;
                }

                let collision = collider_max.x - body_min.x;
                let collision_perp = body_delta.y * collision / body_delta.x;
                if collision < 0.001
                    && collision > body_delta.x
                    && ((collider_min.y..collider_max.y).contains(&(body_min.y + collision_perp))
                        || (collider_min.y..collider_max.y)
                            .contains(&(body_max.y + collision_perp)))
                {
                    body_delta.x = collision;
                }

                let collision = collider_max.y - body_min.y;
                let collision_perp = body_delta.x * collision / body_delta.y;
                if collision < 0.001
                    && collision > body_delta.y
                    && ((collider_min.x..collider_max.x).contains(&(body_min.x + collision_perp))
                        || (collider_min.x..collider_max.x)
                            .contains(&(body_max.x + collision_perp)))
                {
                    body_delta.y = collision;
                }
            }

            body.center += body_delta;
        }
    }
}
