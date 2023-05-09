use bevy::{prelude::*, render::view::{NoFrustumCulling, ComputedVisibility , Visibility}};
use bevy_egui::{ EguiPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin,};
use bevy::window::PresentMode;
use bevy_flycam::prelude::*;
use crate::color_method::*;
pub mod event;
mod renderer;
mod color_method;
mod neighbours;
mod rotating_camera;
mod rules;
mod utils;
use renderer::*;
use neighbours::NeighbourMethod;
use rules::*;

mod simulation;
use simulation::sims::Example;

fn main() {
   let mut task_pool_settings = TaskPoolOptions::default();
   task_pool_settings.async_compute.percent = 1.0 as f32;
   task_pool_settings.compute.percent = 0.0 as f32;
   task_pool_settings.io.percent = 0.0 as f32;

   App::new()
       .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Cell Simulation".into(),
            resolution: (1920., 1080.).into(),
            present_mode: PresentMode::AutoNoVsync,
            // WASM config
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(task_pool_settings)
       .add_plugin(EguiPlugin)
       .insert_resource(ClearColor(Color::rgb(0.0 as f32, 0.0 as f32, 0.0 as f32)))
       .add_plugin(NoCameraPlayerPlugin)
       .insert_resource(MovementSettings {
        sensitivity: 0.00015, 
        speed: 25.0,          
    })
    // Change key bindings
    .insert_resource(KeyBindings {
        move_ascend: KeyCode::LShift,
        move_descend: KeyCode::LControl,
        ..Default::default()
    })
       .add_plugin(FrameTimeDiagnosticsPlugin::default())
       .add_plugin(CellMaterialPlugin)
       .add_plugin(simulation::SimsPlugin)
       .add_startup_system(setup)
       .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sims: ResMut<simulation::Sims>,
) {
    sims.add_sim("Default Simulation Type".into(),
    Box::new(simulation::CellSimulation::new()));


    sims.add_example(Example {
        name: "Constructor".into(),
        rule: Rule {
            survival_rule: Value::new(&[2, 6, 9]),
            birth_rule: Value::new(&[4, 6, 8, 9, 10]),
            states: 10,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::DistToCenter,
        color1: Color::YELLOW,
        color2: Color::RED,
    });

    sims.add_example(Example {
        name: "VN pyramid".into(),
        rule: Rule {
            survival_rule: Value::from_range(0..=6),
            birth_rule: Value::new(&[1,3]),
            states: 2,
            neighbour_method: NeighbourMethod::VonNeuman,
        },
        color_method: ColorMethod::DistToCenter,
        color1: Color::GREEN,
        color2: Color::BLUE,
    });

    sims.add_example(Example {
        name: "Fancy Snancy".into(),
        rule: Rule {
            survival_rule: Value::new(&[0,1,2,3,7,8,9,11,13,18,21,22,24,26]),
            birth_rule: Value::new(&[4,13,17,20,21,22,23,24,26]),
            states: 4,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::RED,
        color2: Color::BLUE,
    });

    sims.add_example(Example {
        name: "Pretty Crystals".into(),
        rule: Rule {
            survival_rule: Value::new(&[5,6,7,8]),
            birth_rule: Value::new(&[6,7,9]),
            states: 10,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::DistToCenter,
        color1: Color::GREEN,
        color2: Color::BLUE,
    });

    sims.add_example(Example {
        name: "Swapping Structures".into(),
        rule: Rule {
            survival_rule: Value::new(&[3,6,9]),
            birth_rule: Value::new(&[4,8,10]),
            states: 20,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::RED,
        color2: Color::GREEN,
    });

    sims.add_example(Example {
        name: "Slowly Expanding Blob".into(),
        rule: Rule {
            survival_rule: Value::from_range(9..=26),
            birth_rule: Value::new(&[5,6,7,12,13,15]),
            states: 20,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::YELLOW,
        color2: Color::BLUE,
    });

    sims.add_example(Example {
        name: "4/4/5-Rule".into(),
        rule: Rule {
            survival_rule: Value::new(&[4]),
            birth_rule: Value::new(&[4]),
            states: 5,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::BLACK,
        color2: Color::RED,
    });

    sims.add_example(Example {
        name: "Growth and Decay".into(),
        rule: Rule {
            survival_rule: Value::new(&[4]),
            birth_rule: Value::new(&[3]),
            states: 20,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::BLACK,
        color2: Color::RED,
    });

    sims.add_example(Example {
        name: "Electric Smoke".into(),
        rule: Rule {
            survival_rule: Value::new(&[6,7]),
            birth_rule: Value::new(&[4,6,9,10,11]),
            states: 6,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::BLUE,
        color2: Color::RED,
    });

    sims.add_example(Example {
        name: "LARGE LINES".into(),
        rule: Rule {
            survival_rule: Value::new(&[5]),
            birth_rule: Value::new(&[4, 6, 9, 10, 11, 16, 17, 18, 19, 20, 21, 22, 23, 24]),
            states: 35,
            neighbour_method: NeighbourMethod::Moore,
        },
        color_method: ColorMethod::State,
        color1: Color::BLUE,
        color2: Color::RED,
    });


    sims.set_example(2);

    commands.spawn((
        meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        InstanceMaterialData(
            (1..=10)
                .flat_map(|x| (1..=100).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
                    scale: 1.0,
                    color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                })
                .collect(),
        ),
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));


    commands.spawn(Camera3dBundle {
            transform: Transform::from_xyz(50.0, 25.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCam);
}
