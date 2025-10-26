use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn generate_block(ecs: &SubWorld, commands: &mut CommandBuffer) {
  let mut movers = <(&Block)>::query().filter(component::<IsMoving>());
  let mover_count = movers.iter(ecs).count();
  if mover_count == 0 {
    let point1 = Point::new(15, 3);
    let point2 = Point::new(16, 3);
    let point3 = Point::new(15, 4);
    let point4 = Point::new(16, 4);
    commands.push(
      (
        Square,
        Block {
          points: vec![point1, point2, point3, point4]
        },
        Render {
          color: ColorPair::new(WHITE, BLACK),
          glyph: to_cp437('@')
        },
        IsMoving
      )
    );
  } 
  
}

// pub fn spawn_square_block(ecs: &mut World) {

  
// }