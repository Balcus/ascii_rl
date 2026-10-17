mod components;
mod map;
mod player;
mod rect;
mod systems;

// w, h
const WINDOW_SIZE: (usize, usize) = (80, 50);

use crate::{
    components::{
        BlocksTile, CombatStats, Enemy, FieldOfView, LeftMover, Player, Position, RandomMover,
        Renderable, SufferDamage, WantsToMelee,
    },
    map::Map,
    player::handle_player_input,
    systems::{
        damage_system::{self, DamageSystem},
        enemy_ai_system::EnemyAI,
        map_indexing_system::MapIndexingSystem,
        melee_combat_system::MeleeCombatSystem,
        visibility_system::VisibilitySystem,
    },
};
use rltk::{GameState, RGB, Rltk};
use specs::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    GameOver,
}

struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vs = VisibilitySystem {};
        vs.run_now(&self.ecs);
        let mut mob = EnemyAI {};
        mob.run_now(&self.ecs);
        let mut map_indexing_system = MapIndexingSystem {};
        map_indexing_system.run_now(&self.ecs);
        let mut melee_combat_system = MeleeCombatSystem {};
        melee_combat_system.run_now(&self.ecs);
        let mut damage_system = DamageSystem {};
        damage_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate
        }

        match newrunstate {
            RunState::AwaitingInput => {
                newrunstate = handle_player_input(self, ctx);
            }
            RunState::PreRun => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::PlayerTurn => {
                self.run_systems();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::GameOver => {}
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

        damage_system::delete_the_dead(&mut self.ecs);

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
    gs.ecs.register::<Enemy>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();

    let map = Map::generate_room_map();

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('J'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(FieldOfView {
                visibile_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Enemy {})
            .with(BlocksTile {})
            .with(CombatStats {
                max_hp: 10,
                hp: 10,
                defense: 1,
                power: 5,
            })
            .build();
    }

    let (player_x, player_y) = map.rooms[0].center();
    let player_entity = gs
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 1,
            power: 3,
        })
        .build();

    gs.ecs.insert(map);
    gs.ecs.insert(rltk::Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::PreRun);
    rltk::main_loop(context, gs)
}
