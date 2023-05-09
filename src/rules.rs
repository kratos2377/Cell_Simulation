use std::ops::RangeInclusive;

use crate::{neighbours::NeighbourMethod};

#[derive(Clone , Copy , PartialEq)]
pub struct Value (
    [bool;27]
);


#[derive(Clone, PartialEq , Copy )]
pub struct Rule {
    pub survival_rule: Value,
    pub birth_rule: Value,
    pub states: u8,
    pub neighbour_method: NeighbourMethod,
}


impl Value {
    pub fn new(indices: &[u8]) -> Self {
        let mut result = Value([false;27]);
        for index in indices {
            result.0[*index as usize] = true;
        }

        result
    }

    pub fn from_range(indices: RangeInclusive<u8>) -> Self  {
        let mut result = Value([false;27]);
        for index in indices {
            result.0[index as usize] = true;
        }

        result
    }

    pub fn is_valid(&self, value: u8) -> bool {
        if (value as usize) < self.0.len() {
            *self.0.get(value as usize).unwrap()
        } else {
            false
        }
    }

    // Change the state of a value
    pub fn change_value(mut self, index: usize) -> Self {
        self.0[index] = !self.0[index];
        return self;
    }

    // Get a specified value
    pub fn get_value(self, index: usize) -> bool {
        self.0[index]
    }

    #[allow(dead_code)]
    pub fn in_range(&self, value: u8) -> bool {
        self.0[value as usize]
    }

    pub fn in_range_incorrect(&self, value: u8) -> bool {
        *self.0.get(value as usize).unwrap_or(&false)
    }
}
