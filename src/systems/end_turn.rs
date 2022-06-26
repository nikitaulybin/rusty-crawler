use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let current_state = *turn_state;
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::EnemyTurn,
        TurnState::EnemyTurn => TurnState::AwaitingInput,
        TurnState::NextLevel => TurnState::AwaitingInput,
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
        let amulet_default = Point::new(-1, -1);
        let amulet_pos = <&Point>::query()
            .filter(component::<AmuletOfYala>())
            .iter(ecs)
            .next()
            .unwrap_or(&amulet_default);
        if amulet_pos == player_pos {
            new_state = TurnState::Victory;
        }

        let player_idx = map_idx(player_pos.x, player_pos.y);
        if map.tiles[player_idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    }

    *turn_state = new_state;
}
