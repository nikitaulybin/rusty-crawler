use std::borrow::Borrow;

use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut player_query = <(Entity, &Health)>::query().filter(component::<Player>());
    let (player_entity, player_health) = player_query.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(HUD_CONSOLE);
    draw_batch.bar_horizontal(
        Point::zero(),
        DISPLAY_WIDTH * 2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK),
    );

    draw_batch.print_color_centered(
        0,
        format!("Health: {} / {}", player_health.current, player_health.max),
        ColorPair::new(WHITE, RED),
    );

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

    let mut y = 3;
    for (name, count) in item_map.iter() {
        draw_batch.print(Point::new(3, y), format!("{} : {} x{}", y - 2, name, count));
        y += 1;
    }

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }
    draw_batch
        .submit((SCREEN_WIDTH * SCREEN_HEIGHT + 2000) as usize)
        .expect("Batch Error");
}
