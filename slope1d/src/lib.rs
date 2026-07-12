//! An extremely simple 1-dimensional version of the engine. This is a lot
//! easier to build than the full 2D engine, but it still helps me find
//! solutions to particular problems that also arise in the 2D version.

use std::range::RangeInclusive;

use thunderdome::{Arena, Index};

#[derive(Debug, Clone)]
pub struct World<M> {
    bodies: Arena<Body<M>>,
}

/// A 1D AABB body, with a length assumed to be 1.
#[derive(Debug, Clone)]
pub struct Body<M> {
    pub metadata: M,
    pub center: f32,
    /// The change in position per frame.
    ///
    /// The engine never changes the velocity itself.
    pub velocity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BodyId(Index);

#[derive(Debug, Clone, PartialEq)]
pub enum Interaction {
    /// Push the two bodies away from each other to avoid overlap.
    ///
    /// `pushback_on_b` must be between 0 and 1, and decides how much of the
    /// pushback `b` gets, while `a` gets `1 - pushback_on_b` of the pushback.
    Collide { pushback_on_b: f32 },
    /// The two bodies pass through each other.
    PassThrough,
}

impl<M> Default for World<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M> World<M> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bodies: Arena::new(),
        }
    }

    pub fn spawn(&mut self, body: Body<M>) -> BodyId {
        BodyId(self.bodies.insert(body))
    }

    pub fn despawn(&mut self, body_id: BodyId) -> Option<Body<M>> {
        self.bodies.remove(body_id.0)
    }

    #[must_use]
    pub fn body(&self, body_id: BodyId) -> Option<&Body<M>> {
        self.bodies.get(body_id.0)
    }

    #[must_use]
    pub fn body_mut(&mut self, body_id: BodyId) -> Option<&mut Body<M>> {
        self.bodies.get_mut(body_id.0)
    }

    pub fn update(&mut self, dictate_interaction: impl Fn(&Body<M>, &Body<M>) -> Interaction) {
        let mut body_indices = self.bodies.iter().map(|(i, _)| i).collect::<Vec<Index>>();
        body_indices
            .sort_unstable_by(|&a, &b| self.bodies[a].center.total_cmp(&self.bodies[b].center));

        for (_, body) in &mut self.bodies {
            body.center += body.velocity;
        }

        let mut interactions = vec![Interaction::PassThrough; body_indices.len() - 1];
        for _ in 0..10 {
            for (i, interaction) in interactions.iter_mut().enumerate() {
                if *interaction != Interaction::PassThrough {
                    continue;
                }

                let body_a = &self.bodies[body_indices[i]];
                let body_b = &self.bodies[body_indices[i + 1]];

                *interaction = if body_b.center - body_a.center <= 1.0 {
                    dictate_interaction(body_a, body_b)
                } else {
                    Interaction::PassThrough
                };
            }

            let mut collision_chains = Vec::<RangeInclusive<usize>>::new();
            for (i, interaction) in interactions.iter().enumerate() {
                match interaction {
                    Interaction::Collide { .. } => {
                        if let Some(last_chain) = collision_chains.last_mut()
                            && last_chain.last == i - 1
                        {
                            last_chain.last += 1;
                        } else {
                            collision_chains.push((i..=i).into());
                        }
                    }
                    Interaction::PassThrough => {}
                }
            }

            dbg!(&collision_chains);

            for collision_chain in collision_chains {
                self.resolve_collision_chain(
                    &body_indices[collision_chain.start..=collision_chain.last + 1],
                    &interactions[collision_chain],
                );
            }
        }
    }

    fn resolve_collision_chain(&mut self, body_indices: &[Index], interactions: &[Interaction]) {
        let mut weight = 1.0;
        let mut sum = 0.0;
        let mut weight_sum = 0.0;

        let mut i = 0;
        loop {
            sum += weight * (self.bodies[body_indices[i]].center - i as f32);
            weight_sum += weight;

            if i >= interactions.len() {
                break;
            }

            let pushback_on_a = match interactions[i] {
                Interaction::Collide { pushback_on_b } => pushback_on_b,
                Interaction::PassThrough => {
                    unreachable!("a single chain should only contain collisions")
                }
            };

            weight *= (1.0 - pushback_on_a) / pushback_on_a;

            i += 1;
        }

        let first_body_result = sum / weight_sum;
        for (i, body_index) in body_indices.iter().enumerate() {
            self.bodies[*body_index].center = first_body_result + i as f32;
        }
    }
}
