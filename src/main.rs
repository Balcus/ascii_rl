mod components;
mod map;
mod player;
mod rect;
mod systems;

// w, h
const WINDOW_SIZE: (usize, usize) = (80, 50);

use crate::{
    components::{FieldOfView, LeftMover, Player, Position, RandomMover, Renderable},
    map::{Map, TileType},
    player::handle_player_input,
    systems::visibility_system::VisibilitySystem,
};
use rltk::{GameState, RGB, Rltk};
use specs::prelude::*;

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        // let mut rms = RandomMovementSystem {};
        // rms.run_now(&self.ecs);
        let mut vs = VisibilitySystem {};
        vs.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        handle_player_input(self, ctx);
        Map::draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let rendarables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, r) in (&positions, &rendarables).join() {
            let idx = Map::xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, r.fg, r.bg, r.glyph);
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple(WINDOW_SIZE.0, WINDOW_SIZE.1)?
        .with_title("RL")
        .build()?;
    context.with_post_scanlines(true);

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<RandomMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<FieldOfView>();

    let map = Map::generate_room_map();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        gs.ecs
        .create_entity()
        .with(Position {x, y})
        .with(Renderable {
            glyph: rltk::to_cp437('J'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        // .with(FieldOfView {
        //     visibile_tiles: Vec::new(),
        //     range: 8,
        //     dirty: true
        // })
        .build();
    }

    let (x, y) = map.rooms[0].center();
    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(FieldOfView {
            visibile_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    rltk::main_loop(context, gs)
}
