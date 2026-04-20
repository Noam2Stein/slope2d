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

    pub fn create<T>(&mut self, object: T) -> Id<T>
    where
        T: Object,
    {
        Id(T::storage_mut(self).insert(object), PhantomData)
    }

    pub fn remove<T>(&mut self, id: Id<T>)
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
            body.center += body.velocity;

            let body_min = body.center - body.extents;
            let body_max = body.center + body.extents;
            for (_, collider) in &self.colliders {
                let collider_min = collider.center - collider.extents;
                let collider_max = collider.center + collider.extents;

                let collision_right = (collider_min.x..collider_max.x).contains(&body_max.x);
                let collision_left = (collider_min.x..collider_max.x).contains(&body_min.x);
                let collision_up = (collider_min.y..collider_max.y).contains(&body_max.y);
                let collision_down = (collider_min.y..collider_max.y).contains(&body_min.y);

                if collision_right && !collision_left && (collision_up || collision_down) {
                    body.center.x = collider_min.x - body.extents.x;
                } else if collision_left && !collision_right && (collision_up || collision_down) {
                    body.center.x = collider_max.x + body.extents.x;
                } else if collision_up && !collision_down && (collision_right || collision_left) {
                    body.center.y = collider_min.y - body.extents.y;
                } else if collision_down && !collision_up && (collision_right || collision_left) {
                    body.center.y = collider_max.y + body.extents.y;
                }
            }
        }
    }
}
