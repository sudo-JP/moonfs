use clap::Parser;
use moonfs_cli::cli::CLI;

fn main() {
    let _cli = CLI::parse();

    // TODO: extract the path from cli, throw error if path doesn't exist
}
