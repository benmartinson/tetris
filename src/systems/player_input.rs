use crate::prelude::*;

#[system]
#[read_component(Block)]
pub fn player_input(
  ecs: &mut SubWorld,
  commands: &mut CommandBuffer,
  #[resource] key: &Option<VirtualKeyCode>,
  #[resource] map: &Map
) {
  let mut movers = <(Entity, &Block)>::query().filter(component::<IsMoving>());

  if let Some(key) = key {
    if *key == VirtualKeyCode::Up || *key == VirtualKeyCode::Space {
      movers
        .iter(ecs)
        .for_each(|(entity, block)| {
          commands.push(((), WantsToRotate { entity: *entity, block: block.clone()  }));
        }
      ); 
      return;
    }
    let delta = match key {
      VirtualKeyCode::Left => Point::new(-1, 0),
      VirtualKeyCode::Right => Point::new(1, 0),
      VirtualKeyCode::Down => Point::new(0, 1),
      _ => Point::new(0, 0),
    };
    movers
      .iter(ecs)
      .for_each(|(entity, block)| {
        commands.push(((), WantsToMove { entity: *entity, block: block.clone(), delta }));
      }
    );
  }
}