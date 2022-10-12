use bevy::prelude::*;
use components::HelloPlugin;
pub mod components;
pub mod systems;
fn main() {
    App::new().add_plugin(HelloPlugin).run();
}
