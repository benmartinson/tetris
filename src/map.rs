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

  pub fn are_tiles_in_bounds(&self, points: Vec<Point>) -> bool {
    for point in points.iter() {
      if self.tiles[map_idx(point.x, point.y)] == TileType::Wall {
        return false
      }
    }
    true
  }
  
}