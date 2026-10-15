use crate::{components::{Enemy, FieldOfView, Position}, map::Map};
use rltk::Point;
use specs::{Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

pub struct EnemyAI {}

impl<'a> System<'a> for EnemyAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Enemy>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut fov, mut pos, enemies) = data;
        for (fov, pos, _enemy) in (&mut fov, &mut pos, &enemies).join() {
            let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // console::log(&format!("Enemy attacks!"));
                return;
            }
            if fov.visibile_tiles.contains(&*&player_pos) {
                let path = rltk::a_star_search(
                    Map::xy_idx(pos.x, pos.y) as i32,
                    Map::xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map
                );
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    fov.dirty = true;
                }
            }
        }
    }
}
