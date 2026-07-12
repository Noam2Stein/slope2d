//! A simple physics engine for 2D platformer games.

#![no_std]

use ggmath::Vec2;

extern crate alloc;

#[derive(Debug)]
pub struct Body {
    pub id: u32,
    pub center: Vec2<f32>,
    pub velocity: Vec2<f32>,
}

pub fn physics_update(bodies: &mut [Body], mut a_push_b_factor: impl FnMut(&Body, &Body) -> f32) {
    for body in bodies.iter_mut() {
        body.center += body.velocity;
    }

    for _ in 0..1_000 {
        let n = bodies.len();
        for [i1, i2] in (0..n).flat_map(|i1| (0..n).map(move |i2| [i1, i2])) {
            let Ok([body_1, body_2]) = bodies.get_disjoint_mut([i1, i2]) else {
                continue;
            };

            let position_1_minus_2 = body_1.center - body_2.center;
            let position_abs_diff = position_1_minus_2.abs();
            let colliding = position_abs_diff.lt_mask(Vec2::ONE).all();

            if colliding {
                let pushback_axis = if position_abs_diff.x > position_abs_diff.y {
                    Vec2::X
                } else {
                    Vec2::Y
                };

                let pushback_factor_2 = a_push_b_factor(body_1, body_2);
                let pushback_factor_1 = 1.0 - pushback_factor_2;

                let full_pushback_2 =
                    position_1_minus_2.signum() * (position_abs_diff - Vec2::ONE) * pushback_axis;
                body_1.center -= full_pushback_2 * pushback_factor_1;
                body_2.center += full_pushback_2 * pushback_factor_2;
            }
        }
    }
}
