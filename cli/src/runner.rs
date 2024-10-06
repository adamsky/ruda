use std::time::Duration;

use anyhow::Result;
use clap::{Arg, ArgMatches, Command};
use tokio_util::sync::CancellationToken;

use ruda::runner;

use crate::config::Config;

pub fn cmd() -> clap::Command {
    Command::new("runner")
        .about("Launch a new runner")
        .display_order(30)
}

pub async fn run(
    matches: &ArgMatches,
    config: Config,
    cancellation: CancellationToken,
) -> Result<()> {
    // let ms = runner::spawn(config.base.runner, cancellation)?;
    // ms.join.await?;

    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(2)).await;
            println!("printing");
        }
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(60*60)) => continue,
                _ = cancellation.cancelled() => break,
            }
        }
    });

    Ok(())
}