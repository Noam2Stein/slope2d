use crate::Interaction;

pub trait Interact {
    fn interact(self, other: Self) -> Interaction;
}
