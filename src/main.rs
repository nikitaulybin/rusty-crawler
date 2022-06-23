mod camera;
mod components;
pub mod lib;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use indexmap::IndexMap;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 3;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 3;
    pub const MAP_CONSOLE: usize = 0;
    pub const PLAYER_CONSOLE: usize = 1;
    pub const HUD_CONSOLE: usize = 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::lib::fixtures::map_prefabs::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    enemy_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        let player_start = map_builder.player_start;
        spawn_player(&mut ecs, player_start);
        spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        map_builder
            .entity_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, *pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            enemy_systems: build_enemy_scheduler(),
        }
    }

    fn reset_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        let player_start = map_builder.player_start;
        spawn_player(&mut self.ecs, player_start);
        spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        map_builder
            .entity_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_CONSOLE);
        ctx.print_color_centered(2, RED, BLACK, "You Died");
        ctx.print_color_centered(4, RED, BLACK, "Press P to start over");

        if let Some(VirtualKeyCode::P) = ctx.key {
            self.reset_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(HUD_CONSOLE);
        ctx.print_color_centered(2, YELLOW, BLACK, "You Won!");
        ctx.print_color_centered(4, YELLOW, BLACK, "Press P to start over");

        if let Some(VirtualKeyCode::P) = ctx.key {
            self.reset_state();
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(MAP_CONSOLE);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(PLAYER_CONSOLE);
        ctx.cls();
        ctx.set_active_console(HUD_CONSOLE);
        ctx.cls();
        self.resources.insert(ctx.key);
        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::EnemyTurn => self
                .enemy_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx),
        };
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_title("Rusty Crawler")
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}
