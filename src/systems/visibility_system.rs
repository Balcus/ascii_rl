use std::todo;
use specs::{Join, System, WriteStorage, ReadStorage};
use crate::components::{FieldOfView, Position};

pub struct VisibilitySystem {}

impl <'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteStorage<'a, FieldOfView>, ReadStorage<'a, Position>);

    fn run(&mut self, (mut fov, pos): Self::SystemData) {
        for (fov, pos) in (&mut fov, &pos).join() {
            todo!()
        }
    }
}