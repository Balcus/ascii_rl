use crate::{
    WINDOW_SIZE,
    components::{FieldOfView, Player, Position},
    map::Map,
};
use rltk::{Point, field_of_view};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut fov, pos, player) = data;
        for (entity, fov, pos) in (&entities, &mut fov, &pos).join() {
            if !fov.dirty {
                continue;
            }

            fov.visibile_tiles.clear();
            fov.visibile_tiles = field_of_view(Point::new(pos.x, pos.y), fov.range, &*map);
            fov.visibile_tiles.retain(|t| {
                t.x >= 0 && t.x < WINDOW_SIZE.0 as i32 && t.y >= 0 && t.y < WINDOW_SIZE.1 as i32
            });

            let p: Option<&Player> = player.get(entity);
            if let Some(_) = p {
                for t in map.visible_tiles.iter_mut() {
                    *t = false;
                }
                for vis in &fov.visibile_tiles {
                    let idx = Map::xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }

            fov.dirty = false;
        }
    }
}
