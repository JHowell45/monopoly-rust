pub mod player;
pub mod property;

use player::Player;

pub struct Game {
    players: Vec<Player>
}

impl Game {
    pub fn new() -> Self {
        Self { players: Vec::new() }
    }

    pub fn add_player(&mut self, name: &str) {
        self.players.push(Player::new(name));
    }
}