use ggmath::{Vec2, Vec3};
use slope2d::{Body, Collider, Interact, Interaction, World};
use testbed::{KeyCode, run};

#[derive(Debug, Clone, Copy)]
enum Kind {
    Player,
    Wall,
}

impl Interact for Kind {
    fn interact(self, other: Self) -> Interaction {
        match (self, other) {
            (Self::Player, Self::Wall) => Interaction::Stop,
            _ => Interaction::Ignore,
        }
    }
}

fn main() {
    let mut world = World::new();

    let player_id = world.spawn(Body {
        colliders: vec![Collider::rect(Vec2::splat(0.5), Vec2::ZERO)],
        ..Body::with_kind(Kind::Player)
    });
    let collider_id = world.spawn(Body {
        colliders: vec![Collider::rect(Vec2::new(3.0, 2.0), Vec2::new(6.0, 1.0))],
        velocity: Vec2::X * 0.01,
        ..Body::with_kind(Kind::Wall)
    });
    let line_id = world.spawn(Body {
        colliders: vec![Collider::line(
            Vec2::new(0.0, -5.0),
            Vec2::new(10.0, 1.0),
            false,
        )],
        ..Body::with_kind(Kind::Wall)
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
        ) * 0.15;

        if ctx.key_pressed(KeyCode::Space) {
            player.velocity.y = 1.0;
        }

        world.update();

        let player = world.get(player_id);
        ctx.draw_rectangle(
            Vec3::X,
            player.colliders[0].as_rect().extents,
            player.position,
            0.0,
        );
        let collider = world.get(collider_id);
        ctx.draw_rectangle(
            Vec3::ZERO,
            collider.colliders[0].as_rect().extents,
            collider.colliders[0].as_rect().center + collider.position,
            0.0,
        );
        let line = world.get(line_id);
        ctx.draw_line(
            Vec3::splat(0.1),
            line.colliders[0].as_line().start + line.position,
            line.colliders[0].as_line().end + line.position,
        );
    });
}
