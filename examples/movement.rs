use ggmath::{Vec2, Vec4};
use slope2d::{Body, Collider, World};
use testbed::{KeyCode, run};

fn main() {
    let mut world = World::new();

    let player_id = world.create(Body {
        extents: Vec2::splat(0.5),
        center: Vec2::ZERO,
        velocity: Vec2::ZERO,
    });
    let collider_id = world.create(Collider {
        extents: Vec2::new(3.0, 2.0),
        center: Vec2::new(6.0, 1.0),
    });

    run(|ctx| {
        let player = world.get_mut(player_id);
        player.velocity = Vec2::new(
            if ctx.key_held(KeyCode::ArrowRight) {
                1.0
            } else if ctx.key_held(KeyCode::ArrowLeft) {
                -1.0
            } else {
                0.0
            },
            if ctx.key_held(KeyCode::ArrowUp) {
                1.0
            } else if ctx.key_held(KeyCode::ArrowDown) {
                -1.0
            } else {
                0.0
            },
        ) * 0.2;

        if ctx.key_pressed(KeyCode::Space) {
            player.velocity.y = 1.0;
        }

        world.update();

        let player = world.get(player_id);
        ctx.draw_rectangle(
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            player.extents * 2.0,
            player.center,
            0.0,
        );
        let collider = world.get(collider_id);
        ctx.draw_rectangle(Vec4::W, collider.extents * 2.0, collider.center, 0.0);
    });
}
