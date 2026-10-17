use rltk::console;
use specs::{Entity, Join, System, World, WorldExt, WriteStorage};

use crate::components::{CombatStats, Player, SufferDamage};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut combat_stats, mut suffer_damage) = data;

        for (stats, damage) in (&mut combat_stats, &mut suffer_damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        suffer_damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();
    {
        let combat_stats = ecs.read_storage::<CombatStats>();
        let players = ecs.read_storage::<Player>();
        let entites = ecs.entities();
        for (entity, stats) in (&entites, &combat_stats).join() {
            if stats.hp < 1 {
                let player = players.get(entity);
                if let Some(_) = player {
                    console::log("You died.");
                    continue;
                }
                dead.push(entity);
            }
        }
    }

    ecs.delete_entities(&dead)
        .expect("Falied to delete dead entities");
}
