use anyhow::Result;
use tokio::{process::Command, task::JoinHandle};
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub platform_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            platform_address: "realtime.ruda.app".to_string(),
        }
    }
}

/// Runner handle can be used to talk to the runner as it's operating.
///
/// The stored `JoinHandle` can be used to await completion of the
/// runner task, including non-recoverable errors.
pub struct Handle {
    // TODO: allow communication with the runner locally using channels
    pub comms: (),

    pub join: JoinHandle<Result<(), anyhow::Error>>,
}

/// Spawns a new runner task and immediately returns a handle to it.
///
/// # Protocol
///
/// By default each runner calls back home to the dash instance at `ruda.app`.
/// The dashboard can be used to spawn and manage exiting runners.
pub fn spawn(config: Config, cancel: CancellationToken) -> Result<Handle> {
    let join = tokio::spawn(async move {
        // establish a duplex connection with the platform
        let client =
            tokio_tungstenite::connect_async(format!("{}/{}", config.platform_address, "")).await?;

        let mut cmd = Command::new("").spawn()?;

        tokio::select! {
            status = cmd.wait() => {
                let status = status?;
                println!("exited with status: {:?}", status.code());
            },
            _ = cancel.cancelled() => {},
        }

        Result::Ok(())
    });

    Ok(Handle { join, comms: () })
}
