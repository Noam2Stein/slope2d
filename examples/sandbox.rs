use ggmath::{Vec2, Vec3};
use slope2d::{Body, Collider, Configuration, Reaction, World};
use testbed::{KeyCode, run};

#[derive(Debug, Clone, Copy)]
enum Tag {
    Player,
    Wall,
}

struct Cfg;

impl Configuration for Cfg {
    type Num = f32;
    type Tag = Tag;

    fn reaction(a: Self::Tag, b: Self::Tag) -> Reaction {
        match (a, b) {
            (Tag::Player, Tag::Wall) => Reaction::Stop,
            _ => Reaction::Ignore,
        }
    }
}

fn main() {
    let mut world = World::<Cfg>::new();

    let player_id = world.spawn(Body::new(
        Tag::Player,
        [Collider::rect(Vec2::splat(0.5), Vec2::ZERO)],
        Vec2::ZERO,
        Vec2::X,
        Vec2::ZERO,
    ));
    let collider_id = world.spawn(Body::new(
        Tag::Wall,
        [Collider::rect(Vec2::new(3.0, 2.0), Vec2::new(6.0, 1.0))],
        Vec2::ZERO,
        Vec2::X,
        Vec2::ZERO,
    ));
    let line_id = world.spawn(Body::new(
        Tag::Wall,
        [Collider::line(
            Vec2::new(0.0, -5.0),
            Vec2::new(10.0, 1.0),
            false,
        )],
        Vec2::ZERO,
        Vec2::X,
        Vec2::ZERO,
    ));

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
            collider.colliders[0].as_rect().center,
            0.0,
        );
        let line = world.get(line_id);
        ctx.draw_line(
            Vec3::splat(0.1),
            line.colliders[0].as_line().start,
            line.colliders[0].as_line().end,
        );
    });
}
