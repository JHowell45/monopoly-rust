pub mod player;
pub mod property;

use std::{collections::HashMap};
use uuid::Uuid;
use player::Player;

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    player_movements: HashMap<Uuid, u32>,
}

impl Game {
    pub fn new() -> Self {
        Self { players: Vec::new(), player_movements: HashMap::new() }
    }

    pub fn add_player(&mut self, name: &str) {
        self.players.push(Player::new(name));
    }
}