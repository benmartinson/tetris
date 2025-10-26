use crate::prelude::*;

#[system(for_each)]
#[read_component(Block)]
pub fn movement(
  entity: &Entity,
  want_move: &WantsToMove,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] map: &Map
) {
  if map.are_tiles_in_bounds(want_move.block.points.clone()) {
    commands.add_component(want_move.entity, want_move.block.clone());
  }
  commands.remove(*entity);
}