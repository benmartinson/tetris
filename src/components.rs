pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
  pub color: ColorPair,
  pub glyph: FontCharType
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
  pub points: Vec<Point>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsMoving;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square;

#[derive(Clone, Debug, PartialEq)]
pub struct WantsToMove {
  pub entity: Entity,
  pub block: Block
}
