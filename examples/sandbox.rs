use ggmath::{Vec2A, Vec3A};
use testbed::{KeyCode, run};

fn main() {
    let mut pos = Vec2A::ZERO;

    run(|ctx| {
        if ctx.key_held(KeyCode::ArrowRight) {
            pos.x += 0.12;
        }
        if ctx.key_held(KeyCode::ArrowLeft) {
            pos.x -= 0.12;
        }
        if ctx.key_held(KeyCode::ArrowUp) {
            pos.y += 0.12;
        }
        if ctx.key_held(KeyCode::ArrowDown) {
            pos.y -= 0.12;
        }

        ctx.draw_rectangle(Vec3A::ONE, Vec2A::splat(0.5), pos, 0.0);
    });
}
