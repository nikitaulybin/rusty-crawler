use crate::prelude::*;
use std::borrow::Borrow;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
#[read_component(ProvidesHealing)]
#[read_component(RevealsMap)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    #[resource] map: &Map,
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
            VirtualKeyCode::Return => {
                let (_, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();

                let player_idx = map_idx(player_pos.x, player_pos.y);
                if map.tiles[player_idx] == TileType::Exit {
                    *turn_state = TurnState::NextLevel;
                    Point::zero()
                } else {
                    Point::zero()
                }
            }
            _ => Point::zero(),
        };
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

    let mut item_map: IndexMap<&str, Vec<Entity>> = IndexMap::new();
    <(Entity, &Name, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, _, carried)| carried.0 == *player_entity)
        .for_each(|(item_entity, name, _, _)| {
            if let Some(items_added) = item_map.remove(&name.0.borrow()) {
                let mut new_items = items_added;
                new_items.push(*item_entity);
                item_map.insert(name.0.borrow(), new_items);
            } else {
                item_map.insert(name.0.borrow(), vec![*item_entity]);
            }
        });

    if let Some((_name, items)) = item_map.get_index(index) {
        let mut items_copy = items.clone();
        let item = items_copy.pop().unwrap();
        commands.push((ActivateItem {
            item,
            applied_by: *player_entity,
        },));
    }
    Point::zero()
}
