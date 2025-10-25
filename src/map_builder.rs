use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
  pub map: Map,
  // pub current_blocks: Vec<Block>
}

impl MapBuilder {
  pub fn new(rng: &mut RandomNumberGenerator) -> Self {
    let mut mb = MapBuilder{
      map: Map::new(),
    };

    mb.fill(TileType::Wall);
    mb.build_rect();
    mb
  }

  fn build_rect(&mut self) {
    let rectangle = Rect::with_exact(
      10,
      2,
      SCREEN_WIDTH-10,
      SCREEN_HEIGHT-2,
    );

    rectangle.for_each(|pt| {
      let idx = map_idx(pt.x, pt.y);
      self.map.tiles[idx] = TileType::Floor;
    })
  }

  fn fill(&mut self, tile: TileType) {
    self.map.tiles.iter_mut().for_each(|t| *t = tile)
  }
}