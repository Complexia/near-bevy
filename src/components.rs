use bevy::prelude::{App, Component, Plugin};

use crate::systems::{add_people, greet_people};

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people).add_system(greet_people);
    }
}
