#![allow(warnings)]

pub mod msg;

use futures_util::{SinkExt, StreamExt};
use tokio::{process::Command, task::JoinHandle};
// use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{api, Result};

use msg::Message;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub platform_address: String,

    pub code: Uuid,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            platform_address: "realtime.ruda.app".to_string(),

            code: Uuid::nil(),
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

    pub join: JoinHandle<Result<()>>,
}

/// Spawns a new runner task and immediately returns a handle to it.
///
/// # Protocol
///
/// By default each runner calls back home to the dash instance at `ruda.app`.
/// The dashboard can be used to spawn and manage exiting runners.
pub fn spawn(config: Config, cancel: CancellationToken) -> Result<Handle> {
    let join = tokio::spawn(async move {
        log::info!("connecting to ws://{}...", config.platform_address);

        // establish a duplex connection with the platform
        let (stream, _) = tokio_tungstenite::connect_async("ws://localhost:10001").await?;
        // let (stream, _) =
        //     tokio_tungstenite::connect_async(format!("ws://{}", config.platform_address)).await?;

        log::info!("connected ws");

        let (mut write, mut read) = stream.split();

        // Introduce ourselves with the secret code
        write
            .send(Message::IntroductionRequest(config.code).try_into()?)
            .await?;

        // let mut cmd = Command::new("").spawn()?;

        loop {
            tokio::select! {
                Some(Ok(msg)) = read.next() => {
                    println!("msg: {:?}", msg);
                    if let Some(resp) = handle_msg(msg.try_into()?).await? {
                        write.send(resp.try_into()?).await?;
                    }
                },
                // status = cmd.wait() => {
                //     let status = status?;
                //     println!("exited with status: {:?}", status.code());
                // },
                _ = cancel.cancelled() => break,
            }
        }

        Result::Ok(())
    });

    Ok(Handle { join, comms: () })
}

pub async fn handle_msg(msg: Message) -> Result<Option<Message>> {
    match msg {
        Message::StatusRequest => {
            let sys = sysinfo::System::new_all();
            let num_cpus = sys.cpus().len();
            Ok(Some(Message::StatusResponse(format!("cpus: {num_cpus}"))))
        }
        Message::IntroductionResponse(ok) => {
            log::info!("introduction ok: {ok}");
            Ok(None)
        }
        _ => unimplemented!(),
    }
}
