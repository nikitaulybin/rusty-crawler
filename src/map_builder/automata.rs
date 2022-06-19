use super::MapArchitect;
use crate::prelude::*;

pub struct AutomataArchitect {}

impl AutomataArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        })
    }

    fn count_neighbours(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbours = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbours = self.count_neighbours(x, y, map);
                let idx = map_idx(x, y);
                if neighbours == 0 || neighbours > 4 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let closest_idx = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (
                    idx,
                    DistanceAlg::Pythagoras.distance2d(
                        Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
                        map.index_to_point2d(idx),
                    ),
                )
            })
            .min_by(|(_, distance1), (_, distance2)| distance1.partial_cmp(distance2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        map.index_to_point2d(closest_idx)
    }
}

impl MapArchitect for AutomataArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..50 {
            self.iteration(&mut mb.map);
        }
        let player_start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&player_start, rng);
        mb.player_start = player_start;
        mb.amulet_start = mb.find_most_distant(&player_start);
        mb
    }
}
