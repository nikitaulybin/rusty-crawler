use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera, ecs: &SubWorld) {
    let fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(MAP_CONSOLE);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            if map.in_bounds(pt)
                && (fov.visible_tiles.contains(&pt) | map.revealed_tiles[map_idx(x, y)])
            {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };
                let color_pair = if fov.visible_tiles.contains(&pt) {
                    ColorPair::new(WHITE, BLACK)
                } else {
                    ColorPair::new(DARK_GRAY, BLACK)
                };
                draw_batch.set(pt - camera.offset(), color_pair, glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch Error");
}
