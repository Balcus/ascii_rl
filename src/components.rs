use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Front,
    Back,
}

#[derive(Component)]
pub struct LeftMover {}

#[derive(Component)]
pub struct RandomMover {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct FieldOfView {
    pub visibile_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}
