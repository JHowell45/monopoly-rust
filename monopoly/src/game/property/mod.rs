use std::collections::HashMap;

pub struct PropertSet {
    colour: String,
    properties: HashMap<String, Property>
}

impl PropertSet {
    pub fn new() -> Self {
        Self { colour: String::from(""), properties: HashMap::new() }
    }

    pub fn number_of_properties(&self) -> usize {
        return self.properties.len();
    }
}

pub struct Property {
    name: String,
    base_rent: f64,
    house_rent: HashMap<u8, f64>,
    houses: u8,
    hotel: bool,
    mortgage_value: f64,
    mortgaged: bool,
    house_price: f64,
    hotel_price: f64
}

impl Property {
    pub fn new(name: &str, base_rent: f64, house_rent_map: HashMap<u8, f64>, mortgage_value: f64, house_price: f64, hotel_price: f64) -> Self {
        Self {
            name: String::from(name),
            base_rent: base_rent,
            house_rent: house_rent_map,
            houses: 0,
            hotel: false,
            mortgage_value: mortgage_value,
            mortgaged: false,
            house_price: house_price,
            hotel_price: hotel_price,
        }
    }

    pub fn get_house_count(&self) -> u8 {
        return self.houses;
    }

    pub fn has_hotel(&self) -> bool {
        return self.hotel;
    }

    pub fn is_mortgaged(&self) -> bool {
        return self.mortgaged;
    }

    pub fn add_house(&mut self) {
        if !self.hotel {
            if self.houses == 4 {
                self.hotel = true;
            } else {
                self.houses += 1;
            }
        } else {
            println!("Already have the max number of houses!");
        }
    }

    pub fn mortgage(&mut self) -> f64 {
        if !self.mortgaged {
            self.mortgaged = true;
        }
        return self.mortgage_value;
    }

    pub fn unmortgage(&mut self) {
        self.mortgaged = false;
    }

    pub fn rent(&self) -> f64 {
        if self.houses == 0 {
            return self.base_rent;
        } else if self.hotel {
            return self.house_rent[&5];
        } else {
            return self.house_rent[&self.houses];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn property_add_house() {
        let mut property = Property::new("test", 0.0, HashMap::from([(1, 2.0), (2, 2.0), (3, 3.0), (4, 4.0), (5, 5.0)]), 50.0, 50.0, 50.0);
        assert_eq!(0, property.get_house_count());
        assert_eq!(false, property.has_hotel());
        
        property.add_house();
        assert_eq!(1, property.get_house_count());
        assert_eq!(false, property.has_hotel());

        property.add_house();
        assert_eq!(2, property.get_house_count());
        assert_eq!(false, property.has_hotel());

        property.add_house();
        assert_eq!(3, property.get_house_count());
        assert_eq!(false, property.has_hotel());

        property.add_house();
        assert_eq!(4, property.get_house_count());
        assert_eq!(false, property.has_hotel());

        property.add_house();
        assert_eq!(4, property.get_house_count());
        assert_eq!(true, property.has_hotel());

        property.add_house();
        assert_eq!(4, property.get_house_count());
        assert_eq!(true, property.has_hotel());
    }

    #[test]
    fn test_mortgage() {
        let mut property = Property::new("test", 0.0, HashMap::from([(1, 2.0), (2, 2.0), (3, 3.0), (4, 4.0), (5, 5.0)]), 50.0, 50.0, 50.0);

        assert_eq!(false, property.is_mortgaged());

        let returned_value = property.mortgage();
        assert!(property.is_mortgaged());
        assert_eq!(50.0, returned_value);

        property.mortgage();
        assert!(property.is_mortgaged());


        property.unmortgage();
        assert!(!property.is_mortgaged());
    }
    
}