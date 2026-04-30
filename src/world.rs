use thunderdome::{Arena, Index};

use crate::{Body, Interact, Num};

#[derive(Debug, Clone)]
pub struct World<T, K>
where
    T: Num,
{
    bodies: Arena<Body<T, K>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(Index);

impl<T, K> World<T, K>
where
    T: Num,
{
    pub fn new() -> Self {
        Self {
            bodies: Arena::new(),
        }
    }

    pub fn spawn(&mut self, body: Body<T, K>) -> BodyId {
        BodyId(self.bodies.insert(body))
    }

    pub fn get(&self, body: BodyId) -> &Body<T, K> {
        self.bodies.get(body.0).expect("body does not exist")
    }

    pub fn get_mut(&mut self, body: BodyId) -> &mut Body<T, K> {
        self.bodies.get_mut(body.0).expect("body does not exist")
    }

    pub fn try_get(&self, body: BodyId) -> Option<&Body<T, K>> {
        self.bodies.get(body.0)
    }

    pub fn try_get_mut(&mut self, body: BodyId) -> Option<&mut Body<T, K>> {
        self.bodies.get_mut(body.0)
    }

    pub fn despawn(&mut self, body: BodyId) -> Body<T, K> {
        self.bodies.remove(body.0).expect("body does not exist")
    }

    pub fn try_despawn(&mut self, body: BodyId) -> Option<Body<T, K>> {
        self.bodies.remove(body.0)
    }

    pub fn update(&mut self)
    where
        K: Interact,
    {
        for (_, body) in &mut self.bodies {
            body.position += body.velocity;
        }
    }
}

impl<T, Tag> Default for World<T, Tag>
where
    T: Num,
{
    fn default() -> Self {
        Self::new()
    }
}
