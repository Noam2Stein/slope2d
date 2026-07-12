use ggmath::{Vec2A, Vec3A};
use slope1d::{Body, Interaction, World};
use testbed::{KeyCode, run};

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct BodyState {
    color: Color,
    is_active: bool,
}

fn main() {
    let mut world = World::new();
    let bodies = [
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Red,
            },
            center: -5.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Green,
            },
            center: -4.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Blue,
            },
            center: -3.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::White,
            },
            center: -2.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Black,
            },
            center: -1.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Red,
            },
            center: 0.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Green,
            },
            center: 1.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Blue,
            },
            center: 2.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::White,
            },
            center: 3.0,
            velocity: 0.0,
        },
        Body {
            metadata: BodyState {
                is_active: true,
                color: Color::Black,
            },
            center: 4.0,
            velocity: 0.0,
        },
    ]
    .map(|body| world.spawn(body));

    let mut controlled_body_index = 0;

    run(|ctx| {
        if ctx.key_pressed(KeyCode::ArrowUp) {
            controlled_body_index += 1;
            controlled_body_index %= bodies.len();
        }
        if ctx.key_pressed(KeyCode::ArrowDown) {
            if controlled_body_index == 0 {
                controlled_body_index = bodies.len() - 1;
            } else {
                controlled_body_index -= 1;
            }
        }

        let controlled_body = world.body_mut(bodies[controlled_body_index]).unwrap();
        controlled_body.velocity = 0.0;
        if ctx.key_held(KeyCode::ArrowRight) {
            controlled_body.velocity += 0.15;
        }
        if ctx.key_held(KeyCode::ArrowLeft) {
            controlled_body.velocity -= 0.15;
        }
        if ctx.key_pressed(KeyCode::Space) {
            controlled_body.metadata.is_active = !controlled_body.metadata.is_active;
        }

        world.update(|a, b| {
            if !a.metadata.is_active || !b.metadata.is_active {
                return Interaction::PassThrough;
            }

            match (a.metadata.color, b.metadata.color) {
                (Color::Red, Color::Red)
                | (Color::Green, Color::Green)
                | (Color::Blue, Color::Blue)
                | (Color::White | Color::Black, Color::White | Color::Black) => {
                    Interaction::Collide { pushback_on_b: 0.5 }
                }
                (Color::Red, Color::Green)
                | (Color::Green, Color::Blue)
                | (Color::Blue, Color::Red)
                | (Color::Black, Color::Red | Color::Green | Color::Blue) => {
                    Interaction::Collide { pushback_on_b: 0.9 }
                }
                (Color::Green, Color::Red)
                | (Color::Blue, Color::Green)
                | (Color::Red, Color::Blue)
                | (Color::Red | Color::Green | Color::Blue, Color::Black) => {
                    Interaction::Collide { pushback_on_b: 0.1 }
                }
                (Color::Red | Color::Green | Color::Blue, Color::White)
                | (Color::White, Color::Red | Color::Green | Color::Blue) => {
                    Interaction::PassThrough
                }
            }
        });

        for body_id in bodies.iter().copied() {
            let body = world.body(body_id).unwrap();

            ctx.draw_rectangle(
                match body.metadata.color {
                    Color::Red => Vec3A::X,
                    Color::Green => Vec3A::Y,
                    Color::Blue => Vec3A::Z,
                    Color::White => Vec3A::ONE,
                    Color::Black => Vec3A::ZERO,
                },
                Vec2A::splat(0.5),
                Vec2A::new(body.center, if body.metadata.is_active { 0.0 } else { 3.0 }),
                0.0,
            );
        }
    });
}
