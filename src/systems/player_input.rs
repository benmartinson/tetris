use crate::prelude::*;

#[system]
#[read_component(Block)]
pub fn player_input(
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] key: &Option<VirtualKeyCode>,
) {
  let mut movers = <(Entity, &Block)>::query().filter(component::<IsMoving>());

  if let Some(key) = key {
    let delta = match key {
      VirtualKeyCode::Left => Point::new(-1, 0),
      VirtualKeyCode::Right => Point::new(1, 0),
      _ => Point::new(0, 0),
    };
    movers
      .iter(ecs)
      .for_each(|(entity, block)| {
        let mut new_block = block.clone();
        new_block.points.iter_mut().for_each(|pt| *pt += delta);
        commands.push(((), WantsToMove { entity: *entity, block: new_block }));
      }
    );
  }
}