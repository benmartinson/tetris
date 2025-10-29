use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
  Wall,
  Floor
}

pub struct Map {
  pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
  ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
  pub fn new() -> Self {
    Self {
      tiles: vec![TileType::Floor;NUM_TILES]
    }
  }

  pub fn are_tiles_in_bounds(&self, points: &[Point]) -> bool {
    points.iter().all(|point| {
        self.tiles[map_idx(point.x, point.y)] != TileType::Wall
    })
  }

  pub fn try_move(&self, points: &[Point], delta: Point) -> Vec<Point> {
    points.iter().map(|&pt| pt + delta).collect()
  }

  pub fn try_rotation(&self, points: &[Point]) -> Vec<Point> {
    let pivot = points[1];
    
    points.iter().map(|&pt| {
        let rel_x = pt.x - pivot.x;
        let rel_y = pt.y - pivot.y;

        let new_x =  rel_y;
        let new_y = -rel_x;

        Point::new(pivot.x + new_x, pivot.y + new_y)
    }).collect()
  }

  pub fn get_completed_lines(&self, lines_to_check: &[i32], points: &[Point]) -> Vec<i32> {
    lines_to_check
      .iter()
      .cloned()
      .filter(|&y| {
          (FLOOR_MIN_X..FLOOR_MAX_X)
              .all(|x| points.contains(&Point::new(x, y)))
      })
      .collect()
  }
}