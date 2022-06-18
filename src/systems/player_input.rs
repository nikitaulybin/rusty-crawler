use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::D => Point::new(1, 0),
            _ => Point::zero(),
        };
        let moved = delta.x != 0 || delta.y != 0;
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        players.iter(ecs).for_each(|(player_entity, pos)| {
            let destination = *pos + delta;
            let mut attack_initiated = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(enemy_entity, _)| {
                    attack_initiated = true;
                    commands.push((WantsToAttack {
                        attacker: *player_entity,
                        victim: *enemy_entity,
                    },));
                });
            if !attack_initiated {
                commands.push((WantsToMove {
                    entity: *player_entity,
                    destination,
                },));
            }

            if !attack_initiated && !moved {
                if let Ok(mut health) = ecs
                    .clone()
                    .entry_mut(*player_entity)
                    .unwrap()
                    .get_component_mut::<Health>()
                {
                    health.current = i32::min(health.max, health.current + 1);
                }
            }
        });

        *turn_state = TurnState::PlayerTurn;
    }
}
