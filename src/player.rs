use crate::{
    State, TileType,
    components::{Player, Position},
    map::xy_idx,
};
use rltk::Rltk;
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

pub const PLAYER_START_POS: Position = Position { x: 50, y: 25 };

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut players = ecs.write_storage::<Player>();
    let mut position = ecs.write_storage::<Position>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_, pos) in (&mut players, &mut position).join() {
        let idx = xy_idx(pos.x + dx, pos.y + dy);
        if map[idx] == TileType::Wall {
            break;
        }
        pos.x = min(79, max(0, pos.x + dx));
        pos.y = min(49, max(0, pos.y + dy));
    }
}

pub fn handle_player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(rltk::VirtualKeyCode::W) => try_move_player(0, -1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::A) => try_move_player(-1, 0, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::S) => try_move_player(0, 1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::D) => try_move_player(1, 0, &mut gs.ecs),
        _ => {}
    }
}
