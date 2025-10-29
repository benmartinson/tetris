use crate::prelude::*;

#[system(for_each)]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn rotation(
  message_entity: &Entity,
  want_rotation: &WantsToRotate,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] map: &Map
) {
  let mut all_blocks = <(Entity, &Block)>::query();
  let moving_entity = &want_rotation.entity;

  let new_points = map.try_rotation(&want_rotation.block.points);
  let is_move_in_boundes = map.are_tiles_in_bounds(&new_points);

  // Check collision with non-moving blocks
  let is_collision = all_blocks
    .iter(ecs)
    .filter(|(entity, _)| *entity != moving_entity)
    .filter(|(entity, _)| {
      ecs.entry_ref(**entity)
        .map(|entry| entry.get_component::<IsMoving>().is_err())
        .unwrap_or(true)
    })
    .any(|(_, block)| {
      block.points.iter().any(|pt| new_points.iter().any(|new_pt| new_pt == pt))
    });

  if is_move_in_boundes && !is_collision {
    if let Ok(entry) = ecs.entry_ref(*moving_entity) {
      if entry.get_component::<IsMoving>().is_ok() {
        commands.add_component(*moving_entity, Block{points: new_points});
      }
    }
  }
  commands.remove(*message_entity);
}