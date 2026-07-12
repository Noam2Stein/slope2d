use ggmath::{Vec2, Vec2A, Vec3A};
use slope2d::{Body, physics_update};
use testbed::{KeyCode, run};

fn main() {
    let mut bodies = vec![
        Body {
            id: 0,
            center: Vec2::ZERO,
            velocity: Vec2::ZERO,
        },
        Body {
            id: 1,
            center: Vec2::new(3.0, 0.0),
            velocity: Vec2::ZERO,
        },
        Body {
            id: 2,
            center: Vec2::new(-3.0, 0.0),
            velocity: Vec2::ZERO,
        },
    ];

    run(|ctx| {
        bodies[0].velocity = Vec2::ZERO;
        if ctx.key_held(KeyCode::ArrowRight) {
            bodies[0].velocity.x += 0.12;
        }
        if ctx.key_held(KeyCode::ArrowLeft) {
            bodies[0].velocity.x -= 0.12;
        }

        physics_update(&mut bodies, |a, b| match (a.id, b.id) {
            (0, 1) => 0.9,
            (1, 0) => 0.1,
            (2, 1) => 1.0,
            (1, 2) => 0.0,
            _ => 0.5,
        });

        for body in &bodies {
            ctx.draw_rectangle(
                match body.id {
                    0 => Vec3A::ONE,
                    1 => Vec3A::X,
                    _ => Vec3A::Y,
                }
                .midpoint(body.center.align().extend(0.0) * 0.1),
                Vec2A::splat(0.5),
                body.center.align(),
                0.0,
            );
        }
    });
}
