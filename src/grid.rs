use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    pub size: GridSize,
    pub cell_data: Cell,
    pub data: Vec<Vec<i32>>,
}

pub struct GridSize {
    pub x: i32,
    pub y: i32,
}

pub struct Cell {
    pub size: Vec2<>,
}
