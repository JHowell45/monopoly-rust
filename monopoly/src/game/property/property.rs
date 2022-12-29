use core::panic;
use std::collections::HashMap;

use uuid::Uuid;

#[derive(Debug)]
pub struct Property {
    id: Uuid,
    name: String,
    owner_id: Option<Uuid>,
    colour: String,
    purchase_price: f64,
    houses: u8,
    rent_map: HashMap<u8, f64>,
    mortgaged: bool,
}

impl Property {
    pub fn new(name: &str, colour: &str, purchase_price: f64, rent_map: HashMap<u8, f64>) -> Self {
        Self { id: Uuid::new_v4(), name: String::from(name), owner_id: None, colour: String::from(colour), purchase_price: purchase_price, houses: 0, rent_map: rent_map, mortgaged: false }
    }

    pub fn get_id(&self) -> Uuid {
        return self.id;
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_owner(&self) -> Option<Uuid> {
        return self.owner_id;
    }

    pub fn set_owner(&mut self, owner_id: Uuid) {
        if self.owner_id == None {
            self.owner_id = Some(owner_id);
        } else {
            panic!("Owner already set!");
        }
    }

    pub fn get_colour(&self) -> &String {
        return &self.colour;
    }

    pub fn get_purchase_price(&self) -> f64 {
        return self.purchase_price;
    }

    pub fn get_mortgage_amount(&self) -> f64 {
        return self.get_purchase_price() / 2.0;
    }

    pub fn get_houses(&self) -> u8 {
        return self.houses;
    }

    pub fn add_house(&mut self) {
        if self.houses < 5 {
            self.houses += 1;
        } else {
            panic!("Cannot add anymore houses / hotels!");
        }
    }

    pub fn get_rent(&self) -> f64 {
        return *self.rent_map.get(&self.houses).unwrap();
    }

    pub fn is_mortgaged(&self) -> bool {
        return self.mortgaged;
    }

    pub fn mortgage(&mut self) {
        if !self.mortgaged {
            self.mortgaged = true;
        }
    }

    pub fn unmortgage(&mut self) {
        if self.mortgaged {
            self.mortgaged = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert_eq!("Old Kent Road", prop.get_name());
    }

    #[test]
    fn test_get_owner() {
        let mut prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert_eq!(None, prop.get_owner());

        let test_owner_id = Uuid::new_v4();
        prop.set_owner(test_owner_id);
        assert_eq!(Some(test_owner_id), prop.get_owner());
    }

    #[test]
    fn test_get_colour() {
        let prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert_eq!("Brown", prop.get_colour());
    }

    #[test]
    fn get_purchase_price() {
        let prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert_eq!(60.0, prop.get_purchase_price());
    }

    #[test]
    fn test_get_mortgage_amount() {
        let prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert_eq!(30.0, prop.get_mortgage_amount());
    }

    #[test]
    fn test_mortgage() {
        let mut prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));

        assert!(!prop.is_mortgaged());

        prop.mortgage();
        assert!(prop.is_mortgaged());

        prop.unmortgage();
        assert!(!prop.is_mortgaged());
    }

    #[test]
    fn test_houses() {
        let mut prop = Property::new("Old Kent Road", "Brown", 60.0, HashMap::from([(0, 2.0), (1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]));
        assert_eq!(0, prop.get_houses());

        prop.add_house();
        assert_eq!(1, prop.get_houses());

        prop.add_house();
        assert_eq!(2, prop.get_houses());

        prop.add_house();
        assert_eq!(3, prop.get_houses());

        prop.add_house();
        assert_eq!(4, prop.get_houses());

        prop.add_house();
        assert_eq!(5, prop.get_houses());
    }
}