use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(RevealsMap)]
#[write_component(Health)]
pub fn use_items(#[resource] map: &mut Map, ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut items = <(Entity, &ActivateItem)>::query();
    let mut healing_queue: Vec<(Entity, ProvidesHealing)> = Vec::new();
    items.iter(ecs).for_each(|(intent, activation)| {
        let item_ref = ecs.entry_ref(activation.item).unwrap();
        if let Ok(potion) = item_ref.get_component::<ProvidesHealing>() {
            healing_queue.push((activation.applied_by, *potion));
        }

        if let Ok(_mapper) = item_ref.get_component::<RevealsMap>() {
            map.revealed_tiles.iter_mut().for_each(|t| *t = true);
        }
        commands.remove(*intent);
        commands.remove(activation.item);
    });

    healing_queue.iter_mut().for_each(|(applied_by, potion)| {
        if let Ok(health) = ecs
            .entry_mut(*applied_by)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current = i32::min(health.max, health.current + potion.amount);
        }
    });
}
