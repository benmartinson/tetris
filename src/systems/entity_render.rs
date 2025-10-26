use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(Render)]
pub fn entity_render(
  ecs: &SubWorld,
) {
  let mut draw_batch = DrawBatch::new();
  draw_batch.target(1);
  let mut renderables = <(&Render, &Block)>::query();
  renderables
    .iter(ecs)
    .for_each(|(render, block)| {
      for pt in block.points.iter() {
        println!("pt rendering x={}y={}", pt.x, pt.y);
        draw_batch.set(
          *pt,
          render.color,
          render.glyph
        );
      }
    }
  );
  draw_batch.submit(5000).expect("Batch error");
}
