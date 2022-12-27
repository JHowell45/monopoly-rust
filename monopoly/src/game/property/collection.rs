use std::collections::HashMap;

#[path = "./set.rs"] mod set;

use set::PropertySet;

#[derive(Debug)]
pub struct PropertyCollection {
    properties: HashMap<String, PropertySet>
}

impl PropertyCollection {
    pub fn new() -> Self {
        Self { properties: HashMap::new() }
    }

    pub fn get_set(&self, colour: &str) -> Option<&PropertySet> {
        return self.properties.get(colour);
    }

    pub fn get_mut_set(&mut self, colour: &str) -> Option<&mut PropertySet> {
        return self.properties.get_mut(colour);
    }

    pub fn add_set(&mut self, ) {

    }
}
