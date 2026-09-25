mod components;
mod map;
mod player;
mod systems;

use crate::{
    components::{LeftMover, Player, Position, RandomMover, Renderable},
    map::{TileType, draw_map, generate_map},
    player::{PLAYER_START_POS, handle_player_input},
    systems::rand_mv_system::RandomMovementSystem,
};
use rltk::{GameState, RGB, Rltk};
use specs::prelude::*;

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut rms = RandomMovementSystem {};
        rms.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        handle_player_input(self, ctx);
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let rendarables = self.ecs.read_storage::<Renderable>();

        for (pos, r) in (&positions, &rendarables).join() {
            ctx.set(pos.x, pos.y, r.fg, r.bg, r.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("RL").build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<RandomMover>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(generate_map());

    gs.ecs
        .create_entity()
        .with(PLAYER_START_POS)
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // Generate 10 random entities
    // for _ in 1..10 {
    //     let mut rng = RandomNumberGenerator::new();
    //     let x = rng.range(0, 80);
    //     let y = rng.range(0, 50);

    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x, y })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('*'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(RandomMover {
    //             direction: Direction::Front,
    //         })
    //         .build();
    // }

    rltk::main_loop(context, gs)
}
