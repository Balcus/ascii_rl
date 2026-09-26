use rltk::RandomNumberGenerator;
use specs::{Join, System, WriteStorage};

use crate::{WINDOW_SIZE, components::{Direction, Position, RandomMover}};

#[allow(dead_code)]
pub struct RandomMovementSystem {}

impl RandomMovementSystem {
    #[allow(dead_code)]
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

        pos.x = pos.x.rem_euclid(WINDOW_SIZE.0 as i32);
        pos.y = pos.y.rem_euclid(WINDOW_SIZE.1 as i32);
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
