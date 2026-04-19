use ggmath::Vec2;
use thunderdome::{Arena, Index};

#[derive(Debug, Clone, Default)]
pub struct World {
    bodies: Arena<Body>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(Index);

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_body(&mut self, body: Body) -> BodyId {
        BodyId(self.bodies.insert(body))
    }

    pub fn body(&self, id: BodyId) -> &Body {
        &self.bodies[id.0]
    }

    pub fn body_mut(&mut self, id: BodyId) -> &mut Body {
        &mut self.bodies[id.0]
    }

    pub fn remove_body(&mut self, id: BodyId) {
        self.bodies.remove(id.0);
    }

    pub fn update(&mut self) {
        for (_, body) in &mut self.bodies {
            body.position += body.velocity;
        }
    }
}
