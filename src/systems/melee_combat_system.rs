use specs::{Entities, Join, ReadStorage, System, WriteStorage};

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
        let (entities, mut wants_melee, combat_stats, mut suffer_datame) = data;

        for (_entity, wants_melee, stats) in (&entities, &wants_melee, &combat_stats).join() {
            if stats.hp > 0 {
                let target_stats = combat_stats.get(wants_melee.target).unwrap();
                if target_stats.hp > 0 {
                    let dmg = i32::max(0, stats.power - target_stats.defense);
                    if dmg > 0 {
                        SufferDamage::new_damage(&mut suffer_datame, wants_melee.target, dmg);
                    }
                }
            }
        }

        wants_melee.clear();
    }
}
