use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(PLAYER_CONSOLE);
    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            if player_fov.visible_tiles.contains(pos) {
                draw_batch.set(*pos - camera.offset(), render.color, render.glyph);
            }
        });
    draw_batch
        .submit((SCREEN_WIDTH * SCREEN_HEIGHT + 1000) as usize)
        .expect("Batch Error");
}
