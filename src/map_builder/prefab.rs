use crate::prelude::*;

const UNREACHABLE: f32 = f32::MAX;

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let prefab: Prefab = match rng.range::<i8>(0, 2) {
        0 => FORTRESS,
        1 => CHESS,
        _ => FORTRESS,
    };
    let mut placement: Option<Point> = None;
    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - prefab.width),
            rng.range(0, SCREEN_HEIGHT - prefab.height),
            prefab.width,
            prefab.height,
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = map_idx(pt.x, pt.y);
            let distance = dijkstra_map.map[idx];

            if distance != UNREACHABLE && distance > 20.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });
        attempts += 1;

        if can_place {
            println!("Found placement");
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
        }
    }

    if let Some(placement) = placement {
        let prefab_chars: Vec<char> = prefab
            .structure_str
            .chars()
            .filter(|c| *c != '\n' && *c != '\r' && *c != ' ')
            .collect();
        let mut current_char_idx = 0;
        for py in placement.y..placement.y + prefab.height {
            for px in placement.x..placement.x + prefab.width {
                let map_idx = map_idx(px, py);
                let c = prefab_chars[current_char_idx];
                match c {
                    '-' => mb.map.tiles[map_idx] = TileType::Floor,
                    '#' => mb.map.tiles[map_idx] = TileType::Wall,
                    'M' => {
                        mb.map.tiles[map_idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(px, py));
                    }
                    _ => {
                        println!("Unhandled char: {}", c);
                    }
                }
                current_char_idx += 1;
            }
        }
    }
}
