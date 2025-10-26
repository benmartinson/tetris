use crate::prelude::*;

#[system(for_each)]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn movement(
  entity: &Entity,
  want_move: &WantsToMove,
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] map: &Map
) {
  if map.are_tiles_in_bounds(&want_move.block.points) {
    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
      if entry.get_component::<IsMoving>().is_ok() {
        commands.add_component(want_move.entity, want_move.block.clone());
      }
    }
  }
  commands.remove(*entity);
}