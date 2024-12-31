use std::ffi::OsString;

use anyhow::{Error, Result};
use clap::ArgMatches;
use clap::{Arg, Command};
use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceLevel, ServiceManager, ServiceStartCtx, ServiceStopCtx,
    ServiceUninstallCtx,
};
use tokio_util::sync::CancellationToken;

pub const SERVICE_LABEL: &'static str = "org.ruda.runner";

pub fn cmds() -> Vec<Command> {
    vec![
        Command::new("install")
            .about("Install ruda runner as a system service")
            .arg(Arg::new("autostart"))
            .arg(
                Arg::new("code")
                    .long("code")
                    .help("Machine secret found on the ruda dashboard")
                    .num_args(1),
            ),
        Command::new("uninstall").about("Uninstall ruda runner as a system service"),
    ]
}

pub async fn install(matches: &ArgMatches, cancellation: CancellationToken) -> Result<()> {
    let code = matches.get_one::<String>("code").unwrap();

    let label: ServiceLabel = SERVICE_LABEL.parse()?;
    let mut manager = <dyn ServiceManager>::native()?;

    // Update our manager to work with user-level services
    manager
        .set_level(ServiceLevel::User)
        .expect("Service manager does not support user-level services");

    // Install our service using the underlying service management platform
    manager.install(ServiceInstallCtx {
        label: label.clone(),
        program: std::env::current_exe()?,
        args: vec![
            OsString::from("runner"),
            OsString::from("--code"),
            OsString::from(code),
        ],
        contents: None, // Optional String for system-specific service content.
        username: None, // Optional String for alternative user to run service.
        working_directory: None, // Optional String for the working directory for the service process.
        environment: None, // Optional list of environment variables to supply the service process.
        autostart: true,   // Specify whether the service should automatically start upon OS reboot.
    })?;

    manager.start(ServiceStartCtx {
        label: label.clone(),
    })?;

    cancellation.cancel();

    Ok(())
}

pub async fn uninstall(matches: &ArgMatches, cancellation: CancellationToken) -> Result<()> {
    let label: ServiceLabel = SERVICE_LABEL.parse()?;
    let mut manager = <dyn ServiceManager>::native()?;

    manager
        .set_level(ServiceLevel::User)
        .expect("Service manager does not support user-level services");

    manager.stop(ServiceStopCtx {
        label: label.clone(),
    })?;

    manager.uninstall(ServiceUninstallCtx {
        label: label.clone(),
    })?;

    cancellation.cancel();

    Ok(())
}
