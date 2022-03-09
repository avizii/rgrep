use rgrep::*;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let config = GrepConfig::parse();
    config.match_with_default_strategy()?;

    Ok(())
}