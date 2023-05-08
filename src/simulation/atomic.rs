use bevy::{
    math::{ivec3, IVec3},
    tasks::{TaskPool},
};

use futures_lite::future;

use crate::{
    renderer::{CellRenderer},
    rule::Rule,
    utils::{self},
};
use crate::simulation::Sim;
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

    pub fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        let radius = bounds_to_chunk_radius(new_bounds);
        let bounds = radius * CHUNK_SIZE;
        self.values    = Values::new(bounds*bounds*bounds);
        self.neighbors = Values::new(bounds*bounds*bounds);
        self.chunk_radius = radius;
        self.chunk_count  = radius*radius*radius;
        bounds as i32
    }

    pub fn bounds(&self) -> i32 {
        (self.chunk_radius * CHUNK_SIZE) as i32
    }

    pub fn total_cell_count(&self) -> usize {
        self.chunk_count * CHUNK_CELL_COUNT
    }

    pub fn center(&self) -> IVec3 {
        let center = self.bounds() / 2;
        ivec3(center, center, center)
    }

    pub fn cell_count(&self) -> usize {
        let mut result = 0;
        for index in 0..self.total_cell_count() {
            if !cell_is_dead(self.values.read(index)) {
                result += 1;
            }
        }
        result
    }


    fn update_neighbors(
        neighbors: &Values,
        index: usize, bounds: i32,
        rule: &Rule, inc: bool
    ) {
        let pos   = utils::index_to_pos(index, bounds);
        let local = pos % CHUNK_SIZE as i32;
        if chunk_is_border_pos(local, 1) {
            for dir in rule.neighbour_method.get_neighbour_iter() {
                let neighbor_pos = utils::wrap(pos + *dir, bounds);
                let index = utils::pos_to_index(neighbor_pos, bounds);

                let neighbors = neighbors.atomic(index);
                if inc {
                    neighbors.fetch_add(1, Ordering::Relaxed);
                }
                else {
                    neighbors.fetch_sub(1, Ordering::Relaxed);
                }
            }
        }
        else {
            for dir in rule.neighbour_method.get_neighbour_iter() {
                let neighbor_pos = pos + *dir;
                let index = utils::pos_to_index(neighbor_pos, bounds);

                let neighbors = neighbors.write(index);
                if inc {
                    *neighbors += 1;
                }
                else {
                    *neighbors -= 1;
                }
            }
        }
    }

    fn update_values(
        values: &Values, neighbors: &Values,
        chunk_index: usize, chunk_radius: usize, bounds: i32,
        rule: &Rule,
        spawns: &mut Vec<usize>, deaths: &mut Vec<usize>,
    ) {
        let chunk_pos = CHUNK_SIZE as i32 * utils::index_to_pos(chunk_index, chunk_radius as i32);
        for offset in 0..CHUNK_CELL_COUNT {
            let pos   = chunk_pos + chunk_offset_to_pos(offset);
            let index = utils::pos_to_index(pos, bounds);

            let value     = values.write(index);
            let neighbors = neighbors.read(index);

            if cell_is_dead(*value) {
                if rule.birth_rule.in_range(neighbors) {
                    *value = rule.states;
                    spawns.push(index);
                }
            }
            else {
                if *value < rule.states || !rule.survival_rule.in_range(neighbors) {
                    if *value == rule.states {
                        deaths.push(index);
                    }

                    *value -= 1;
                }
            }
        }
    }

    pub fn update(&mut self, rule: &Rule, tasks: &TaskPool) {
        // update values.
        let mut value_tasks = vec![];
        for chunk_index in 0..self.chunk_count {
            let values    = self.values.clone();
            let neighbors = self.neighbors.clone();
            let chunk_radius = self.chunk_radius;
            let bounds = self.bounds();

            let rule = rule.clone(); // shrug
            let mut chunk_spawns = vec![];
            let mut chunk_deaths = vec![];

            value_tasks.push(tasks.spawn(async move {
                Self::update_values(
                    &values, &neighbors,
                    chunk_index, chunk_radius, bounds,
                    &rule,
                    &mut chunk_spawns, &mut chunk_deaths);
                (chunk_spawns, chunk_deaths)
            }));
        }

        // collect spawns & deaths.
        let mut chunk_spawns = vec![];
        let mut chunk_deaths = vec![];
        for task in value_tasks {
            let (spawns, deaths) = future::block_on(task);
            chunk_spawns.push(spawns);
            chunk_deaths.push(deaths);
        }


        // update neighbors.
        let mut neighbor_tasks = vec![];
        for (spawns, deaths) in chunk_spawns.into_iter().zip(chunk_deaths) {
            let neighbors = self.neighbors.clone();
            let bounds = self.bounds();
            let rule = rule.clone(); // shrug

            neighbor_tasks.push(tasks.spawn(async move {
                for index in spawns.iter() {
                    Self::update_neighbors(
                        &neighbors,
                        *index, bounds,
                        &rule, true);
                }

                for index in deaths.iter() {
                    Self::update_neighbors(
                        &neighbors,
                        *index, bounds,
                        &rule, false);
                }
            }));
        }

        for task in neighbor_tasks {
            future::block_on(task);
        }
    }


    // TEMP: move to sims.
    #[allow(dead_code)]
    fn validate(&self, rule: &Rule) {
        for index in 0..self.total_cell_count() {
            let pos = utils::index_to_pos(index, self.bounds());

            let mut neighbors = 0;
            for dir in rule.neighbour_method.get_neighbour_iter() {
                let neighbor_pos = utils::wrap(pos + *dir, self.bounds());
                let index = utils::pos_to_index(neighbor_pos, self.bounds());

                let value = self.values.read(index);
                if value == rule.states {
                    neighbors += 1;
                }
            }

            assert_eq!(neighbors, self.neighbors.read(index));
        }
    }

    pub fn spawn_noise(&mut self, rule: &Rule) {
        let center = self.center();
        let bounds = self.bounds();

        utils::make_some_noise_default(center, |pos| {
            let index = utils::pos_to_index(utils::wrap(pos, bounds), self.bounds());
            let value = self.values.write(index);
            if cell_is_dead(*value) {
                *value = rule.states;
                Self::update_neighbors(
                    &self.neighbors,
                    index, self.bounds(),
                    rule, true);
            }
        });
    }
}


impl Sim for CellSimulation {
    fn update(&mut self, rule: &Rule, task_pool: &TaskPool) {
        self.update(rule, task_pool);
    }

    fn render(&self, renderer: &mut CellRenderer) {
        for index in 0..self.total_cell_count() {
            renderer.set(index,
                self.values.read(index),
                self.neighbors.read(index));
        }
    }

    fn spawn_noise(&mut self, rule: &Rule) {
        self.spawn_noise(rule);
    }

    fn cell_count(&self) -> usize {
        self.cell_count()
    }

    fn bounds(&self) -> i32 {
        self.bounds()
    }

    fn set_bounds(&mut self, new_bounds: i32) -> i32 {
        self.set_bounds(new_bounds)
    }
}