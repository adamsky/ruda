#![allow(warnings)]

#[macro_use]
extern crate serde_derive;

mod deploy;
mod login;
mod install;
mod runner;

mod config;
mod util;

use std::time::Duration;

use clap::{Arg, ArgMatches, Command};
use tokio_util::sync::CancellationToken;

use config::Config;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: Config = ruda::config::load()?;

    let cancel = CancellationToken::new();

    match cmd().get_matches().subcommand() {
        Some(("deploy", m)) => deploy::run(m, cancel.clone()).await?,
        Some(("runner", m)) => runner::run(m, config, cancel.clone()).await?,

        Some(("login", m)) => login::run(m, cancel.clone()).await?,

        Some(("install", m)) => install::install(m, cancel.clone()).await?,
        Some(("uninstall", m)) => install::uninstall(m, cancel.clone()).await?,
        _ => unimplemented!(),
    }

    // Wait for either ctrl_c signal or message from within server task(s)
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Initiating graceful shutdown...");
            cancel.cancel();
        },
        _ = cancel.cancelled() => {},
    }

    tokio::time::sleep(Duration::from_millis(300)).await;

    Ok(())
}

pub fn cmd() -> Command {
    Command::new("ruda")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(VERSION)
        .author(AUTHORS)
        .about(
            "Deploy applications blazingly fast.\n\
             Learn more at https://ruda.app",
        )
        .arg(
            Arg::new("verbosity")
                .long("verbosity")
                .short('v')
                .display_order(100)
                .value_name("level")
                .default_value("info")
                .value_parser(["trace", "debug", "info", "warn", "error", "none"])
                .global(true)
                .help("Set the verbosity of the log output"),
        )
        .subcommand(deploy::cmd())
        .subcommand(runner::cmd())
        .subcommand(login::cmd())
        .subcommands(install::cmds())
}
