use crate::prelude::*;
// mod player_input;
mod map_render;
mod entity_render;
mod movement;
mod generate_block;

pub fn build_input_scheduler() -> Schedule {
  Schedule::builder()
    // .add_system(player_input::player_input_system())
    // .add_system(movement::movement_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(generate_block::generate_block_system())
    .add_system(entity_render::entity_render_system())
    .build()
}
