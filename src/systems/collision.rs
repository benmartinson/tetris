use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn collision(
  ecs: &mut SubWorld,
  #[resource] map: &Map,
  commands: &mut CommandBuffer
) {
  let mut blocks = <&Block>::query();
  let mut mover_query = <(Entity, &Block)>::query().filter(component::<IsMoving>());
  let movers: Vec<(&Entity, &Block)> = mover_query.iter(ecs).collect();
  movers.iter().for_each(|(entity, block)| {
    let delta = Point::new(0, 1);
    if !map.try_move(&block.points, delta) {
      commands.remove_component::<IsMoving>(**entity);
    }
  });

  // for block in blocks.iter(ecs) {

  // }
}
