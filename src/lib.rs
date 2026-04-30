//! A simple physics engine for 2D platformer games.

#![no_std]

extern crate alloc;

pub use crate::body::*;
pub use crate::collider::*;
pub use crate::contact::*;
pub use crate::interaction::*;
pub use crate::kind::*;
pub use crate::num::*;
pub use crate::world::*;

mod body;
mod collider;
mod contact;
mod interaction;
mod kind;
mod num;
mod world;
