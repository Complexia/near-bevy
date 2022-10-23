use bevy::{
    prelude::{App, Bundle, Commands, Component, Plugin, Transform},
    tasks::{AsyncComputeTaskPool, Task},
};
use near_workspaces::Worker;

use crate::workspaces::NearInterface;

pub struct NearPlugin;

impl Plugin for NearPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_task);
    }

    fn name(&self) -> &str {
        "near-bevy"
    }
}

#[derive(Component)]
struct ComputeTransform(Task<Transform>);

#[derive(Bundle)]
struct BundleTransform {
    compute_transform: ComputeTransform,
}

fn spawn_task() {
    // Spawn new task on the AsyncComputeTaskPool
    let thread_pool = AsyncComputeTaskPool::get();
    let task = thread_pool.spawn(async move {
        NearInterface::initialize_worker().await

        //Transform::from_xyz(x as f32, y as f32, z as f32)
    });
}
