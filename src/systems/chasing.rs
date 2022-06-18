use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
pub fn chasing(ecs: &SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut players = <(Entity, &Point, &Player)>::query();

    let player_entity: &Entity = players.iter(ecs).next().unwrap().0;
    let player_pos: &Point = players.iter(ecs).next().unwrap().1;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);
    let mut rng = RandomNumberGenerator::new();
    movers
        .iter(ecs)
        .for_each(|(enemy_entity, enemy_pos, _, fov)| {
            if !fov.visible_tiles.contains(player_pos) {
                commands.push((WantsToMove {
                    entity: *enemy_entity,
                    destination: *enemy_pos
                        + match rng.range(0, 4) {
                            0 => Point::new(1, 0),
                            1 => Point::new(0, 1),
                            2 => Point::new(-1, 0),
                            _ => Point::new(0, -1),
                        },
                },));
            } else {
                let enemy_idx = map.point2d_to_index(*enemy_pos);
                if let Some(destination) =
                    DijkstraMap::find_lowest_exit(&dijkstra_map, enemy_idx, map)
                {
                    let distance = DistanceAlg::Pythagoras.distance2d(*enemy_pos, *player_pos);
                    if distance >= 1.2 {
                        commands.push((WantsToMove {
                            entity: *enemy_entity,
                            destination: map.index_to_point2d(destination),
                        },));
                    } else {
                        commands.push((WantsToAttack {
                            attacker: *enemy_entity,
                            victim: *player_entity,
                        },));
                    };
                }
            }
        });
}
