use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the YAML file containing URL pairs
    #[arg(short, long)]
    pub config: String,

    /// Output directory for HTML diff files
    #[arg(short, long, default_value = ".")]
    pub output_dir: PathBuf,
}
