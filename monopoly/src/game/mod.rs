pub mod player;
pub mod property;

use std::collections::HashMap;
use uuid::Uuid;
use player::Player;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    player_movements: HashMap<Uuid, u32>,
    current_round: u64,
    board_size: u32
}

impl Game {
    pub fn new() -> Self {
        Self { players: Vec::new(), player_movements: HashMap::new(), current_round: 0, board_size: 40 }
    }

    pub fn add_player(&mut self, name: &str) {
        let player = Player::new(name);
        let player_id = player.get_id();
        self.players.push(player);
        self.player_movements.insert(player_id, 0);
    }

    pub fn update_player_position(&mut self, player_id: Uuid, movement: u32) {
        if let Some(player) = self.player_movements.get_mut(&player_id) {
            *player += movement;
        }
    }

    pub fn next_round(&mut self) {
        for player in self.players.iter_mut() {
            let dice = player.roll();
            let roll: u32 = dice.iter().sum::<u8>().into();
            println!("{:?} = {}", dice, roll);
            // self.update_player_position(player.get_id(), roll)
        }
        // self.current_round += 1;
    }
}