use bevy::{
    prelude::{Color, Plugin, Query, ResMut, Resource, IntoSystemConfig},
    tasks::AsyncComputeTaskPool,
};


use crate::{
    simulation::Sim,
    renderer::{CellRenderer, InstanceData, InstanceMaterialData},
    rules::Rule,
    utils,
};

use crate::simulation::settings::*;
use crate::color_method;

#[derive(Clone)]
pub struct Example {
    pub name: String,
    pub rule: Rule,
    pub color_method: color_method::ColorMethod,
    pub color1: Color,
    pub color2: Color,
}

#[derive(Resource)]
pub struct Sims {
   pub sims: Vec<(String, Box<dyn Sim>)>,
    pub active_sim: usize,
    pub bounds: i32,
    pub update_dt: std::time::Duration,
    pub renderer: Option<Box<CellRenderer>>, // rust...
    pub rule: Option<Rule>, // this is really quite dumb. maybe Cell would have been a good idea.
    pub color_method: color_method::ColorMethod,
    pub color1: Color,
    pub color2: Color,
    pub examples: Vec<Example>,
}


impl Sims {
    pub fn new() -> Sims {
        Sims {
            sims: vec![],
            active_sim: usize::MAX,
            bounds: 64,
            update_dt: std::time::Duration::from_secs(0),
            renderer: Some(Box::new(CellRenderer::new())),
            rule: None,
            color_method: color_method::ColorMethod::DistToCenter,
            color1: Color::YELLOW,
            color2: Color::RED,
            examples: vec![],
        }
    }

    pub fn add_sim(&mut self, name: String, sim: Box<dyn Sim>) {
        self.sims.push((name, sim));
    }

    pub fn add_example(&mut self, example: Example) {
        self.examples.push(example);
    }

    pub fn set_sim(&mut self, index: usize) {
        if self.active_sim < self.sims.len() {
            self.sims[self.active_sim].1.reset();
        }

        let rule = self.rule.take().unwrap();
        self.active_sim = index;
        self.bounds = self.sims[index].1.set_bounds(self.bounds);
        self.sims[index].1.spawn_noise(&rule);
        self.renderer.as_mut().unwrap().set_bounds(self.bounds);
        self.rule = Some(rule);
    }

    pub fn set_example(&mut self, index: usize) {
        let example = self.examples[index].clone();
        let rule = example.rule;
        self.color_method = example.color_method;
        self.color1 = example.color1;
        self.color2 = example.color2;

        if self.active_sim < self.sims.len() {
            let sim = &mut self.sims[self.active_sim].1;
            sim.reset();
            sim.spawn_noise(&rule);
        }
        self.rule = Some(rule);
    }
}

pub fn update(
    mut this: ResMut<Sims>,
    mut query: Query<&mut InstanceMaterialData>,
) {
    if this.active_sim > this.sims.len() {
        this.set_sim(0);
    }

    let mut bounds = this.bounds;
    let mut active_sim = this.active_sim;
    let rule = this.rule.take().unwrap();
    let mut renderer = this.renderer.take().unwrap();

    let sim = &mut this.sims[active_sim].1;

    let t0 = std::time::Instant::now();
    sim.update(&rule, AsyncComputeTaskPool::get());
    let update_dt = t0.elapsed();
    sim.render(&mut renderer);

    let instance_data = &mut query.iter_mut().next().unwrap().0;
    instance_data.truncate(0);
    for index in 0..renderer.cell_count() {
        let value = renderer.values[index];
        let neighbors = renderer.neighbors[index];

        if value != 0 {
            let pos = utils::index_to_pos(index, bounds);
            instance_data.push(InstanceData {
                position: (pos - utils::get_center(bounds as i32)).as_vec3(),
                scale: 1.0,
                color: this
                    .color_method
                    .set_colour(
                        this.color1,
                        this.color2,
                        value,
                        rule.states,
                        neighbors,
                        utils::dist_to_center(pos, bounds),
                        index,
                        renderer.cell_count(),
                    )
                    .into(),
            });
        }
    }
    this.bounds = bounds;
    this.active_sim = active_sim;
    this.update_dt = update_dt;
    this.renderer = Some(renderer);
    this.rule = Some(rule);
}



pub struct SimsPlugin;
impl Plugin for SimsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .insert_resource(Sims::new())
        .add_system(settings_ui.before(update))
        .add_system(update);
    }
}
