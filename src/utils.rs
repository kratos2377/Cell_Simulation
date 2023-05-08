use bevy::{
    math::{ivec3, IVec3, Vec4},
    prelude::Color,
};
use std::ops::RangeInclusive;
use rand::Rng;


pub fn is_in_bounds(pos: IVec3, bounds: i32) -> bool {
    pos.x < bounds && pos.y < bounds && pos.z < bounds
}

pub fn wrap(pos: IVec3, bounds: i32) -> IVec3 {
    (pos+bounds)%bounds
}

pub fn dist_to_center(cell_Pos: IVec3, bounds: i32) -> f32 {
    let cell_pos = cell_pos - center(bounds);
    let max = bounds as f32 / 2.0;
    cell_pos.as_vec3().length() / max
}


pub fn make_some_noise<F: FnMut(IVec3)>(center: IVec3 , radius: i32 , amount: usize , mut f: F) {
    let mut rand = rand::thread_rng();
    (0..amount).for_each(|_| {
        f(center + ivec3(
            rand.gen_range(-radius..=radius),
            rand.gen_range(-radius..=radius),
            rand.gen_range(-radius..=radius),
        ));
    });
}

