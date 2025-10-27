use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn gravity(
  ecs: &mut SubWorld, 
  #[resource] frame_time: &mut f32,
  #[resource] map: &Map,
  commands: &mut CommandBuffer,
) {
  if *frame_time == 0.0 {
    let mut movers = <(Entity, &Block)>::query().filter(component::<IsMoving>());
    movers
      .iter(ecs)
      .for_each(|(entity, block)| {
        let delta = Point::new(0, 1);
        // let mut new_block = map.try_move(&block.points, delta) ;
        commands.push(((), WantsToMove { entity: *entity, block: block.clone(), delta}));
      });
  }
}
