use crate::player::PLAYER_START_POS;
use rltk::{RGB, RandomNumberGenerator, Rltk};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn generate_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 50 * 80];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(PLAYER_START_POS.x, PLAYER_START_POS.y) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(tile_map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in tile_map {
        match tile {
            TileType::Wall => ctx.set(
                x,
                y,
                RGB::named(rltk::LIME_GREEN),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('#'),
            ),
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('.'),
            ),
        }
        x += 1;
        if x > 79 {
            y += 1;
            x = 0;
        }
    }
}
