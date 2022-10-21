use bevy::prelude::*;
use near_plugin::NearPlugin;

pub mod components;
pub mod near_plugin;

pub mod workspaces;
fn main() {
    App::new().add_plugin(NearPlugin).run();
}
