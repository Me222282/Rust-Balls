mod ball;
mod physics;
mod program;
mod maths;
mod graphics;
mod state;
#[macro_use]
mod helpers;

use state::run;
use program::Program;

fn main() {
    pollster::block_on(run::<Program>());
}
