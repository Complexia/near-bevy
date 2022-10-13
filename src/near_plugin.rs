use bevy::prelude::{App, Plugin};

use crate::components::NearInterface;

pub struct NearPlugin;

impl Plugin for NearPlugin {
    fn build(&self, app: &mut App) {
        //app.add_system(NearInterface::mint_nft);
    }
}
