mod map;
mod map_builder;
mod components;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::components::*;
    // pub use crate::spawner::*;
    pub use crate::systems::*;
    // pub use crate::turn_state::*;
}

use prelude::*;

enum GameMode {
    Menu,
}

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        resources.insert(map_builder.map);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(5, "Welcome to Bracket");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        self.resources.insert(ctx.key);
        // let current_state = self.resources.get::<TurnState>().unwrap().clone();
        self.input_systems.execute(
                &mut self.ecs,
                &mut self.resources
            );
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
        let context = BTermBuilder::new()
            .with_title("Dungeon Crawler")
            .with_fps_cap(30.0)
            .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
            .with_tile_dimensions(32, 32)
            .with_resource_path("resources/")
            .with_font("dungeonfont.png", 32, 32)
            .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
            .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
            .build()?;
    
    main_loop(context, State::new())
}
