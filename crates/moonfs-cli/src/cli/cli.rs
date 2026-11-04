use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "MoonFS is a Distributed File System")]
pub struct CLI {
    // --p for short, --path for long
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    #[arg(short, long)]
    pub server: String,

    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,

    #[arg(long, default_value = "8000")]
    pub port: u16,
}
