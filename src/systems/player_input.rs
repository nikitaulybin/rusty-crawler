use crate::prelude::*;
use std::borrow::Borrow;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(ProvidesHealing)]
#[read_component(RevealsMap)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::G => {
                let (player_entity, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();

                let mut pickable_items = <(Entity, &Point, &Item)>::query();
                pickable_items
                    .iter(ecs)
                    .filter(|(_, pos, _)| **pos == player_pos)
                    .for_each(|(item_entity, _, _)| {
                        commands.remove_component::<Point>(*item_entity);
                        commands.add_component(*item_entity, Carried(player_entity));
                    });
                Point::zero()
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::zero(),
        };
        let moved = delta.x != 0 || delta.y != 0;
        players.iter(ecs).for_each(|(player_entity, pos)| {
            let destination = *pos + delta;
            let mut attack_initiated = false;
            let nearby_tiles = vec![
                *pos + Point::new(1, 0),
                *pos + Point::new(1, 1),
                *pos + Point::new(0, 1),
                *pos + Point::new(0, -1),
                *pos + Point::new(-1, 0),
                *pos + Point::new(-1, -1),
                *pos + Point::new(-1, 1),
                *pos + Point::new(1, -1),
            ];

            let enemies_nearby = enemies
                .iter(ecs)
                .filter(|(_, enemy_pos)| nearby_tiles.contains(enemy_pos))
                .count()
                > 0;

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

            if !attack_initiated && !moved && !enemies_nearby {
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

fn use_item(index: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut item_map: IndexMap<&str, i32> = IndexMap::new();
    <(&Item, &Name, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == *player_entity)
        .for_each(|(_, name, _)| {
            if let Some(item_count) = item_map.remove(&name.0.borrow()) {
                item_map.insert(name.0.borrow(), item_count + 1);
            } else {
                item_map.insert(name.0.borrow(), 1);
            }
        });

    if let Some(item) = item_map.get_index(index) {
        let is_potion = ecs
            .entry_ref(*item)
            .unwrap()
            .get_component::<ProvidesHealing>()
            .is_ok();
        if is_potion {
            println!("About to use healing potion")
        } else {
            println!("Using map")
        }
        commands.push((ActivateItem {
            item,
            applied_by: *player_entity,
        },));
    }
    Point::zero()
}
