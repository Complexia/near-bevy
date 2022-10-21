use bevy::prelude::{App, Plugin};

pub struct NearPlugin;

impl Plugin for NearPlugin {
    fn build(&self, app: &mut App) {}

    fn name(&self) -> &str {
        "near-bevy"
    }
}
