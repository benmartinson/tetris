use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn gravity(
  ecs: &mut SubWorld, 
  #[resource] frame_time: &f32,
  #[resource] map: &Map,
  commands: &mut CommandBuffer,
) {
  if *frame_time == 0.0 {
    let mut movers = <(Entity, &Block)>::query().filter(component::<IsMoving>());
    movers
      .iter(ecs)
      .for_each(|(entity, block)| {
        let delta = Point::new(0, 1);
        commands.push(((), WantsToMove { entity: *entity, block: block.clone(), delta}));
      });
  }
}
