use uuid::Uuid;

pub struct Player {
    id: Uuid,
    name: String,
    money: f64,
    bankrupt: bool
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self { id: Uuid::new_v4(), name: String::from(name), money: 1500.00, bankrupt: false }
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
}