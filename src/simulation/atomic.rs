use bevy::{
    math::{ivec3, IVec3},
    tasks::{TaskPool},
};

use futures_lite::future;

use crate::{
    cell_renderer::{CellRenderer},
    rule::Rule,
    utils::{self},
};

use std::sync::{atomic::{AtomicU8, Ordering}, Arc};
use std::cell::UnsafeCell;


const CHUNK_SIZE:       usize = 32;
const CHUNK_CELL_COUNT: usize = CHUNK_SIZE*CHUNK_SIZE*CHUNK_SIZE;

fn bounds_to_chunk_radius(bounds: i32) -> usize {
    (bounds as usize + CHUNK_SIZE - 1) / CHUNK_SIZE
}

fn chunk_offset_to_pos(offset: usize) -> IVec3 {
    utils::index_to_pos(offset, CHUNK_SIZE as i32)
}

fn chunk_is_border_pos(pos: IVec3, offset: i32) -> bool {
    pos.x - offset <= 0 || pos.x + offset >= CHUNK_SIZE as i32 - 1 ||
    pos.y - offset <= 0 || pos.y + offset >= CHUNK_SIZE as i32 - 1 ||
    pos.z - offset <= 0 || pos.z + offset >= CHUNK_SIZE as i32 - 1
}


#[derive(Clone)]
struct Values (Arc<Vec<UnsafeCell<AtomicU8>>>);

unsafe impl Sync for Values {}
unsafe impl Send for Values {}

impl Values {
    fn new(length: usize) -> Values {
        Values(Arc::new((0..length).map(|_| UnsafeCell::new(AtomicU8::new(0))).collect()))
    }

    fn read(&self, index: usize) -> u8 {
        unsafe { *(*self.0[index].get()).get_mut() }
    }

    fn write(&self, index: usize) -> &mut u8 {
        unsafe { (*self.0[index].get()).get_mut() }
    }

    fn atomic(&self,index: usize) -> &mut AtomicU8 {
        unsafe { &mut *self.0[index].get() }
    }
}


fn cell_is_dead(value: u8) -> bool {
    value == 0
}


pub struct CellSimulation {
    values:    Values,
    neighbors: Values,
    chunk_radius: usize,
    chunk_count:  usize,
}

impl CellSimulation {
    pub fn new() -> Self {
        CellSimulation {
            values:    Values::new(0),
            neighbors: Values::new(0),
            chunk_radius: 0,
            chunk_count: 0,
        }
    }
}