use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_egui::{EguiPlugin};
use rotating_camera::{RotatingCamera, RotatingCameraPlugin};


fn main() {
   let mut task_pool_settings = DefaultTaskPoolOptions::default();
   task_pool_settings.async_compute.percent = 1.0 as f32;
   task_pool_settings.compute.percent = 0.0 as f32;
   task_pool_settings.io.percent = 0.0 as f32;

   App::new()
       .insert_resource(task_pool_settings)
       .add_plugins(DefaultPlugins)
       .add_plugin(EguiPlugin)
       .insert_resource(ClearColor(Color::rgb(0.65f32, 0.9f32, 0.96f32)))
       .add_event::<CellStatesChangedEvent>()
       .add_plugin(RotatingCameraPlugin)
       .add_startup_system(setup)
       .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sims: ResMut<cells::Sims>,
) {

}
