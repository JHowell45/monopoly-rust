use uuid::Uuid;
use rand::Rng;

#[derive(Debug)]
pub struct Player {
    id: Uuid,
    name: String,
    money: f64,
    bankrupt: bool,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self { id: Uuid::new_v4(), name: String::from(name), money: 1500.00, bankrupt: false }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_money(&self) -> f64 {
        return self.money;
    }

    pub fn is_bankrupt(&self) -> bool {
        return self.bankrupt;
    }

    pub fn roll(&self) -> [u8; 2] {
        let mut rng = rand::thread_rng();
        let left_dice = rng.gen_range(0..6);
        let right_dice = rng.gen_range(0..6);
        return [left_dice, right_dice];
    }
}