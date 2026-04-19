use ggmath::{Vec2, Vec4};
use slope2d::{Body, World};
use testbed::{KeyCode, run};

fn main() {
    let mut world = World::new();

    let player_id = world.create_body(Body {
        position: Vec2::ZERO,
        velocity: Vec2::ZERO,
    });

    run(|ctx| {
        let player = world.body_mut(player_id);
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

        let player = world.body(player_id);
        ctx.draw_rectangle(
            Vec4::new(1.0, 0.0, 0.0, 1.0),
            Vec2::ONE,
            player.position,
            0.0,
        );
    });
}
