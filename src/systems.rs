use bevy::{ math::VectorSpace, platform::cell, prelude::*, window::PrimaryWindow };
use crate::grid::*;

impl Grid {
    pub fn new(size: GridSize) -> Self {
        let (x, y) = (size.x, size.y);
        Self {
            size,
            cell_data: Cell { size: Vec2::ZERO },
            data: vec![vec![0; x as usize]; y as usize],
            pos: Vec2::ZERO,
        }
    }

    pub fn build(&mut self, window_size: Vec2<>) {
        let (x, y) = (self.size.x as f32, self.size.y as f32);

        let cell_x = window_size.x / x;
        let cell_y = window_size.y / y;

        self.cell_data = Cell {
            size: Vec2::new(cell_x, cell_y),
        };

        self.pos = Vec2::new(
            -window_size.x / 2.0 + cell_x / 2.0,
            -window_size.y / 2.0 + cell_y / 2.0
        );
    }

    pub fn draw(&self, gizmos: &mut Gizmos) {
        let (cell_x, cell_y) = (self.cell_data.size.x, self.cell_data.size.y);
        for i in 0..self.size.x {
            for j in 0..self.size.y {
                gizmos.rect_2d(
                    Isometry2d::from_xy(
                        self.pos.x + cell_x * (i as f32),
                        self.pos.y + cell_y * (j as f32)
                    ),
                    Vec2::new(cell_x, cell_y),
                    Color::WHITE
                );
            }
        }
    }
    pub fn update_grid(&mut self, window_size: Vec2<>) {
        let (x, y) = (self.size.x as f32, self.size.y as f32);
        let cell_x = window_size.x / x;
        let cell_y = window_size.y / y;
        self.pos = Vec2::new(
            -window_size.x / 2.0 + cell_x / 2.0,
            -window_size.y / 2.0 + cell_y / 2.0
        );
    }
    pub fn get_grid_coords(&self, world_coords: Vec3<>) -> Option<Vec3<>> {
        let pos = Vec3::new(
            (
                ((world_coords.x as f32) - (self.pos.x - self.cell_data.size.x / 2.0)) /
                self.cell_data.size.x
            ).floor(),
            (
                ((world_coords.y as f32) - (self.pos.y - self.cell_data.size.y / 2.0)) /
                self.cell_data.size.y
            ).floor(),
            0.0
        );
        if
            pos.x > (self.size.x as f32) - 1.0 ||
            pos.x < 0.0 ||
            pos.y > (self.size.y as f32) - 1.0 ||
            pos.y < 0.0
        {
            return None;
        } else {
            Some(pos)
        }
    }
}
