use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};

use crate::Result;

pub fn spawn() -> Result<()> {
    tokio::spawn(async {
        let addr = "127.0.0.1:10001";
        let listener = TcpListener::bind(addr)
            .await
            .expect("failed to bind socket");
        info!("realtime listening on: {}", addr);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    });

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();

    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
