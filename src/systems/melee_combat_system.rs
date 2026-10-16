use specs::{Entities, ReadStorage, System, WriteStorage};

use crate::components::{CombatStats, SufferDamage, WantsToMelee};

pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToMelee>,
        ReadStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        todo!()
    }
}
