use crate::prelude::*;

#[system]
#[write_component(Block)]
#[read_component(IsCollapsing)]
pub fn line_collapse(
  ecs: &mut SubWorld, 
  #[resource] map: &Map,
  #[resource] frame_time: &f32,
  commands: &mut CommandBuffer,
) {
  if *frame_time == 0.0 {
    let mut lines: Vec<_> = <(Entity, &IsCollapsing)>::query()
      .iter(ecs)
      .collect();
    lines.sort_by(|(_, a), (_, b)| (b.y).cmp(&a.y));
    for (entity, _) in &lines {
      commands.remove(**entity);
    }

    let mut blocks = <(Entity, &Block)>::query();
    blocks
      .iter(ecs)
      .for_each(|(entity, block)| {
        // for each block, loop through the lines that isCollapsing
        // and move the points down accordingly
        let mut new_points: Vec<Point> = block.points.clone();
        for (_, collapsing) in &lines {
          for pt in new_points.iter_mut() {
            if collapsing.y >= pt.y {
              pt.y += 1
            }
          }
        }
        commands.add_component(*entity, Block{points: new_points})
      });

      // Problem is: we need to call add_component only once for each block, so we need a seperate new_blocks object to keep track 


    // for (entity, toCollapse) in lines {
    //   blocks.iter(ecs).for_each(|block| {
    //     let new_points = block.points.iter()
    //       .map(|| {
    //         if toCollapse.line >= pt.y {
    //           pt.y -= 1;
    //         }
    //         pt
    //       })
    //       .collect();

    //     commands.add_component(*entity, Block)
    //   });
    // }


  }
}
