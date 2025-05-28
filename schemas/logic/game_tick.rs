use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameTickInput {
    pub player_id: String,
    pub player_pos: (i32, i32),
    pub velocity: (i32, i32),
    pub actions: Vec<String>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameTickOutput {
    pub new_position: (i32, i32),
    pub updated_ammo: i32,
    pub event_log: Vec<String>,
}

pub fn run_game_tick(input: GameTickInput, current_ammo: i32) -> Option<GameTickOutput> {
    let new_x = input.player_pos.0 + input.velocity.0;
    let new_y = input.player_pos.1 + input.velocity.1;
    let mut ammo = current_ammo;
    let mut events = vec![];

    for action in &input.actions {
        match action.as_str() {
            "shoot" => {
                if ammo > 0 {
                    ammo -= 1;
                    events.push("shot_fired".to_string());
                } else {
                    events.push("no_ammo".to_string());
                }
            },
            "jump" => {
                events.push("jumped".to_string());
            },
            _ => {}
        }
    }

    Some(GameTickOutput {
        new_position: (new_x, new_y),
        updated_ammo: ammo,
        event_log: events,
    })
}