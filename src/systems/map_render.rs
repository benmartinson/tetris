use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map) {
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(0);

  for y in 0..SCREEN_HEIGHT {
    for x in 0..SCREEN_WIDTH {
      let pt = Point::new(x, y);
      let idx = map_idx(x, y);
      let glyph = match map.tiles[idx] {
        TileType::Floor => {
          to_cp437('.')
        },
        TileType::Wall => {
          to_cp437('#')
        }
      };

      draw_batch.set(
        pt,
        ColorPair::new(
          WHITE, BLACK
        ),
        glyph
      );
    }
  }
  draw_batch.submit(0).expect("Batch Error");
}