use std::cmp::{max, min};

use rltk::{GameState, RGB, RandomNumberGenerator, Rltk};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
    Front,
    Back,
}

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lms = LeftMovementSystem {};
        let mut rms = RandomMovementSystem {};
        lms.run_now(&self.ecs);
        rms.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();
        handle_player_input(self, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let rendarables = self.ecs.read_storage::<Renderable>();

        for (pos, r) in (&positions, &rendarables).join() {
            ctx.set(pos.x, pos.y, r.fg, r.bg, r.glyph);
        }
    }
}

#[derive(Component)]
struct LeftMover {}

struct LeftMovementSystem {}

impl<'a> System<'a> for LeftMovementSystem {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (l, mut pos): Self::SystemData) {
        for (_, pos) in (&l, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component)]
struct RandomMover {
    direction: Direction,
}

struct RandomMovementSystem {}

impl RandomMovementSystem {
    fn choose_direction(current_direction: &mut Direction, rng: &mut RandomNumberGenerator) {
        let previous_direciton = current_direction.clone();
        *current_direction = match rng.range(0, 9) {
            0 => Direction::Back,
            1 => Direction::Front,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => previous_direciton,
        }
    }

    fn update_position(direciton: &Direction, pos: &mut Position) {
        match direciton {
            Direction::Left => pos.x -= 1,
            Direction::Right => pos.x += 1,
            Direction::Front => pos.y += 1,
            Direction::Back => pos.y -= 1,
        }

        pos.x = pos.x.rem_euclid(80);
        pos.y = pos.y.rem_euclid(50);
    }
}

impl<'a> System<'a> for RandomMovementSystem {
    type SystemData = (WriteStorage<'a, RandomMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut rm, mut pos): Self::SystemData) {
        let mut rng = RandomNumberGenerator::new();

        for (rm, pos) in (&mut rm, &mut pos).join() {
            Self::choose_direction(&mut rm.direction, &mut rng);
            Self::update_position(&rm.direction, pos);
        }
    }
}

#[derive(Component)]
struct Player {}

fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut players = ecs.write_storage::<Player>();
    let mut position = ecs.write_storage::<Position>();

    for (_, pos) in (&mut players, &mut position).join() {
        pos.x = min(79 , max(0, pos.x + dx));
        pos.y = min(49, max(0, pos.y + dy));
    }
}

fn handle_player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {},
        Some(rltk::VirtualKeyCode::W) => try_move_player(0, -1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::A) => try_move_player(-1, 0, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::S) => try_move_player(0, 1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::D) => try_move_player(1, 0, &mut gs.ecs),
        _ => {}
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

    gs.ecs
        .create_entity()
        .with(Position { x: 50, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::GREEN),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for _ in 1..10 {
        let mut rng = RandomNumberGenerator::new();
        let x = rng.range(0, 80);
        let y = rng.range(0, 50);

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: rltk::to_cp437('*'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(RandomMover {
                direction: Direction::Front,
            })
            .build();
    }

    rltk::main_loop(context, gs)
}
