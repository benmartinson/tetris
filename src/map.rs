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

  pub fn is_tile_open(&self, point: Point) -> bool {
    self.tiles[map_idx(point.x, point.y)] == TileType::Floor
  }

  // pub fn try_idx(&self, point: Point) -> Option<usize> {
  //   if !self.in_bounds(point) {
  //     None
  //   } else {
  //     Some(map_idx(point.x, point.y))
  //   }
  // }

  
}