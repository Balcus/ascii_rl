use crate::{
    RunState,
    components::{Enemy, FieldOfView, Position, WantsToMelee},
    map::Map,
};
use rltk::Point;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player,
            run_state,
            entities,
            mut fov,
            mut pos,
            enemies,
            mut wants_to_melee,
        ) = data;

        if *run_state != RunState::MonsterTurn {
            return;
        }

        for (entity, fov, pos, _enemy) in (&entities, &mut fov, &mut pos, &enemies).join() {
            let distance =
                rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee
                    .insert(entity, WantsToMelee { target: *player })
                    .expect("Failed for enemy to attack player");
                continue;
            }
            if fov.visibile_tiles.contains(&*&player_pos) {
                let path = rltk::a_star_search(
                    Map::xy_idx(pos.x, pos.y) as i32,
                    Map::xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                if path.success && path.steps.len() > 1 {
                    let old_idx = Map::xy_idx(pos.x, pos.y);
                    map.blocked[old_idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    let new_idx = Map::xy_idx(pos.x, pos.y);
                    map.blocked[new_idx] = true;
                    fov.dirty = true;
                }
            }
        }
    }
}
