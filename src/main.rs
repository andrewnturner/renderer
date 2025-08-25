mod cli;
mod drawing;
mod drawing_target;
mod model;
mod render;
mod run_interactive;
mod run_static;

use clap::Parser;

use crate::cli::RendererArgs;
use crate::run_interactive::run_interactive;
use crate::run_static::run_static;

fn main() {
    let args = RendererArgs::parse();

    if args.interactive {
        run_interactive(args);
    } else {
        run_static(args);
    }
}
