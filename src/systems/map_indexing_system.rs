use specs::{Join, ReadStorage, System, WriteExpect};

use crate::{components::{BlocksTile, Position}, map::Map};

pub struct MapIndexingSystem {}

impl <'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blockers) = data;
        
        map.populate_blocked();
        for (pos, _blocker) in (&position, &blockers).join() {
            let idx = Map::xy_idx(pos.x, pos.y);
            map.blocked[idx] = true;
        }
    }
}