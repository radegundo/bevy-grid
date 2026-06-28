mod grid;
mod systems;

use bevy::prelude::*;

pub use grid::*;

pub mod prelude {
    pub use super::grid::*;
    pub use super::grid::Cell;
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {}
}
