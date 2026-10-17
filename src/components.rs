use std::vec;

use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

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

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Failed to insert damage!");
        }
    }
}
