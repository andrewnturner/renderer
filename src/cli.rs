use clap::{command, Parser, ValueHint};

use crate::render::RenderMode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct RendererArgs {
    /// Path of OBJ file to render
    #[arg(short='m', long, value_hint = ValueHint::FilePath)]
    pub model: String,

    /// Path to render GIF file to
    #[arg(short, long, value_hint = ValueHint::FilePath, )]
    pub out: Option<String>,

    #[arg(short = 'W', long, default_value = "600")]
    pub width: u32,

    #[arg(short = 'H', long, default_value = "400")]
    pub height: u32,

    /// Launch in interactive mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Render mode
    #[arg(short = 'M', long, default_value = "solid")]
    pub mode: RenderMode,
}
