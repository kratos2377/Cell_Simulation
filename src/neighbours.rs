use bevy::math::{IVec3};

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NeighbourMethod {
    Moore,
    VonNeuman,
}

impl NeighbourMethod {
    pub fn get_neighbour_iter(&self) -> &'static [IVec3] {
        match self {
            NeighbourMethod::Moore => &MOORE_NEIGHBOURS[..],
            NeighbourMethod::VonNeuman => &VONNEUMAN_NEIGHBOURS[..],
        }
    }
}

pub static VONNEUMAN_NEIGHBOURS: [IVec3; 6] = [
    IVec3::from_array([1, 0, 0]),
    IVec3::from_array([-1, 0, 0]),
    IVec3::from_array([0, 1, 0]),
    IVec3::from_array([0, -1, 0]),
    IVec3::from_array([0, 0, -1]),
    IVec3::from_array([0, 0, 1]),
];

pub static MOORE_NEIGHBOURS: [IVec3; 26] = [
    IVec3::from_array([-1, -1, -1]),
    IVec3::from_array([0, -1, -1]),
    IVec3::from_array([1, -1, -1]),
    IVec3::from_array([-1, 0, -1]),
    IVec3::from_array([0, 0, -1]),
    IVec3::from_array([1, 0, -1]),
    IVec3::from_array([-1, 1, -1]),
    IVec3::from_array([0, 1, -1]),
    IVec3::from_array([1, 1, -1]),
    IVec3::from_array([-1, -1, 0]),
    IVec3::from_array([0, -1, 0]),
    IVec3::from_array([1, -1, 0]),
    IVec3::from_array([-1, 0, 0]),
    IVec3::from_array([1, 0, 0]),
    IVec3::from_array([-1, 1, 0]),
    IVec3::from_array([0, 1, 0]),
    IVec3::from_array([1, 1, 0]),
    IVec3::from_array([-1, -1, 1]),
    IVec3::from_array([0, -1, 1]),
    IVec3::from_array([1, -1, 1]),
    IVec3::from_array([-1, 0, 1]),
    IVec3::from_array([0, 0, 1]),
    IVec3::from_array([1, 0, 1]),
    IVec3::from_array([-1, 1, 1]),
    IVec3::from_array([0, 1, 1]),
    IVec3::from_array([1, 1, 1]),
];
