use std::i8;

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    let mut targets = <(Entity, &Point, &Health)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let new_pos = match rng.range::<i8>(0, 4) {
            0 => Point::new(1, 0),
            1 => Point::new(0, 1),
            2 => Point::new(-1, 0),
            _ => Point::new(0, -1),
        } + *pos;
        let mut attacked = false;
        targets
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == new_pos)
            .for_each(|(target, _, _)| {
                if ecs
                    .entry_ref(*target)
                    .unwrap()
                    .get_component::<Player>()
                    .is_ok()
                {
                    commands.push((WantsToAttack {
                        attacker: *entity,
                        victim: *target,
                    },));
                }
                attacked = true;
            });
        if !attacked {
            commands.push((WantsToMove {
                entity: *entity,
                destination: new_pos,
            },));
        }
    })
}
