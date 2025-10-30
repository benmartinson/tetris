use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsSettling)]
pub fn line_removal(
  ecs: &mut SubWorld, 
  #[resource] map: &Map,
  commands: &mut CommandBuffer,
) {
    let mut blocks = <(Entity, &Block)>::query();
    let mut settlingBlocks = <(Entity, &Block)>::query().filter(component::<IsSettling>());
    let mut all_points: Vec<Point> = vec![];
    let mut lines_to_check: Vec<i32> = vec![];

    settlingBlocks.iter(ecs).for_each(|(entity, block)| {
      block.points.iter().for_each(|&pt| {
        if !lines_to_check.contains(&pt.y) {
          lines_to_check.push(pt.y);
        }
      });
      commands.remove_component::<IsSettling>(*entity);
    });

    if lines_to_check.len() == 0 {
      return;
    }

    println!("lines to check = {}", lines_to_check.len());
    blocks 
      .iter(ecs)
      .for_each(|(_, block)| {
        block.points.iter().for_each(|&pt| {
          if lines_to_check.contains(&pt.y) {
            all_points.push(pt);
          }
        });
      });
    let mut lines_completed = map.get_completed_lines(&lines_to_check, &all_points);
    println!("lines completed = {}", lines_completed.len());
    if lines_completed.len() == 0 {
      return;
    }

    blocks 
      .iter(ecs)
      .for_each(|(entity, block)| { 
        let new_points: Vec<Point> = block.points
          .iter()
          .cloned()
          .filter(|pt| !lines_completed.contains(&pt.y))
          .collect();
        if new_points.len() == 0 {
          commands.remove(*entity);
        } else {
          commands.add_component(*entity, Block {points: new_points});
        }
      });

    lines_completed.iter().for_each(|completed_line| {
      commands.push(((), IsCollapsing { y: *completed_line }));
    })
}
