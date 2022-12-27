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

    pub fn number_of_properties(&self) -> usize {
        return self.properties.len();
    }
}
