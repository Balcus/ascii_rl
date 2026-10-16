use crate::{
    RunState, State, WINDOW_SIZE, components::{CombatStats, FieldOfView, Player, Position, WantsToMelee}, map::Map,
};
use rltk::{Rltk, console};
use specs::{Join, World, WorldExt};
use std::cmp::{max, min};

pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) {
    let mut players = ecs.write_storage::<Player>();
    let mut position = ecs.write_storage::<Position>();
    let mut player_pos = ecs.write_resource::<rltk::Point>();
    let mut fov = ecs.write_storage::<FieldOfView>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _, pos, fov) in (&entities, &mut players, &mut position, &mut fov).join() {
        if pos.x + dx < 1 || pos.x + dx > map.width-1 || pos.y + dy < 1 || pos.y + dy > map.height-1 { return; }
        let dest_idx = Map::xy_idx(pos.x + dx, pos.y + dy);
    
        for entity in map.tile_content[dest_idx].iter() {
            let target = combat_stats.get(*entity);
            if let Some(target) = target {
                wants_to_melee.insert(*entity, WantsToMelee { target: *entity }).expect("Failed to attack target!");
                return;
            }
        }

        if map.blocked[dest_idx] {
            break;
        }

        pos.x = min(WINDOW_SIZE.0 as i32 - 1, max(0, pos.x + dx));
        pos.y = min(WINDOW_SIZE.1 as i32 - 1, max(0, pos.y + dy));
        player_pos.x = pos.x;
        player_pos.y = pos.y;
        fov.dirty = true;
    }
}

pub fn handle_player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    match ctx.key {
        Some(rltk::VirtualKeyCode::W) => try_move_player(0, -1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::A) => try_move_player(-1, 0, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::S) => try_move_player(0, 1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::D) => try_move_player(1, 0, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::E) => try_move_player(1, -1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::Q) => try_move_player(-1, -1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::C) => try_move_player(1, 1, &mut gs.ecs),
        Some(rltk::VirtualKeyCode::Z) => try_move_player(-1, 1, &mut gs.ecs),
        _ => return RunState::Paused,
    }
    RunState::Running
}
