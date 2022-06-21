use super::MapArchitect;
use crate::prelude::*;

const MIN_TILES: i32 = (SCREEN_WIDTH * SCREEN_HEIGHT) / 3;
const MAX_TURNS: i32 = 200;
const UNREACHABLE: f32 = f32::MAX;

pub struct DrunkardsWalkArchitect {}

impl DrunkardsWalkArchitect {
    fn build_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let mut start_point = center;
        while map.tiles.iter().filter(|t| **t == TileType::Floor).count() < MIN_TILES as usize {
            self.carve_region(&start_point, rng, map);

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[map.point2d_to_index(center)],
                map,
                1024.0,
            );

            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist == &UNREACHABLE)
                .for_each(|(idx, _)| map.tiles[idx] = TileType::Wall);

            start_point =
                map.index_to_point2d(rng.range(0, (SCREEN_WIDTH * SCREEN_HEIGHT) as usize));
        }
    }

    fn carve_region(
        &mut self,
        start_point: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map,
    ) {
        let mut current_point = *start_point;
        for _ in 0..MAX_TURNS {
            map.tiles[map_idx(current_point.x, current_point.y)] = TileType::Floor;
            current_point += match rng.range::<i8>(0, 4) {
                0 => Point::new(1, 0),
                1 => Point::new(0, 1),
                2 => Point::new(-1, 0),
                _ => Point::new(0, -1),
            };
            if !map.in_bounds(current_point) {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        mb.fill(TileType::Wall);
        self.build_map(rng, &mut mb.map);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);
        mb.amulet_start = mb.find_most_distant(&mb.player_start);
        mb
    }
}
