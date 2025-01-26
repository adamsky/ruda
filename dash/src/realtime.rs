use std::{collections::HashMap, sync::Arc};

use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use saasbase::Database;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, Mutex},
};

use ruda::runner::msg::Message;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{data, error::Error, Result};

#[derive(Clone, Debug)]
pub enum Request {
    Deploy { env: String },
}

#[derive(Clone, Debug)]
pub enum Response {
    Ok,
    NoMachineAssigned,
}

#[derive(Clone)]
pub struct Handle {
    /// NOTE: Request is a tuple of (AppId, Request). We always pass the
    /// application id alongside the request. From the application id we can
    /// later deduce the machine to which the request should be routed.
    pub exec: Executor<(Uuid, Request), std::result::Result<Response, String>>,
}

pub type MachineHandles =
    Arc<Mutex<HashMap<Uuid, Executor<Request, std::result::Result<Response, String>>>>>;

pub fn spawn(db: Database, cancel: CancellationToken) -> Result<Handle> {
    // requests channel for the returned handle
    let (mut handle_sender, handle_receiver) = mpsc::channel::<(
        (Uuid, Request),
        oneshot::Sender<std::result::Result<Response, String>>,
    )>(20);
    let mut handle_stream = tokio_stream::wrappers::ReceiverStream::new(handle_receiver);
    let mut handle_executor = Executor::new(handle_sender);

    tokio::spawn(async move {
        let addr = "127.0.0.1:10001";
        let listener = TcpListener::bind(addr)
            .await
            .expect("failed to bind socket");
        log::info!("realtime listening on: {}", addr);

        let mut machine_handles = Arc::new(Mutex::new(HashMap::<
            Uuid,
            Executor<Request, std::result::Result<Response, String>>,
        >::default()));

        loop {
            tokio::select! {
                Some(((app_id, req), s)) = handle_stream.next() => {
                    // requests from the handle stream are routed to the
                    // appropriate machine handle
                    log::debug!("realtime request: {req:?}, app_id {app_id}");

                    let app = db.get::<data::App>(app_id).unwrap();

                    let resp = if let Some(machine_id) = app.machine {
                        let machines = machine_handles.lock().await;
                        let handle = machines.get(&machine_id).unwrap().clone();
                        drop(machines);
                        handle.execute(req).await.unwrap()
                    } else {
                        Ok(Response::NoMachineAssigned)
                    };

                    s.send(resp).unwrap();
                }
                Ok((stream, _)) = listener.accept() => {
                    let db = db.clone();
                    println!("accepting new connection...");
                    let machines = machine_handles.clone();
                    tokio::spawn(async move {
                        let addr = stream
                            .peer_addr()
                            .expect("connected streams should have a peer address");
                        log::info!("Peer address: {}", addr);

                        let ws_stream = tokio_tungstenite::accept_async(stream)
                            .await
                            .expect("Error during the websocket handshake occurred");

                        log::info!("New WebSocket connection: {}", addr);

                        let (mut write, mut read) = ws_stream.split();

                        tokio::select!{
                            // handle requests coming from realtime
                            // handle requests coming on the websocket
                            Some(Ok(msg)) = read.next() => {
                                // TODO: how to
                                // handle message locally
                                let resp = handle_msg(msg.try_into()?, db.clone(), machines).await?;
                                // machines.lock().await.insert(Uuid::new_v4(), write);

                                // return the response to the caller
                                write.send(resp.try_into()?).await;
                            }
                        }

                        while let Some(Ok(msg)) = read.next().await {
                        }

                        log::warn!("loop ended");

                        Ok::<(), Error>(())
                    });
                }
                _ = cancel.cancelled() => {
                    log::trace!("`realtime` task shutting down...");
                    return;
                }
            }
        }
    });

    Ok(Handle {
        exec: handle_executor,
    })
}

async fn handle_msg(
    msg: Message,
    db: Database,
    machine_handles: MachineHandles,
) -> Result<Message> {
    match msg {
        Message::IntroductionRequest(code) => {
            log::info!("new introduction with code: {code}");
            // This can be a new runner calling in for the first time, or
            // an already known runner just reconnecting.

            // `machine` is effectively a synonym for `runner` here
            let machines = db.get_collection::<data::Machine>()?;

            if let Some(mut machine) = machines.into_iter().find(|m| m.secret == code) {
                machine.status = data::machine::Status::Connected;
                db.set(&machine)?;

                // machine_handles.lock().await.insert(machine.id, )

                return Ok(Message::IntroductionResponse(true));
            }

            Ok(Message::IntroductionResponse(false))
        }
        _ => unimplemented!(),
    }
}

#[derive(Clone)]
pub struct Executor<IN, OUT> {
    sender: mpsc::Sender<(IN, oneshot::Sender<OUT>)>,
}

impl<IN, OUT> Executor<IN, OUT> {
    pub fn new(sender: mpsc::Sender<(IN, oneshot::Sender<OUT>)>) -> Self {
        Self { sender }
    }

    pub async fn execute(&self, msg: IN) -> Result<OUT> {
        let (sender, receiver) = oneshot::channel::<OUT>();
        self.sender
            .send((msg, sender))
            .await
            .map_err(|e| "interface failed, receiver dropped: {e}");
        Ok(receiver
            .await
            .map_err(|e| Error::NetworkError(e.to_string()))?)
    }
}
