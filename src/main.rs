use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = imgkit_scuti::Cli::parse();
    imgkit_scuti::run(cli)
}
