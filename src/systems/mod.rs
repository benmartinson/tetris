use crate::prelude::*;
mod player_input;
mod map_render;
mod entity_render;
mod movement;
mod generate_block;
mod gravity;
mod rotation;
mod line_removal;
mod line_collapse;

pub fn build_input_scheduler() -> Schedule {
  Schedule::builder()
    .add_system(gravity::gravity_system())
    .flush()
    .add_system(line_collapse::line_collapse_system())
    .flush()
    .add_system(generate_block::generate_block_system())
    .flush()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(rotation::rotation_system())
    .flush()
    .add_system(movement::movement_system())
    .flush()
    .add_system(line_removal::line_removal_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .build()
}
