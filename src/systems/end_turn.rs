use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let current_state = *turn_state;
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    if let Some((player_health, player_pos)) = <(&Health, &Point)>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
    {
        if player_health.current < 1 {
            new_state = TurnState::GameOver;
        }

        let amulet_pos = <&Point>::query()
            .filter(component::<AmuletOfYala>())
            .iter(ecs)
            .next()
            .unwrap();
        if amulet_pos == player_pos {
            new_state = TurnState::Victory;
        }
    }

    *turn_state = new_state;
}
