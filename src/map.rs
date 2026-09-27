use crate::{WINDOW_SIZE, rect::Rectangle};
use rltk::{Algorithm2D, BaseMap, Point, RGB, RandomNumberGenerator, Rltk};
use specs::World;
use std::{
    cmp::{max, min},
    vec,
};

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[allow(dead_code)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rectangle>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(x: i32, y: i32) -> usize {
        (y as usize * WINDOW_SIZE.0) + x as usize
    }

    pub fn generate_room_map() -> Self {
        let mut map = Self {
            tiles: vec![TileType::Wall; WINDOW_SIZE.0 * WINDOW_SIZE.1],
            rooms: Vec::new(),
            width: WINDOW_SIZE.0 as i32,
            height: WINDOW_SIZE.1 as i32,
            revealed_tiles: vec![false; WINDOW_SIZE.0 * WINDOW_SIZE.1],
            visible_tiles: vec![false; WINDOW_SIZE.0 * WINDOW_SIZE.1],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);

            let x = rng.roll_dice(1, WINDOW_SIZE.0 as i32 - w - 1) - 1;
            let y = rng.roll_dice(1, WINDOW_SIZE.1 as i32 - h - 1) - 1;

            let room = Rectangle::new(x, y, w, h);

            let mut apply = true;

            for other in &map.rooms {
                if other.intersect(&room) {
                    apply = false;
                }
            }

            if apply {
                Self::apply_room_to_map(&room, &mut map);

                if !map.rooms.is_empty() {
                    let (x, y) = room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        Self::generate_horizontal_tunnel(&mut map, x, prev_x, prev_y);
                        Self::generate_vertical_tunnel(&mut map, y, prev_y, x);
                    } else {
                        Self::generate_vertical_tunnel(&mut map, y, prev_y, prev_x);
                        Self::generate_horizontal_tunnel(&mut map, x, prev_x, y);
                    }
                }

                map.rooms.push(room);
            }
        }

        map
    }

    fn generate_horizontal_tunnel(map: &mut Self, r1x: i32, r2x: i32, y: i32) {
        for x in min(r1x, r2x)..=max(r1x, r2x) {
            let idx = Self::xy_idx(x, y);
            if idx > 0 && idx < WINDOW_SIZE.0 * WINDOW_SIZE.1 {
                map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn generate_vertical_tunnel(map: &mut Self, r1y: i32, r2y: i32, x: i32) {
        for y in min(r1y, r2y)..=max(r1y, r2y) {
            let idx = Self::xy_idx(x, y);
            if idx > 0 && idx < WINDOW_SIZE.0 * WINDOW_SIZE.1 {
                map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_room_to_map(room: &Rectangle, map: &mut Self) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                map.tiles[Self::xy_idx(x, y)] = TileType::Floor;
            }
        }
    }

    pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
        let map = ecs.fetch::<Map>();

        let mut x = 0;
        let mut y = 0;

        for (idx, tile) in map.tiles.iter().enumerate() {
            if map.revealed_tiles[idx] {
                let glyph;
                let mut fg;
                match tile {
                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0.6118, 0.3412, 0.0314);
                    }
                    TileType::Floor => {
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(1.0, 0.8235, 0.0);
                    }
                }

                if !map.visible_tiles[idx] {
                    fg = fg.to_greyscale()
                }

                ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }

            x += 1;
            if x > WINDOW_SIZE.0 as i32 - 1 {
                y += 1;
                x = 0;
            }
        }
    }
}
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> rltk::prelude::Point {
        Point::new(self.width, self.height)
    }
}
