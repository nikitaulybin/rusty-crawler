use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim, attack.attacker))
        .collect();

    victims.iter().for_each(|(message, victim, attacker)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = ecs
            .entry_ref(*attacker)
            .unwrap()
            .get_component::<Damage>()
            .unwrap()
            .0;
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= base_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message)
    })
}
