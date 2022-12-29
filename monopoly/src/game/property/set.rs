#[path = "./property.rs"] mod property;

use std::{collections::HashMap, hash::Hash};
use property::Property;

use uuid::Uuid;

pub struct PropertySet {
    colour: String,
    properties: HashMap<Uuid, Property>
}

impl PropertySet {
    pub fn new(colour: &str) -> Self {
        Self { colour: String::from(colour), properties: HashMap::new() }
    }

    pub fn get_property_owner(&self, property_name: &str) -> Option<Uuid> {
        for (id, property: &Property) in self.properties.into_iter() {
            if property.get_name() == property_name {
                return property.get_owner();
            }
        }
        panic!("Failed to find the property!");
    }
}