use bevy::{ prelude::*, window::PrimaryWindow };
use crate::grid::*;

impl Grid {
    fn new(size: GridSize) -> Self {
        let (x, y) = (size.x, size.y);
        Self {
            size,
            cell_data: Cell { size: Vec2::ZERO },
            data: vec![vec![0; x as usize]; y as usize],
        }
    }
    fn build(&mut self, window_query: Query<&Window, With<PrimaryWindow>>) {
        if let Ok(window) = window_query.single() {
            let window_width = window.width();
            let window_height = window.height();
            let (x, y) = (self.size.x, self.size.y);
            self.cell_data = Cell {
                size: Vec2::new(window_width / (x as f32), window_height / (y as f32)),
            };
        }
    }
    fn draw(&self, gizmos: &mut Gizmos) {
        let (cell_x, cell_y) = (self.cell_data.size.x, self.cell_data.size.y);
        for i in 0..self.size.x {
            for j in 0..self.size.y {
                gizmos.rect_2d(
                    Isometry2d::from_xy((i as f32) * cell_x, (j as f32) * cell_y),
                    Vec2::new(cell_x, cell_y),
                    Color::WHITE
                );
            }
        }
    }
}
