use std::collections::HashMap;

#[path = "./base.rs"] mod base;

use base::Property;

#[derive(Debug)]
pub struct PropertySet {
    colour: String,
    properties: HashMap<String, Property>,
    total_properties: u8,
}

impl PropertySet {
    pub fn new(colour: &str, total_properties: u8) -> Self {
        Self { colour: String::from(colour), properties: HashMap::new(), total_properties: total_properties }
    }

    pub fn brown() -> Self {
        let mut properties = HashMap::new();
        let colour = "Brown";
        let name = "Old Kent Road";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 60.0, 2.0, HashMap::from([(1, 10.0), (2, 30.0), (3, 90.0), (4, 160.0), (5, 250.0)]), 50.0, 50.0)
        );
        let name = "Whitechapel Road";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 60.0, 4.0, HashMap::from([(1, 20.0), (2, 60.0), (3, 180.0), (4, 320.0), (5, 450.0)]), 50.0, 50.0)
        );
        let length = properties.len() as u8;
        return Self { colour: String::from(colour), properties: properties, total_properties: length }
    }

    pub fn light_blue() -> Self {
        let mut properties = HashMap::new();
        let colour = "Light Blue";
        let name = "The Angel, Islington";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 100.0, 6.0, HashMap::from([(1, 30.0), (2, 90.0), (3, 270.0), (4, 400.0), (5, 550.0)]), 50.0, 50.0)
        );
        let name = "Euston Road";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 100.0, 6.0, HashMap::from([(1, 30.0), (2, 90.0), (3, 270.0), (4, 400.0), (5, 550.0)]), 50.0, 50.0)
        );
        let name = "Pentonville Road";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 120.0, 8.0, HashMap::from([(1, 40.0), (2, 100.0), (3, 300.0), (4, 450.0), (5, 600.0)]), 50.0, 50.0)
        );
        let length = properties.len() as u8;
        return Self { colour: String::from(colour), properties: properties, total_properties: length }
    }

    pub fn pink() -> Self {
        let mut properties = HashMap::new();
        let colour = "Pink";
        let name = "Pall Mall";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 140.0, 10.0, HashMap::from([(1, 50.0), (2, 150.0), (3, 450.0), (4, 625.0), (5, 750.0)]), 100.0, 100.0)
        );
        let name = "Whitehall";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 140.0, 10.0, HashMap::from([(1, 50.0), (2, 150.0), (3, 450.0), (4, 625.0), (5, 750.0)]), 100.0, 100.0)
        );
        let name = "Northumberland Avenue";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 160.0, 12.0, HashMap::from([(1, 60.0), (2, 180.0), (3, 500.0), (4, 700.0), (5, 900.0)]), 100.0, 100.0)
        );
        let length = properties.len() as u8;
        return Self { colour: String::from(colour), properties: properties, total_properties: length }
    }

    pub fn orange() -> Self {
        let mut properties = HashMap::new();
        let colour = "Orange";
        let name = "Bow Street";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 180.0, 14.0, HashMap::from([(1, 70.0), (2, 200.0), (3, 550.0), (4, 750.0), (5, 950.0)]), 100.0, 100.0)
        );
        let name = "Marlborough Street";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 140.0, 10.0, HashMap::from([(1, 50.0), (2, 150.0), (3, 450.0), (4, 625.0), (5, 750.0)]), 100.0, 100.0)
        );
        let name = "Vine Street";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 200.0, 16.0, HashMap::from([(1, 80.0), (2, 220.0), (3, 600.0), (4, 800.0), (5, 1000.0)]), 100.0, 100.0)
        );
        let length = properties.len() as u8;
        return Self { colour: String::from(colour), properties: properties, total_properties: length }
    }

    pub fn red() -> Self {
        let mut properties = HashMap::new();
        let colour = "Red";
        let name = "The Strand";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 220.0, 18.0, HashMap::from([(1, 90.0), (2, 250.0), (3, 700.0), (4, 875.0), (5, 1050.0)]), 150.0, 150.0)
        );
        let name = "Fleet Street";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 220.0, 18.0, HashMap::from([(1, 90.0), (2, 250.0), (3, 700.0), (4, 875.0), (5, 1050.0)]), 150.0, 150.0)
        );
        let name = "Trafalgar Square";
        properties.insert(
            String::from(name), 
            Property::new(name, colour, 200.0, 20.0, HashMap::from([(1, 100.0), (2, 300.0), (3, 750.0), (4, 925.0), (5, 1100.0)]), 150.0, 150.0)
        );
        let length = properties.len() as u8;
        return Self { colour: String::from(colour), properties: properties, total_properties: length }
    }

    pub fn complete_set(&self) -> bool {
        return self.properties.len() as u8 == self.total_properties;
    }

    pub fn get_rent(&self, property_name: &str) -> f64 {
        if let Some(property) = self.properties.get(property_name) {
            if self.complete_set() && property.get_house_count() == 0 {
                return 2.0 * property.rent();
            } else {
                return property.rent();
            }
        } else {
            panic!("Could not find the property!");
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {

    }
}