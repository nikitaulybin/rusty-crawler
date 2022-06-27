use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let mut enemies = <(Entity, &Point, &Name)>::query();
    let offset = camera.offset();
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_CONSOLE);

    let player_fov = <&FieldOfView>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    enemies
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&map_pos))
        .for_each(|(entity, _, name)| {
            let mut screen_pos = *mouse_pos * 2;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{}: {}hp", name.0, health.current)
                } else {
                    name.0.clone()
                };
            let hud_width = DISPLAY_WIDTH * 2;
            let treshold_x = hud_width - display.chars().count() as i32;
            if screen_pos.x > treshold_x {
                screen_pos.x = treshold_x;
            }
            draw_batch.print(screen_pos + Point::new(0, -1), display);
        });
    draw_batch
        .submit((SCREEN_WIDTH * SCREEN_HEIGHT + 3000) as usize)
        .expect("Batch Error");
}
