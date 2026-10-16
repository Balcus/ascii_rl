use specs::{Entities, Join, ReadStorage, System, WriteExpect};

use crate::{components::{BlocksTile, Position}, map::Map};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers, entities) = data;
        
        map.populate_blocked();
        map.clear_content_index();
        for (entity, pos) in (&entities, &position).join() {
            let idx = Map::xy_idx(pos.x, pos.y);
            let _p: Option<&BlocksTile> = blockers.get(entity);
            if let Some(_p) = _p {
                map.blocked[idx] = true;
            }

            map.tile_content[idx].push(entity);
        }
    }
}