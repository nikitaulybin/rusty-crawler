use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);
        let is_player = ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
        if let Ok(fov) = ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<FieldOfView>()
        {
            commands.add_component(want_move.entity, FieldOfView::clone_dirty(fov));
            if is_player {
                let visible_tiles = &fov.visible_tiles;
                visible_tiles
                    .iter()
                    .for_each(|p| map.revealed_tiles[map_idx(p.x, p.y)] = true)
            }
        }

        if is_player {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}
