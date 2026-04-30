use core::{fmt::Debug, hash::Hash, marker::PhantomData};

use thunderdome::{Arena, Index};

use crate::{Body, Configuration};

pub struct World<Cfg>
where
    Cfg: Configuration,
{
    bodies: Arena<Body<Cfg>>,
}

pub struct BodyId<Cfg>(Index, PhantomData<Cfg>)
where
    Cfg: Configuration;

impl<Cfg> World<Cfg>
where
    Cfg: Configuration,
{
    pub fn new() -> Self {
        Self {
            bodies: Arena::new(),
        }
    }

    pub fn spawn(&mut self, body: Body<Cfg>) -> BodyId<Cfg> {
        BodyId(self.bodies.insert(body), PhantomData)
    }

    pub fn get(&self, body: BodyId<Cfg>) -> &Body<Cfg> {
        self.bodies.get(body.0).expect("body does not exist")
    }

    pub fn get_mut(&mut self, body: BodyId<Cfg>) -> &mut Body<Cfg> {
        self.bodies.get_mut(body.0).expect("body does not exist")
    }

    pub fn try_get(&self, body: BodyId<Cfg>) -> Option<&Body<Cfg>> {
        self.bodies.get(body.0)
    }

    pub fn try_get_mut(&mut self, body: BodyId<Cfg>) -> Option<&mut Body<Cfg>> {
        self.bodies.get_mut(body.0)
    }

    pub fn despawn(&mut self, body: BodyId<Cfg>) -> Body<Cfg> {
        self.bodies.remove(body.0).expect("body does not exist")
    }

    pub fn try_despawn(&mut self, body: BodyId<Cfg>) -> Option<Body<Cfg>> {
        self.bodies.remove(body.0)
    }

    pub fn update(&mut self) {
        for (_, body) in &mut self.bodies {
            body.position += body.velocity;
        }
    }
}

impl<Cfg> Clone for World<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        Self {
            bodies: self.bodies.clone(),
        }
    }
}

impl<Cfg> Default for World<Cfg>
where
    Cfg: Configuration,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Cfg> Debug for BodyId<Cfg>
where
    Cfg: Configuration,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BodyId").field(&self.0).finish()
    }
}

impl<Cfg> Clone for BodyId<Cfg>
where
    Cfg: Configuration,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<Cfg> Copy for BodyId<Cfg> where Cfg: Configuration {}

impl<Cfg> PartialEq for BodyId<Cfg>
where
    Cfg: Configuration,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<Cfg> Eq for BodyId<Cfg> where Cfg: Configuration {}

impl<Cfg> Hash for BodyId<Cfg>
where
    Cfg: Configuration,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
