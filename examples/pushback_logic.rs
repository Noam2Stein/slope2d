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

fn main() {
    let bodies = vec![1.0, 1.4, 2.0, 2.3, 5.4, 6.4];
    let pushback_factors = vec![0.3, 0.7, 0.9, 0.1, 0.5];

    let mut bodies_slow = bodies.clone();
    slow(&mut bodies_slow, &pushback_factors);
    let mut bodies_fast = bodies.clone();
    fast(&mut bodies_fast, &pushback_factors);

    if (0..bodies.len()).all(|i| (bodies_slow[i] - bodies_fast[i]).abs() < 1e-3) {
        dbg!(bodies_slow);
        dbg!(bodies_fast);
    } else {
        dbg!(bodies_slow);
        dbg!(bodies_fast);
        println!();
        println!("not equal!");
    }
}

fn slow(bodies: &mut [f64], pushback_factors: &[f64]) {
    for _ in 0..100_000 {
        for (i, pushback_factor) in pushback_factors.iter().copied().enumerate() {
            let [body_1, body_2] = bodies.get_disjoint_mut([i, i + 1]).unwrap();

            let intersection = 1.0 - (*body_2 - *body_1);
            *body_1 -= intersection * (1.0 - pushback_factor);
            *body_2 += intersection * pushback_factor;
        }
    }
}

fn fast(bodies: &mut [f64], pushback_factors: &[f64]) {
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
