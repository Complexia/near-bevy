use bevy::prelude::*;
pub mod components;
pub mod systems;
fn main() {
    App::new()
        .add_startup_system(crate::systems::add_people)
        .add_system(crate::systems::greet_people)
        .run();
}
