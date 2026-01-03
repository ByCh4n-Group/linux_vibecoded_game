mod assets;
mod combat;
mod defs;
mod discord_rpc;
mod game_state;
mod input_handler;
mod physics;
mod player;
mod scenes;
mod system;
mod texts;
mod world;

use crate::defs::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::game_state::GameState;
use tetra::ContextBuilder;

fn main() -> tetra::Result {
    ContextBuilder::new("Gorkitale", SCREEN_WIDTH, SCREEN_HEIGHT)
        .quit_on_escape(false)
        .build()?
        .run(GameState::new)
}
