//! This file shows the rules the engine should follow, at least for the next
//! early version which only works with AABBs. The idea is that when two bodies
//! collide, you can control how much of the pushback each body gets, dependning
//! on whatever logic you want.
//!
//! This is normally done using mass constants, but I think this approach will
//! work better because it lets you control exactly how interactions look,
//! without needing to tune mass variables that all affect each other.
//!
//! The goal after this is to use this algorithm for a 2D engine with AABBs,
//! though that introduces many more edge cases.
//!
//! The initial idea for resolving collisions is shown in [`slow`] and [`fast`]
//! is a non-iterative algorithm that achives the same result.

use std::range::RangeInclusive;

use ggmath::{Vec2A, Vec3A};
use testbed::{KeyCode, run};

fn main() {
    let bodies = vec![-2.0, -1.0, -0.5, 1.7, 2.0, 2.5];
    let pushback_factors = vec![0.5, 0.5, 0.1, 0.5, 0.5];

    let mut bodies_slow = bodies.clone();
    approximate_solution(&mut bodies_slow, &pushback_factors);
    let mut bodies_fast = bodies.clone();
    directly_compute_solution(&mut bodies_fast, &pushback_factors);

    if (0..bodies.len()).all(|i| (bodies_slow[i] - bodies_fast[i]).abs() < 1e-3) {
        dbg!(bodies_slow);
        dbg!(bodies_fast);
    } else {
        dbg!(bodies_slow);
        dbg!(bodies_fast);
        println!();
        println!("not equal!");
    }

    let mut bodies = bodies;
    let mut idx = 0;

    run(|ctx| {
        if ctx.key_held(KeyCode::ArrowRight) {
            bodies[idx] += 0.15;
        }
        if ctx.key_held(KeyCode::ArrowLeft) {
            bodies[idx] -= 0.15;
        }
        if ctx.key_pressed(KeyCode::ArrowUp) {
            idx += 1;
        }
        if ctx.key_pressed(KeyCode::ArrowDown) {
            idx -= 1;
        }

        directly_compute_solution(&mut bodies, &pushback_factors);

        for (i, body) in bodies.iter().enumerate() {
            ctx.draw_rectangle(
                Vec3A::new(i as f32 * 0.1, 0.5, 0.5),
                Vec2A::splat(0.5),
                Vec2A::new(*body as f32, 0.0),
                0.0,
            );
        }
    });
}

fn approximate_solution(bodies: &mut [f64], pushback_factors: &[f64]) {
    for _ in 0..100_000 {
        for (i, pushback_factor) in pushback_factors.iter().copied().enumerate() {
            let [body_1, body_2] = bodies.get_disjoint_mut([i, i + 1]).unwrap();

            let overlap = 1.0 - (*body_2 - *body_1);
            if overlap > 0.0 {
                *body_1 -= overlap * (1.0 - pushback_factor);
                *body_2 += overlap * pushback_factor;
            }
        }
    }
}

fn directly_compute_solution(bodies: &mut [f64], pushback_factors: &[f64]) {
    for _ in 0..5 {
        let mut ranges = Vec::<RangeInclusive<usize>>::new();
        while ranges
            .last()
            .is_none_or(|&last| last.last < bodies.len() - 1)
        {
            let start = ranges.last().copied().map_or(0, |range| range.last + 1);
            let last = (start + 1..bodies.len())
                .take_while(|&last| bodies[last] - bodies[last - 1] <= 1.0)
                .last()
                .unwrap_or(start);

            ranges.push((start..=last).into());
        }

        for range in ranges {
            compute_single_chain(
                &mut bodies[range],
                &pushback_factors[range.start..range.last],
            );
        }
    }
}

fn compute_single_chain(bodies: &mut [f64], pushback_factors: &[f64]) {
    let mut weight = 1.0;
    let mut sum = 0.0;
    let mut weight_sum = 0.0;

    let mut i = 0;
    loop {
        sum += weight * (bodies[i] - i as f64);
        weight_sum += weight;

        if i >= pushback_factors.len() {
            break;
        }

        let pushback_factor = pushback_factors[i];
        weight *= (1.0 - pushback_factor) / pushback_factor;

        i += 1;
    }

    let first_body_result = sum / weight_sum;
    for (i, body) in bodies.iter_mut().enumerate() {
        *body = first_body_result + i as f64;
    }
}
