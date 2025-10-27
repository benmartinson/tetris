use crate::prelude::*;

#[system]
#[read_component(Block)]
#[read_component(IsMoving)]
pub fn generate_block(ecs: &SubWorld, commands: &mut CommandBuffer) {
  let mut movers = <&Block>::query().filter(component::<IsMoving>());
  let mover_count = movers.iter(ecs).count();
  if mover_count == 0 {
    let mut rng = RandomNumberGenerator::new();
    let (block, color) = match rng.range(0, 5) {
      0 => spawn_square_block(),
      1 => spawn_snake_block(),
      2 => spawn_es_block(),
      3 => spawn_el_block(),
      4 => spawn_tee_block(),
      _ => unreachable!(), 
    };
    commands.push(
      (
        Square,
        Block {
          points: block
        },
        Render {
          color: ColorPair::new(color, BLACK),
          glyph: to_cp437('#')
        },
        IsMoving
      )
    );
  } 
  
}

pub fn spawn_square_block() -> (Vec<Point>, (u8, u8, u8)) {
  let point1 = Point::new(15, 2);
  let point2 = Point::new(16, 2);
  let point3 = Point::new(15, 3);
  let point4 = Point::new(16, 3);

  (vec![point1, point2, point3, point4], YELLOW)
}

pub fn spawn_snake_block() -> (Vec<Point>, (u8, u8, u8)) {
  let point1 = Point::new(15, 2);
  let point2 = Point::new(16, 2);
  let point3 = Point::new(17, 2);
  let point4 = Point::new(18, 2);

  (vec![point1, point2, point3, point4], RED)
}

pub fn spawn_el_block() -> (Vec<Point>, (u8, u8, u8)) {
  let point1 = Point::new(15, 2);
  let point2 = Point::new(15, 3);
  let point3 = Point::new(15, 4);
  let point4 = Point::new(16, 4);

  (vec![point1, point2, point3, point4], BLUE)
}

pub fn spawn_es_block() -> (Vec<Point>, (u8, u8, u8)) {
  let point1 = Point::new(16, 2);
  let point2 = Point::new(17, 2);
  let point3 = Point::new(16, 3);
  let point4 = Point::new(15, 3);

  (vec![point1, point2, point3, point4], GREEN)
}

pub fn spawn_tee_block() -> (Vec<Point>, (u8, u8, u8)) {
  let point1 = Point::new(15, 2);
  let point2 = Point::new(16, 2);
  let point3 = Point::new(17, 2);
  let point4 = Point::new(16, 3);

  (vec![point1, point2, point3, point4], PURPLE)
}
